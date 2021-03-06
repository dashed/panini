#![feature(plugin, plugin_registrar, rustc_private)]
#![plugin(quasi_macros)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rustc;
extern crate syntax;

extern crate aster;
extern crate quasi;

extern crate bit_matrix;
extern crate cfg;
extern crate cfg_regex;
extern crate gearley;

pub mod front;
pub mod middle;
pub mod back;

pub mod rs;

pub use front::lexer;

use std::error::Error;

use front::ast::Stmts;
use middle::error::TransformationError;
use back::GenResult;

pub fn codegen<'cx>(ecx: &'cx mut rs::ExtCtxt,
                sp: rs::Span,
                stmts: Stmts) -> Box<rs::MacResult + 'cx> {
    match middle::ir::IrMappedGrammar::transform_from_stmts(stmts) {
        Ok(ir) => {
            ir.report_warnings(ecx);
            if let Some(errors) = ir.get_errors() {
                for error in errors {
                    report_error(ecx, sp, error);
                }
                return rs::DummyResult::any(rs::DUMMY_SP);
            }
            let result = back::IrTranslator::new(ir.into()).generate().translate(ecx);
            // Log the generated code.
            let _ = env_logger::init();
            match result {
                GenResult::Parser(expr) => {
                    info!("{}", rs::pprust::expr_to_string(&*expr));
                    rs::MacEager::expr(expr)
                }
                GenResult::Lexer(stmts) => {
                    info!(" ========== BEGIN LEXER OUT");
                    let mut whole_str = String::new();
                    for stmt in &stmts {
                        whole_str.push_str(&rs::pprust::stmt_to_string(stmt)[..]);
                    }
                    info!("{}", whole_str);
                    info!(" ========== END LEXER OUT");
                    rs::MacEager::stmts(rs::SmallVector::many(stmts))
                }
            }
        },
        Err(error) => {
            report_error(ecx, sp, &error);
            rs::DummyResult::any(rs::DUMMY_SP)
        }
    }
}

fn report_error(ecx: &mut rs::ExtCtxt, sp: rs::Span, error: &TransformationError) {
    match error {
        &TransformationError::RecursiveType(ref types) => {
            for &(ref lhs, ref causes) in types {
                let mut diag = ecx.struct_span_err(lhs.span, error.description());
                let multispan = rs::MultiSpan::from_spans(causes.iter().map(|c| c.span).collect());
                let msg = if multispan.primary_spans().len() == 1 {
                    "this symbol has a recursive type:"
                } else {
                    "these symbols have recursive types:"
                };
                diag.span_note(multispan, msg);
                diag.emit();
            }
        }
        _ => {
            ecx.span_err(sp, error.description());
        }
    }
}
