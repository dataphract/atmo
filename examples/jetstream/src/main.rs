use atmo_jetstream::{EventKind, Options, Subscriber};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let url = "jetstream2.us-east.bsky.network";
    let opts = Options {
        ..Default::default()
    };

    let mut subscriber = Subscriber::new(url, opts).await.unwrap();

    while let Some(res) = subscriber.next().await {
        let evt = match res {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        let acct = match evt.kind {
            EventKind::Account(acct) => acct,
            _ => continue,
        };

        println!("{acct:?}");
    }

    eprintln!("WebSocket closed");
}
