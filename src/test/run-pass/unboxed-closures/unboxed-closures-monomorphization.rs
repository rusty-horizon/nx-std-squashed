// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// Test that unboxed closures in contexts with free type parameters
// monomorphize correctly (issue #16791)

fn main(){
    fn bar<'a, T:Clone+'a> (t: T) -> Box<FnMut()->T + 'a> {
        Box::new(move || t.clone())
    }

    let mut f = bar(42_u32);
    assert_eq!(f(), 42);

    let mut f = bar("forty-two");
    assert_eq!(f(), "forty-two");

    let x = 42_u32;
    let mut f = bar(&x);
    assert_eq!(f(), &x);

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Foo(usize, &'static str);

    let x = Foo(42, "forty-two");
    let mut f = bar(x);
    assert_eq!(f(), x);
}
