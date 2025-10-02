use atmo::jetstream::Subscriber;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let url = "jetstream2.us-east.bsky.network";

    let mut subscriber = Subscriber::builder(url)
        .wanted_collections(vec!["app.bsky.feed.*".into()])
        .wanted_dids(vec![])
        .connect()
        .await
        .unwrap();

    println!("listening for `app.bsky.feed.*` events...");
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
