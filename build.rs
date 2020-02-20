use std::env::var;

fn main() {
    let lapack_link_type = env_or_default("CARGO_LAPACK_TYPE", "dylib");
    let lapack_link_name = env_or_default("CARGO_LAPACK", "lapack");
    let blas_link_type = env_or_default("CARGO_BLAS_TYPE", "dylib");
    let blas_link_name = env_or_default("CARGO_BLAS", "blas");
    let scs_link_type = env_or_default("CARGO_SCS_TYPE", "dylib");
    let scs_link_name = env_or_default("CARGO_SCS", "scsdir");

    println!(
        "cargo:rustc-link-lib={}={}",
        lapack_link_type, lapack_link_name
    );
    println!("cargo:rustc-link-lib={}={}", blas_link_type, blas_link_name);
    println!("cargo:rustc-link-lib={}={}", scs_link_type, scs_link_name);
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}
