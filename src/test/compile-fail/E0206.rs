// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

type Foo = i32;

impl Copy for Foo { }
//~^ ERROR the trait `Copy` may not be implemented for this type
//~| NOTE type is not a structure or enumeration
//~| ERROR E0117

#[derive(Copy, Clone)]
struct Bar;

impl Copy for &'static Bar { }
//~^ ERROR the trait `Copy` may not be implemented for this type
//~| NOTE type is not a structure or enumeration

fn main() {
}
