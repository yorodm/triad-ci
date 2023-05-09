use anyhow::Result;
use async_trait::async_trait;

use super::container::{
    ContainerId, ContainerOptions, ContainerStatus, FetchLogOptions, Image, Service, Volume,
};

pub(crate) struct Podman;

#[async_trait]
impl Service for Podman {
    async fn create_container(&self, options: ContainerOptions) -> Result<ContainerId> {
        todo!()
    }
    async fn container_status(&self) -> Result<ContainerStatus> {
        todo!()
    }
    async fn create_volume(&self) -> Result<Volume> {
        todo!()
    }
    async fn fetch_logs(&self, opts: FetchLogOptions) -> Result<String> {
        todo!()
    }
    async fn pull_image(&self, img: Image) -> Result<()> {
        todo!()
    }
}
