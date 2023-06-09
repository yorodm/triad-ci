/*
createContainer :: CreateContainerOptions -> IO ContainerId,
    startContainer :: ContainerId -> IO (),
    containerStatus :: ContainerId -> IO ContainerStatus,
    createVolume :: IO Volume,
    fetchLogs :: FetchLogsOptions -> IO ByteString,
    pullImage :: Image -> IO ()
 */

use async_trait::async_trait;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct ContainerOptions {
    pub(crate) image: Image,
    pub(crate) script: String,
    pub(crate) volume: Volume,
}

pub(crate) struct FetchLogOptions;

#[derive(Debug)]
pub(crate) struct Volume(String);

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Image {
    name: String,
    tag: Option<String>,
}

impl Image {
    pub(crate) fn name(&self) -> String {
        self.name
    }

    pub(crate) fn tag(&self) -> String {
        if let Some(t) = self.tag {
            t
        } else {
            "latest".to_owned()
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref tag) = self.tag {
            write!(f, "{}:{}", self.name, tag)
        } else {
            write!(f, "{}:latest", self.name)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ContainerId(String);

impl From<String> for ContainerId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for ContainerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) type Result<T> = std::result::Result<T, ContainerError>;

#[async_trait]
pub(crate) trait Service {
    async fn create_container(&self, options: ContainerOptions) -> Result<ContainerId>;
    async fn container_status(&self) -> Result<ContainerStatus>;
    async fn create_volume(&self) -> Result<Volume>;
    async fn fetch_logs(&self, opts: FetchLogOptions) -> Result<String>;
    async fn pull_image(&self, img: Image) -> Result<()>;
    async fn start_container(&self, id: ContainerId) -> Result<()>;
}

#[derive(Debug)]
pub(crate) enum ContainerStatus {
    Running,
    Exited,
    Other(String),
}

#[derive(Debug)]
pub(crate) enum ContainerError {
    BadParameter,
    BadResponse,
    NotFound,
    ServerError,
    Forbidden,
    Unknown,
    Transport,
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<StatusCode> for ContainerError {
    fn from(value: StatusCode) -> Self {
        todo!()
    }
}

// TODO:
impl From<serde_json::Error> for ContainerError {
    fn from(value: serde_json::Error) -> Self {
        ContainerError::BadResponse
    }
}

impl From<hyper::Error> for ContainerError {
    fn from(value: hyper::Error) -> Self {
        todo!()
    }
}

impl Error for ContainerError {}

pub(crate) fn raise_for_status<R>(s: StatusCode) -> Result<R> {
    todo!()
}
