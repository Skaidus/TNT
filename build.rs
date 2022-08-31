// use std::path::Path;


// // reference: https://doc.rust-lang.org/cargo/reference/build-scripts.html
// // Guide: https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/
    
//     //println!("cargo:rerun-if-changed=aks_theorem/aks_theorem.cpp");
// // build.rs

// use std::env;
// fn main() {
//     // env::set_var("CC", "g++");
//     // env::set_var("CFLAGS", "-L=ntl/lib/libntl.a");
//     cxx_build::bridge("src/main.rs")
//         .file("src/aks_theorem.cc")
//         .include("ntl/include")
//         //.include("gmp")
//         .flag("-L ")
//         .flag("ntl/lib/libntl.a")
//         .include("include")
//         //.include("ntl/lib")
//         //.flag("-L ")
//         //.flag("ntl/lib/libntl.a")
//         //.flag("./ntl/lib/libntl.a")
//         .flag_if_supported("-std=c++14")
//     .compile("aks_theorem");
//     // println!("cargo:rustc-link-search=native=ntl/lib");
//     // //println!("cargo:rustc-link-search=native=ntl/lib");
//     // //println!("cargo:rustc-link-lib=static=ntl");   
//     // println!("cargo:rerun-if-changed=src/main.rs");
//     // println!("cargo:rerun-if-changed=src/aks_theorem.cc");
//     // println!("cargo:rerun-if-changed=include/aks_theorem.h");
//     // println!("cargo:rustc-link-search=aks_theorem");
//     // println!("cargo:rustc-link-lib=aks_theorem");

// }

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/aks_theorem.h");
    println!("cargo:rerun-if-changed=src/aks_theorem.cc");
    let mut link_type = "static";
    println!("cargo:rerun-if-env-changed=STATIC_LIBS");
    println!("cargo:rerun-if-env-changed=NTL_LIB_PATH");
    //println!("cargo:rerun-if-env-changed=DELSUM_GF2X_LIB_PATH");
    println!("cargo:rerun-if-env-changed=GMP_LIB_PATH");
    println!("cargo:rerun-if-env-changed=NTL_INCLUDE");
    let mut build = cxx_build::bridge("src/polywrap.rs");
    for (key, value) in std::env::vars() {
        println!("{}", key);
        match key.as_str() {
            "DELSUM_STATIC_LIBS" => {
                if value == "1" {
                    link_type = "static";
                }
            },
            "DELSUM_NTL_LIB_PATH" | "DELSUM_GMP_LIB_PATH" => {
                println!("cargo:rustc-link-search=native={}", value);
            },
            "DELSUM_NTL_INCLUDE" => {
                build.include(value);
            },
            _ => continue,
        }
    }
    
    println!("cargo:rustc-link-search=native=output/lib");
    println!("cargo:rustc-link-lib={}=gmp", link_type);
    println!("cargo:rustc-link-lib={}=ntl", link_type);
    build.include("output/include");
    // gmp is required for thread safety apparently?
    
    build
        // needed for NTL
        .flag_if_supported("-fpermissive")
        .file("src/aks_theorem.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-unused-parameter");
    build.compile("aks_theorem");
}