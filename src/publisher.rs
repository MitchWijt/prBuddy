use async_trait::async_trait;

#[async_trait]
pub trait Publish {
    async fn publish(&self) -> Result<(), &'static str>;
}