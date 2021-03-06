/*******************************************************************************
 * Copyright (c) 2018-2019 Aion foundation.
 *
 *     This file is part of the aion network project.
 *
 *     The aion network project is free software: you can redistribute it
 *     and/or modify it under the terms of the GNU General Public License
 *     as published by the Free Software Foundation, either version 3 of
 *     the License, or any later version.
 *
 *     The aion network project is distributed in the hope that it will
 *     be useful, but WITHOUT ANY WARRANTY; without even the implied
 *     warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *     See the GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with the aion network project source files.
 *     If not, see <https://www.gnu.org/licenses/>.
 *
 ******************************************************************************/

extern crate cmake;
extern crate target_info;

use target_info::Target;

fn main() {
    let dst = cmake::build("native/rust_solidity");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!(
        "cargo:rustc-link-search=native={}/build/deps/lib",
        dst.display()
    );
    if Target::os() == "macos" {
        println!("cargo:rustc-link-search=native=/usr/local/opt/boost@1.60/lib");
    };

    println!("cargo:rustc-link-lib=static=solc");
    println!("cargo:rustc-link-lib=static=jsoncpp");
    println!("cargo:rustc-link-lib=static=solidity");
    println!("cargo:rustc-link-lib=static=evmasm");
    println!("cargo:rustc-link-lib=static=devcore");
    println!("cargo:rustc-link-lib=static=jsoncpp");
    println!("cargo:rustc-link-lib=boost_filesystem");
    println!("cargo:rustc-link-lib=boost_program_options");
    println!("cargo:rustc-link-lib=boost_regex");
    println!("cargo:rustc-link-lib=boost_system");
    if Target::os() == "macos" {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
