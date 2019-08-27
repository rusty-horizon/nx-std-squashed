// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_variables)]
#![deny(non_shorthand_field_patterns)]

pub struct Value<A> { pub value: A }

#[macro_export]
macro_rules! pat {
    ($a:pat) => {
        Value { value: $a }
    };
}

fn main() {
    let pat!(value) = Value { value: () };
}
