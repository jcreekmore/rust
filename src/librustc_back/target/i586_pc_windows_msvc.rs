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
    let mut base = super::i686_pc_windows_msvc::target();
    base.options.cpu = "pentium".to_string();
    base.options.is_builtin = true;
    base.llvm_target = "i586-pc-windows-msvc".to_string();
    return base
}

#[cfg(test)]
mod test {
    use serialize::json;
    use target::Target;

    #[test]
    fn parse_json() {
        let contents = include_str!("json/i586-pc-windows-msvc.json");
        let obj = json::from_str(&contents).unwrap();

        let target = Target::from_json(obj);
        assert_eq!(target, super::target());
    }
}
