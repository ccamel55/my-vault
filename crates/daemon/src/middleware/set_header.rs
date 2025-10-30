use poem::EndpointExt;

pub struct SetDefaultHeader;

impl SetDefaultHeader {
    pub fn new() -> Self {
        Self {}
    }
}

impl<E: poem::Endpoint> poem::Middleware<E> for SetDefaultHeader {
    type Output = poem::middleware::SetHeaderEndpoint<E>;

    /// Add headers to response which disable MIME type sniffing and
    /// disable displaying of response.
    fn transform(&self, ep: E) -> Self::Output {
        ep.with(
            poem::middleware::SetHeader::new()
                .appending(poem::http::header::X_CONTENT_TYPE_OPTIONS, "nosniff")
                .appending(poem::http::header::X_FRAME_OPTIONS, "deny"),
        )
    }
}
