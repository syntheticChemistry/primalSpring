//! Build script: exposes the compile-time TARGET triple via `env!("TARGET")`.

fn main() {
    println!(
        "cargo::rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_owned())
    );
}
