use anyhow::Result;

use super::container::{Service, ContainerId, ContainerOptions, Volume, FetchLogOptions, Image};

pub (crate) struct Podman;

impl Service for Podman {
    fn create_container(&self, options: ContainerOptions) -> Result<ContainerId> {
        todo!()
    }

    fn container_status(&self) {
        todo!()
    }

    fn create_volume(&self) -> Result<Volume> {
        todo!()
    }

    fn fetch_logs(&self, opts: FetchLogOptions) -> Result<String> {
        todo!()
    }

    fn pull_image(&self, img: Image) -> Result<()> {
        todo!()
    }
}
