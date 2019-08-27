// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-pass
#![allow(dead_code)]
#![allow(unused_variables)]

struct Index;

impl Index {
    fn new() -> Self { Index }
}

fn user() {
    let new = Index::new;

    fn inner() {
        let index = Index::new();
    }

    let index2 = new();
}

fn main() {}
