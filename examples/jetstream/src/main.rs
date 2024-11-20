use atmo_jetstream::OptionsUpdate;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use url::Url;

static ZSTD_DICT: &[u8] = include_bytes!("../../../atmo_jetstream/zstd_dictionary");

#[tokio::main]
async fn main() {
    let mut decoder = zstd::bulk::Decompressor::with_dictionary(ZSTD_DICT).unwrap();

    let url = Url::parse_with_params(
        "wss://jetstream2.us-east.bsky.network/subscribe",
        [("requireHello", "true"), ("compress", "true")],
    )
    .unwrap();

    let (mut ws, _resp) = tokio_tungstenite::connect_async(url).await.unwrap();

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

        let Message::Binary(compressed) = msg else {
            continue;
        };

        let bytes = decoder.decompress(&compressed, 65536).unwrap();
        let evt: atmo_jetstream::Event = serde_json::from_slice(&bytes).unwrap();

        println!("{evt:?}");
    }
}
