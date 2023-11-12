fn main() {
    cc::Build::new()
        .file("src/zip/zip.c")
        .flag("-lz")
        .shared_flag(true)
        .static_flag(true)
        .compile("zip");
    println!("cargo:rustc-link-arg=-lz");
}
