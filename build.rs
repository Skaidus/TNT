use std::path::Path;

fn main() {
    let library_path = Path::new("./ntl-11.5.1/include");
    cc::Build::new()
        .cpp(true)
        .file("aks_theorem/aks_theorem.cpp")
        .include(library_path)
        .compile("aks_theorem.a");
}