#[cfg(not(target_arch = "wasm32"))]
mod native {
    use core::pin::Pin;
    use std::time::Duration;

    use futures_util::stream::Stream;

    pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

    pub async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use core::pin::Pin;
    use std::time::Duration;

    use futures_util::stream::Stream;
    use tokio_with_wasm::alias as tokio;

    pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;

    pub async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
pub use wasm32::*;
