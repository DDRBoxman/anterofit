pub use mime_::Mime;

pub fn octet_stream() -> Mime {
    mime!(Application/OctetStream)
}

pub fn json() -> Mime {
    mime!(Application/Json)
}

pub fn form_urlencoded() -> Mime {
    mime!(Application/WwwFormUrlEncoded)
}

pub fn formdata(boundary: &str) -> Mime {
    mime!(Multipart/FormData; ("boundary")=(boundary))
}