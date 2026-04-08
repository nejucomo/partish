use futures::{TryStream, TryStreamExt as _};

use crate::Handler;

/// A [StreamHandler] consumes its own stream items by dispatching them to itself as a [Handler]
pub trait StreamHandler:
    Sized
    + Unpin
    + TryStream
    + Handler<<Self as TryStream>::Ok, Ok = bool, Error = <Self as TryStream>::Error>
{
    /// Process all items from the stream, unless there is an error or the handler signals an early exit with `Ok(false)`
    async fn run(mut self) -> Result<(), <Self as TryStream>::Error> {
        while self.handle_one_item().await? {}
        Ok(())
    }

    /// Wait for the next item, then process it
    ///
    /// # Return
    ///
    /// - `true` if processing should continue.
    /// - `false` if the handler requests stopping or there are no more items
    async fn handle_one_item(&mut self) -> Result<bool, <Self as TryStream>::Error> {
        if let Some(v) = self.try_next().await? {
            self.handle(v).await
        } else {
            Ok(false)
        }
    }
}
