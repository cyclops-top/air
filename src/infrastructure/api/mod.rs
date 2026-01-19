//! API Infrastructure Module
//! 后续可在此实现针对 Android/iOS 客户端的 REST 或 gRPC 接口。

#[allow(dead_code)]
pub struct ApiServer {
    // 占位配置
}

impl ApiServer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub async fn start(&self) -> anyhow::Result<()> {
        // 后续实现 API 服务器启动逻辑
        Ok(())
    }
}