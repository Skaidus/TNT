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
    let mut build = cxx_build::bridge("src/polywrap.rs");
    println!("cargo:rustc-link-search=native=output/lib");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=ntl");
    build.include("output/include");
    build
        // needed for NTL
        .flag_if_supported("-fpermissive")
        .file("src/aks_theorem.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-unused-parameter");
    build.compile("aks_theorem");
}