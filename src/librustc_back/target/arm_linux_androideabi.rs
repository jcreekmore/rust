// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
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
    let mut base = super::android_base::opts();
    base.features = "+v7,+vfp3,+d16".to_string();
    base.max_atomic_width = 64;

    Target {
        llvm_target: "arm-linux-androideabi".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        data_layout: "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        target_os: "android".to_string(),
        target_env: "".to_string(),
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
        let contents = include_str!("json/arm-linux-androideabi.json");
        let obj = json::from_str(&contents).unwrap();

        let target = Target::from_json(obj);
        assert_eq!(target, super::target());
    }
}
