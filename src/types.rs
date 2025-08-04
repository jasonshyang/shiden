use std::pin::Pin;

use anyhow::Result;
use tokio::sync::oneshot;
use tokio_stream::Stream;

pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

#[async_trait::async_trait]
pub trait Collector<E>: Send + Sync {
    fn name(&self) -> &'static str;

    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>>;
}

#[async_trait::async_trait]
pub trait StateEngine<E, D>: Send + Sync {
    fn name(&self) -> &'static str;

    async fn sync_state(&mut self) -> Result<()>;

    fn process_event(&mut self, event: E) -> Result<()>;

    fn process_request(&self, request: OneShot<D>) -> Result<()>;

    fn on_shutdown(&mut self) -> Result<()>;
}

pub trait Strategy<D, I, A>: Send + Sync {
    type InputBuilder: InputBuilder<D, I> + Default;

    fn name(&self) -> &'static str;

    fn interval_ms(&self) -> u64;

    fn evaluate(&self, input: I) -> Vec<A>;
}

#[async_trait::async_trait]
pub trait Executor<A>: Send + Sync {
    fn name(&self) -> &'static str;

    async fn execute(&self, action: A) -> Result<()>;
}

pub trait InputBuilder<D, I>: Send + Sync {
    fn insert(&mut self, data: D);

    fn build(self) -> Result<I>;
}

#[derive(Debug)]
pub struct OneShot<D> {
    pub sender: oneshot::Sender<D>,
}

impl<D> OneShot<D> {
    pub fn new() -> (Self, oneshot::Receiver<D>) {
        let (sender, receiver) = oneshot::channel();
        (Self { sender }, receiver)
    }

    pub fn respond(self, data: D) -> Result<()> {
        self.sender
            .send(data)
            .map_err(|_| anyhow::anyhow!("Failed to send response"))?;

        Ok(())
    }
}
