/*
createContainer :: CreateContainerOptions -> IO ContainerId,
    startContainer :: ContainerId -> IO (),
    containerStatus :: ContainerId -> IO ContainerStatus,
    createVolume :: IO Volume,
    fetchLogs :: FetchLogsOptions -> IO ByteString,
    pullImage :: Image -> IO ()
 */

use std::fmt::Display;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub (crate) struct ContainerOptions {
    pub (crate) image: Image,
    pub (crate) script: String,
    pub (crate) volume: Volume
}

pub (crate) struct FetchLogOptions;

#[derive(Debug)]
pub (crate) struct Volume(String);

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub (crate) struct Image {
    name: String,
    tag: Option<String>
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}",self.name, self.tag.unwrap_or("latest".to_owned()))
    }
}

#[derive(Debug)]
pub (crate) struct ContainerId(String);


impl From<String> for ContainerId {

    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
pub (crate) trait Service {

    async fn create_container(&self, options: ContainerOptions) -> Result<ContainerId>;
    async fn container_status(&self) -> Result<ContainerStatus>;
    async fn create_volume(&self) -> Result<Volume>;
    async fn fetch_logs(&self, opts: FetchLogOptions) -> Result<String>;
    async fn pull_image(&self, img: Image) -> Result<()>;
}

#[derive(Debug)]
pub (crate) enum ContainerStatus {
    Running,
    Exited,
    Other(String)
}
