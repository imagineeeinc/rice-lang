use read_input::prelude::*;
use std::fmt;
mod rice;
use rice::lexar;

fn main() {
    println!("Welcome Rice Lang");
    let mut done = false;
    let ctrldkey = false;
    while !done {
        let mut input: String = input().msg("> ").get();
        let mut inp = input;
        lexar(inp);
        println!("{}", input);
        if ctrldkey == true {
            done = true;
        }
    }
}