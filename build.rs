#[cfg(all(
    feature = "http",
    not(any(
        feature = "rustls_backend",
        feature = "rustls_backend_no_provider",
        feature = "native_tls_backend"
    ))
))]
compile_error!(
    "You have the `http` feature enabled; either the `rustls_backend`, `rustls_backend_no_provider`, \
    or `native_tls_backend` feature must be enabled to let Serenity make requests over the network.\n\
    - `rustls_backend` uses Rustls, a pure Rust TLS-implemenation.\n\
    - `rustls_backend_no_provider` also uses Rustls, but does not register a crypto provider by default.\n\
    - `native_tls_backend` uses SChannel on Windows, Secure Transport on macOS, and OpenSSL on \
    other platforms.\n\
    If you are unsure, go with `rustls_backend`."
);

#[cfg(all(
    feature = "gateway",
    not(any(
        feature = "payload_compression",
        feature = "transport_compression_zlib",
        feature = "transport_compression_zstd"
    ))
))]
compile_error!(
    "You have the `gateway` feature enabled; at least one of the following features must be enabled: \
    `payload_compression`, `transport_compression_zlib`, `transport_compression_zstd`"
);

fn main() {
    println!("cargo:rustc-check-cfg=cfg(tokio_unstable, ignore_serenity_deprecated)");
}
