use async_trait::async_trait;

pub struct ApiPayload {
    pub status: String,
    pub body: String,
}

pub struct GitApiResponse {
    pub url: String
}

#[async_trait]
pub trait GitApi {
    async fn open_pull_request(&self) -> Result<GitApiResponse, &'static str>;
    async fn call_api(&self) -> Result<ApiPayload, &'static str>;
}