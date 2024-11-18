// Copyright 2019 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

// Minimum version of the TSS 2.0 libraries that this crate can use.
// const MINIMUM_VERSION: &str = "2.3.3";

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn main() {
    if cfg!(not(feature = "docs")) {
        let tss2_esys = pkg_config::Config::new()
            .atleast_version(MINIMUM_VERSION)
            .probe("tss2-esys")
            .expect("Error with pkg-config finding tss2-esys.");
        let tss2_tctildr = pkg_config::Config::new()
            .atleast_version(MINIMUM_VERSION)
            .probe("tss2-tctildr")
            .expect("Error with pkg-config finding tss2-tctildr.");
        let tss2_mu = pkg_config::Config::new()
            .atleast_version(MINIMUM_VERSION)
            .probe("tss2-mu")
            .expect("Error with pkg-config finding tss2-mu.");

        // These three pkg-config files should contain only one include/lib path.
        let tss2_esys_include_path = tss2_esys.include_paths[0]
            .clone()
            .into_os_string()
            .into_string()
            .expect("Error converting OsString to String.");
        let tss2_tctildr_include_path = tss2_tctildr.include_paths[0]
            .clone()
            .into_os_string()
            .into_string()
            .expect("Error converting OsString to String.");
        let tss2_mu_include_path = tss2_mu.include_paths[0]
            .clone()
            .into_os_string()
            .into_string()
            .expect("Error converting OsString to String.");

        let bindings = bindgen::Builder::default()
            .clang_arg(format!("-I{}/tss2/", tss2_esys_include_path))
            .clang_arg(format!("-I{}/tss2/", tss2_tctildr_include_path))
            .clang_arg(format!("-I{}/tss2/", tss2_mu_include_path))
            .rustfmt_bindings(true)
            .header(format!("{}/tss2/tss2_esys.h", tss2_esys_include_path))
            .header(format!("{}/tss2/tss2_tctildr.h", tss2_tctildr_include_path))
            .header(format!("{}/tss2/tss2_mu.h", tss2_mu_include_path))
            .generate_comments(false)
            .derive_default(true)
            .generate()
            .expect("Unable to generate bindings to TSS2 ESYS APIs.");

        let out_path = PathBuf::from(
            env::var("OUT_DIR").expect("Error while getting the OUT_DIR environment variable."),
        );
        bindings
            .write_to_file(out_path.join("tss2_esys_bindings.rs"))
            .unwrap_or_else(|_| panic!("Couldn't write bindings to {:?}!", out_path));
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {
    println!("cargo:rustc-link-lib=tss2-esys");
    println!("cargo:rustc-link-lib=tss2-tctildr");
    println!("cargo:rustc-link-lib=tss2-mu");
}
