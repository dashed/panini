#![feature(plugin, test)]
#![plugin(panini, enum_stream)]

extern crate gearley;
extern crate cfg;

extern crate test;

use test::Bencher;
use self::Test::*;

#[derive(Clone, Copy)]
enum Test {
    Number,
    Mul,
    Add,
    LParen,
    RParen,
}

#[bench]
fn bench_simple_arithmetic(b: &mut Bencher) {
    let mut parser = grammar! {
        start ::= start add product {};
        start ::= product {};
        product ::= product mul factor {};
        product ::= factor {};
        factor ::= number {};
        factor ::= lparen start rparen {};

        enum_stream {
            number ::= Number;
            mul ::= Mul;
            add ::= Add;
            lparen ::= LParen;
            rparen ::= RParen;
        }
    };

    let input = &[
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,

        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add,
        LParen,
        Number, Mul, Number, Add, Number,
        Mul, Number, Add, Number,
        RParen, Add, Number
    ];

    b.iter(|| {
        assert!(parser.recognize(input.iter().cloned()))
    })
}
