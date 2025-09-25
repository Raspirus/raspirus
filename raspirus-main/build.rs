#[rustfmt::skip]
fn main() {
    println!("cargo:rustc-env=MAIN_PKG_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=MAIN_PKG_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=MAIN_PKG_DESCRIPTION={}", env!("CARGO_PKG_DESCRIPTION"));
}
