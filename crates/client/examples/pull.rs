/*
   Copyright The containerd Authors.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use client::services::v1::{
    images_client::ImagesClient, transfer_client::TransferClient, ListImagesRequest,
    TransferRequest,
};
use containerd_client as client;
use prost::Message;
use prost_types::Any;

/// Make sure you run containerd before running this example.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect Failed");

    let mut client = TransferClient::new(channel.clone());
    let registry = Any {
        type_url: "types.containerd.io/containerd.types.transfer.OCIRegistry".to_string(),
        value: client::types::OciRegistry {
            reference: "registry-1.docker.io/library/hello-world:latest".to_string(),
            ..Default::default()
        }
        .encode_to_vec(),
    };
    let image_store = Any {
        type_url: "types.containerd.io/containerd.types.transfer.ImageStore".to_string(),
        value: client::types::ImageStore {
            name: "registry-1.docker.io/library/hello-world:latest".to_string(),
            ..Default::default()
        }
        .encode_to_vec(),
    };
    let req = TransferRequest {
        source: Some(registry),
        destination: Some(image_store),
        options: None,
    };
    let _resp = client.transfer(req).await.expect("Failed to pull image");

    let mut client = ImagesClient::new(channel.clone());
    let req = ListImagesRequest { filters: vec![] };
    let resp = client.list(req).await.expect("Failed to list images");

    println!("Response: {:?}", resp.get_ref());
}
