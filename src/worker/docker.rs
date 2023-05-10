use std::path::{Path, PathBuf};

use super::container::{
    raise_for_status, ContainerError, ContainerId, ContainerOptions, ContainerStatus,
    FetchLogOptions, Image, Result, Service, Volume,
};
use async_trait::async_trait;
use hyper::body::to_bytes;
use hyper::{Body, Client, Method};
use hyper::{Request, Response};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub(crate) struct Docker {
    socket_path: PathBuf,
    client: Client<UnixConnector, Body>,
}

impl Docker {
    const VERSION: &'static str = "/v1.42/";

    fn new<P: AsRef<Path>>(socket_path: P) -> Docker {
        Docker {
            socket_path: socket_path.as_ref().to_path_buf(),
            client: Client::unix(),
        }
    }

    // TODO: move this into the trait
    async fn send_request<'a, S: Serialize, T: AsRef<str>>(
        &self,
        path: T,
        request: Option<S>,
        method: Method,
    ) -> Result<Response<Body>> {
        let req = Request::builder().method(method).uri(Uri::new(
            &self.socket_path,
            &format!("{}/{}", Self::VERSION, path.as_ref()),
        ));
        if let Some(r) = request {
            let json = serde_json::to_string(&r)?;
            Ok(self
                .client
                .request(
                    req.body(Body::from(json))
                        .map_err(|_| ContainerError::Transport)?,
                )
                .await?)
        } else {
            Ok(self
                .client
                .request(
                    req.body(Body::empty())
                        .map_err(|_| ContainerError::Transport)?,
                )
                .await?)
        }
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
        let request = CreateRequest {
            tty: true,
            labels: json!({
                "triad-ci":""
            }),
            entrypoint: vec!["/bin/sh".to_owned(), "-c".to_owned()],
            cmd: "echo \"$TRIAD_SCRIPT\" | /bin/sh".to_owned(),
            env: vec![format!("TRIAD_SCRIPT={}", options.script)],
            working_dir: "/app".to_owned(),
            host_config: HostConfig { binds: vec![bind] },
            image: options.image.to_string(),
        };
        let response = self
            .send_request::<CreateRequest, _>("containers/create", Some(request), Method::POST)
            .await?;
        raise_for_status(response.status())?;
        let create_response =
            serde_json::from_slice::<CreateResponse>(&to_bytes(response.into_body()).await?)?;
        Ok(create_response.id.into())
    }

    async fn start_container(&self, id: ContainerId) -> Result<()> {
        let response = self
            .send_request::<(), _>(&format!("containers/start/{}", id), None, Method::POST)
            .await?;
        Ok(())
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
        let query = format!("/images/create?tag={}&fromImage={}", img.name(), img.tag());
        let resp = self
            .send_request::<(), _>(query, None, Method::POST)
            .await?;
        raise_for_status(resp.status())?;
        Ok(())
    }
}
