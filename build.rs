/***************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **************************************************************************************************/
//! Build Script to copy the required linker script from the dependent `ruspiro-boot` crate to the
//! current build folder
//!

use std::{env, fs, path::Path};

fn main() {
    // copy the linker script from the boot crate to the current directory
    // so it will be invoked by the linker
    match env::var_os("DEP_RUSPIRO_TEST_FRAMEWORK_LINKERSCRIPT") {
        Some(source) => {
            println!("found test framework dependency");
            let ld_source = source.to_str().unwrap().replace("\\", "/");
            let src_file = Path::new(&ld_source);
            let trg_file = format!(
                "{}/{}",
                env::current_dir().unwrap().display(),
                src_file.file_name().unwrap().to_str().unwrap()
            );
            println!("Copy linker script from {:?}, to {:?}", src_file, trg_file);
            fs::copy(src_file, trg_file).unwrap();
        },
        _ => ()
    }
    
}
