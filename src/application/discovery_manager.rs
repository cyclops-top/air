use crate::domain::traits::DiscoveryProvider;
use crate::domain::models::DiscoveryMsg;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub struct DiscoveryManager {
    provider: Arc<dyn DiscoveryProvider>,
}

impl DiscoveryManager {
    pub fn new(provider: Arc<dyn DiscoveryProvider>) -> Self {
        Self { provider }
    }

    pub fn register_service(&self, msg: &DiscoveryMsg) -> anyhow::Result<String> {
        self.provider.register_service(msg)
    }

    pub fn unregister_service(&self, fullname: &str) -> anyhow::Result<()> {
        self.provider.unregister_service(fullname)
    }

    pub async fn start_discovery(&self, tx: mpsc::Sender<DiscoveryMsg>, shutdown_rx: oneshot::Receiver<()>) -> anyhow::Result<()> {
        // Trait method doesn't take shutdown_rx, so we just start it.
        // The provider implementation should handle lifecycle internally or via channel drop.
        self.provider.start_discovery(tx)?;
        
        // Wait for shutdown signal
        let _ = shutdown_rx.await;
        Ok(())
    }
}