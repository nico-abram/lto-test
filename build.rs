fn main() {
    println!("cargo:rerun-if-changed=cbuild/src/test.c");

    println!("cargo:rustc-link-lib=cbuild/out/test");
}
