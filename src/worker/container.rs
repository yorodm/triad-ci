/*
createContainer :: CreateContainerOptions -> IO ContainerId,
    startContainer :: ContainerId -> IO (),
    containerStatus :: ContainerId -> IO ContainerStatus,
    createVolume :: IO Volume,
    fetchLogs :: FetchLogsOptions -> IO ByteString,
    pullImage :: Image -> IO ()
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub (crate) struct ContainerOptions;
pub (crate) struct FetchLogOptions;

#[derive(Debug)]
pub (crate) struct Volume(String);

#[derive(Debug)]
pub (crate) struct Image(String);

#[derive(Debug)]
pub (crate) struct ContainerId(String);

pub (crate) trait Service {

    fn create_container(&self, options: ContainerOptions) -> Result<ContainerId>;
    fn container_status(&self);
    fn create_volume(&self) -> Result<Volume>;
    fn fetch_logs(&self, opts: FetchLogOptions) -> Result<String>;
    fn pull_image(&self, img: Image) -> Result<()>;
}

#[derive(Debug)]
pub (crate) enum ContainerStatus {
    Running,
    Exited,
    Other(String)
}
