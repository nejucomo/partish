use std::future::Future;

/// A [Handler] can process `Request`s asynchronously to produce [Self::Response]s.
pub trait Handler<Request> {
    /// The [Ok] response type for handling [Request]s
    type Ok;

    /// The [Err] response type for handling [Request]s
    type Error;

    /// Handle a [Request] asynchronously to produce a [Self::Response]
    fn handle(&mut self, req: Request) -> impl Future<Output = Result<Self::Ok, Self::Error>>;
}
