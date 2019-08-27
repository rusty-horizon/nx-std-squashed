// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]
// pretty-expanded FIXME #23616

extern {
    #[link_name = "malloc"]
    fn malloc1(len: i32) -> *const u8;
    #[link_name = "malloc"]
    fn malloc2(len: i32, foo: i32) -> *const u8;
}

pub fn main () {}
