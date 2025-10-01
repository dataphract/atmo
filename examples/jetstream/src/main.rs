use std::process::exit;

use atmo::{
    api::com::atproto::identity::{resolve_handle, ResolveHandle},
    jetstream::Subscriber,
};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cl = atmo::api::XrpcClient::new();

    let handle = "bsky.app";
    let url = "https://bsky.social".try_into().unwrap();
    let resp = cl
        .request(&url, ResolveHandle)
        .params(&resolve_handle::Params {
            handle: handle.parse().unwrap(),
        })
        .expect("serializing params failed")
        .send()
        .await
        .expect("sending request failed");

    let did = match resp.result() {
        Ok(out) => out.did.clone(),
        Err(e) => {
            eprintln!("RPC error: {e}");
            exit(1);
        }
    };

    println!("resolved handle `{handle}` to DID `{}`", did.as_str());

    let url = "jetstream2.us-east.bsky.network";

    let mut subscriber = Subscriber::builder(url)
        .wanted_collections(vec!["app.bsky.feed.like".into()])
        .wanted_dids(vec![])
        .connect()
        .await
        .unwrap();

    println!("listening for posts...");
    while let Some(res) = subscriber.next().await {
        let evt = match res {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        println!("{evt:?}");
    }

    eprintln!("WebSocket closed");
}
