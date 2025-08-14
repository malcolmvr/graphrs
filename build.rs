fn main() {
    #[cfg(feature = "adjacency_matrix")]
    println!("cargo:rustc-link-lib=dylib=lapack");
}
