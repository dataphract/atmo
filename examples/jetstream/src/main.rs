use atmo_jetstream::OptionsUpdate;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse_with_params(
        "wss://jetstream2.us-east.bsky.network/subscribe",
        [("requireHello", "true")],
    )
    .unwrap();

    let (mut ws, resp) = tokio_tungstenite::connect_async(url).await.unwrap();

    let options_update = atmo_jetstream::SubscriberSourcedMessage::OptionsUpdate(OptionsUpdate {
        ..Default::default()
    });

    ws.send(serde_json::to_string(&options_update).unwrap().into())
        .await
        .unwrap();

    while let Some(res) = ws.next().await {
        let msg = match res {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        let Message::Text(t) = msg else {
            continue;
        };

        let evt: atmo_jetstream::Event = serde_json::from_str(&t).unwrap();

        println!("{evt:?}");
    }
}
