use actix_multipart_derive_impl::MultipartForm;
use actix_web::web::BytesMut;

#[derive(Debug, Clone, Default, MultipartForm)]
struct Form {
    name: String,
    #[multipart(max_size = 8096)]
    file: BytesMut,
}

fn main () {}
