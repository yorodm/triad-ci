use std::path::{Path, PathBuf};

use super::container::{
    ContainerId, ContainerOptions, ContainerStatus, FetchLogOptions, Image, Service, Volume,
};
use anyhow::Result;
use async_trait::async_trait;
use hyper::body::to_bytes;
use hyper::{Body, Client, Method};
use hyper::{Request, Response, Uri as HyperUri};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, json, Value};

pub(crate) struct Docker {
    socket: HyperUri,
    client: Client<UnixConnector, Body>,
}

impl Docker {
    fn new<P: AsRef<Path>>(socket_path: P) -> Docker {
        Docker {
            socket: Uri::new(socket_path, "/").into(),
            client: Client::unix(),
        }
    }

    async fn send_request<'a, S: Serialize>(
        &self,
        request: S,
        method: Method,
    ) -> Result<Response<Body>> {
        let json = serde_json::to_string(&request)?;
        let req = Request::builder()
            .method(method)
            .uri(&self.socket)
            .body(Body::from(json))?;
        Ok(self.client.request(req).await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateRequest {
    tty: bool,
    labels: Value,
    entrypoint: Vec<String>,
    cmd: String,
    env: Vec<String>,
    working_dir: String,
    host_config: HostConfig,
    image: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateResponse {
    id: String,
    warnings: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HostConfig {
    binds: Vec<String>,
}

#[async_trait]
impl Service for Docker {
    async fn create_container(&self, options: ContainerOptions) -> Result<ContainerId> {
        let bind = format!("{}:/app", options.volume.to_string());
        let image = options.image.to_string();
        let request = CreateRequest {
            tty: true,
            labels: json!({
                "triad-ci":""
            }),
            entrypoint: vec!["/bin/sh".to_owned(), "-c".to_owned()],
            cmd: "echo \"$QUAD_SCRIPT\" | /bin/sh".to_owned(),
            env: vec![format!("QUAD_SCRIPT={}", options.script)],
            working_dir: "/app".to_owned(),
            host_config: HostConfig { binds: vec![bind] },
            image: options.image.to_string(),
        };
        let response = self
            .send_request::<CreateRequest>(request, Method::POST)
            .await?;
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
