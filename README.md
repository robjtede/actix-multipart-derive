# actix-multipart-derive

> WIP derive macro wrapping `actix-multipart` to make multipart forms easier to consume.

[![crates.io](https://img.shields.io/crates/v/actix-multipart-derive?label=latest)](https://crates.io/crates/actix-multipart-derive)
[![Documentation](https://docs.rs/actix-multipart-derive/badge.svg?version=0.1.0)](https://docs.rs/actix-multipart-derive/0.1.0)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/actix-multipart-derive)
[![Dependency Status](https://deps.rs/crate/actix-multipart-derive/0.1.0/status.svg)](https://deps.rs/crate/actix-multipart-derive/0.1.0)

## Goal

Consuming multipart forms should be expressive while maintaining it's stream-oriented
implementation under the hood.

```rust
use actix_multipart_derive::MultipartForm;
use actix_web::{post, web::BytesMut, App, HttpServer};

#[derive(Debug, Clone, Default, MultipartForm)]
struct Form {
    name: String,

    #[multipart(max_size = 1024)]
    file: BytesMut,
}

#[post("/")]
async fn index(form: Form) -> &'static str {
    println!("{:?}", &form);

    "Hello world!\r\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
        .await
}

```
