use atmo_jetstream::Subscriber;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let url = "jetstream2.us-east.bsky.network";

    let mut subscriber = Subscriber::builder(url).connect().await.unwrap();

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
