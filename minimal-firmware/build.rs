fn main() {
    // apply to all targets
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    // only applies to tests
    println!("cargo:rustc-link-arg-tests=-Tembedded-test.x");
}
