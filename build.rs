use std::path::Path;


// reference: https://doc.rust-lang.org/cargo/reference/build-scripts.html
// Guide: https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/
fn main() {
    let library_path = Path::new("./ntl-11.5.1/include");
    println!("cargo:rerun-if-changed=aks_theorem/aks_theorem.cpp");
    cc::Build::new()
        .cpp(true)
        .file("aks_theorem/aks_theorem.cpp")
        .include(library_path)
        .compile("libakstheorem.a");
}