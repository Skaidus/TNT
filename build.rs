fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/aks_theorem.h");
    println!("cargo:rerun-if-changed=src/number_theory/primality/aks/aks_theorem.cc");
    let mut build = cxx_build::bridge("src/number_theory/primality/aks/aks_theorem.rs");
    println!("cargo:rustc-link-search=native=output/lib");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=gf2x");
    println!("cargo:rustc-link-lib=static=ntl");
    build.include("output/include");
    build
        // needed for NTL
        .flag_if_supported("-fpermissive")
        .file("src/number_theory/primality/aks/aks_theorem.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-unused-parameter");
    build.compile("aks_theorem");
}