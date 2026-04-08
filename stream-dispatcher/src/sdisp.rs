use std::pin::{Pin, pin};

use futures::{StreamExt, TryStream, TryStreamExt as _};

use crate::Handler;

/// A [StreamDispatcher] which consumes stream `S` which produces `T` items and dispatches them to the [Handler] `H`
pub trait StreamHandler:
    Sized + TryStream + Handler<<Self as TryStream>::Ok, Ok = bool, Error = <Self as TryStream>::Error>
{
    /// Process all items from `S`, unless there is an error or `H` signals an early exit with `Ok(false)`
    async fn run(self) -> Result<(), <Self as TryStream>::Error> {
        let mut pself = pin!(self);
        while pself.as_mut().handle_one_item().await? {}
        Ok(())
    }

    /// Wait for the next item, then process it
    ///
    /// # Return
    ///
    /// - `true` if processing should continue.
    /// - `false` if the handler requests stopping or there are no more items
    async fn handle_one_item(self: Pin<&mut Self>) -> Result<bool, <Self as TryStream>::Error> {
        if let Some(v) = self.try_next().await? {
            self.handle(v).await
        } else {
            Ok(false)
        }
    }
}
