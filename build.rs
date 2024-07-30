// build.rs
//
// Copyright (c) 2023-2024 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use std::env;
use std::path::{Path, PathBuf};

use cmake::Config;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/sys");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=CTranslate2");
    println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
    if let Ok(library_path) = env::var("LIBRARY_PATH") {
        library_path
            .split(':')
            .filter(|v| !v.is_empty())
            .for_each(|v| {
                println!("cargo:rustc-link-search={}", v);
            });
    }

    println!("cargo::rustc-link-lib=dylib={}", "ctranslate2");

    cxx_build::bridges([
        "src/sys/types.rs",
        "src/sys/config.rs",
        "src/sys/translator.rs",
        "src/sys/generator.rs",
        "src/sys/storage_view.rs",
        "src/sys/whisper.rs",
    ])
    .file("src/sys/translator.cpp")
    .file("src/sys/generator.cpp")
    .file("src/sys/whisper.cpp")
    .include("CTranslate2/include")
    .std("c++17")
    .static_crt(cfg!(target_os = "windows"))
    .flag_if_supported("/EHsc")
    .compile("ct2rs");
}
