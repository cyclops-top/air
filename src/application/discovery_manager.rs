use crate::domain::models::DiscoveryMsg;
use crate::domain::traits::DiscoveryProvider;
use std::sync::Arc;

pub struct DiscoveryManager {
    provider: Arc<dyn DiscoveryProvider>,
}

impl DiscoveryManager {
    pub fn new(provider: Arc<dyn DiscoveryProvider>) -> Self {
        Self { provider }
    }

    pub fn register_service(&self, msg: &DiscoveryMsg) -> anyhow::Result<String> {
        self.provider.register(msg)
    }

    pub fn unregister_service(&self, fullname: &str) -> anyhow::Result<()> {
        self.provider.unregister(fullname)
    }

    pub async fn start_discovery(
        &self, 
        tx: tokio::sync::mpsc::Sender<DiscoveryMsg>, 
        shutdown_rx: tokio::sync::oneshot::Receiver<()>
    ) -> anyhow::Result<()> {
        self.provider.listen(tx, shutdown_rx).await
    }
}
