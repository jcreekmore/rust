// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::Target;

pub fn target() -> Target {
    let mut base = super::linux_base::opts();
    base.max_atomic_width = 128;
    Target {
        llvm_target: "aarch64-unknown-linux-gnu".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_env: "gnu".to_string(),
        data_layout: "e-m:e-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        target_os: "linux".to_string(),
        target_vendor: "unknown".to_string(),
        options: base,
    }
}

#[cfg(test)]
mod test {
    use serialize::json;
    use target::Target;

    #[test]
    fn parse_json() {
        let contents = include_str!("json/aarch64-unknown-linux-gnu.json");
        let obj = json::from_str(&contents).unwrap();

        let target = Target::from_json(obj);
        assert_eq!(target, super::target());
    }
}
