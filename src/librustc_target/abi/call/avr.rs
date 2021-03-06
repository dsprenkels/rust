// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use abi::call::{FnType, ArgType};

fn classify_ret_ty<Ty>(ret: &mut ArgType<Ty>) {
    if ret.layout.is_aggregate()  {
        ret.make_indirect();
    } else {
        ret.extend_integer_width_to(8); // Is 8 correct?
    }
}

fn classify_arg_ty<Ty>(arg: &mut ArgType<Ty>) {
    if arg.layout.is_aggregate() {
        arg.make_indirect();
    } else {
        arg.extend_integer_width_to(8);
    }
}

pub fn compute_abi_info<Ty>(fty: &mut FnType<Ty>) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() {
            continue;
        }

        classify_arg_ty(arg);
    }
}
