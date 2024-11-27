//! AT protocol [Jetstream] subscriber.
//!
//! [Jetstream]: https://github.com/bluesky-social/jetstream
use std::{
    num::NonZeroU32,
    pin::Pin,
    task::{ready, Poll},
};

use atmo_api::com::atproto::sync::subscribe_repos::{Account, Identity};
use atmo_core::{CidString, Did, Nsid, RecordKey, Unknown};
use futures::{SinkExt, Stream, StreamExt};
use http::{uri::InvalidUri, Uri};
use serde::{de::Error as _, Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{self, Message},
    MaybeTlsStream,
};

mod error;

pub use error::Error;

const DEFAULT_DECOMPRESS_LIMIT: usize = 65536;
static ZSTD_DICT: &[u8] = include_bytes!("../zstd_dictionary");

/// A Jetstream subscriber.
///
/// This type wraps a WebSocket and deserializes [`Event`]s sent from the Jetstream server.
pub struct Subscriber {
    ws: tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
    decode: zstd::bulk::Decompressor<'static>,
    // Store URI for reconnects (TODO).
    _uri: Uri,
}

impl Subscriber {
    /// Creates a new `Subscriber`.
    ///
    /// This connects to the Jetstream server at `uri` and sets the initial subscription options to
    /// `opts`.
    #[tracing::instrument(skip(uri))]
    pub async fn new<U>(uri: U, opts: Options) -> Result<Self, Error>
    where
        Uri: TryFrom<U, Error = InvalidUri>,
    {
        let decode = zstd::bulk::Decompressor::with_dictionary(ZSTD_DICT).unwrap();

        let uri = Uri::try_from(uri).map_err(|e| tungstenite::Error::HttpFormat(e.into()))?;
        let uri = http::uri::Builder::from(uri)
            .scheme("wss")
            .path_and_query("/subscribe?requireHello=true&compress=true")
            .build()
            .map_err(tungstenite::Error::from)?;

        let (mut ws, _resp) = tokio_tungstenite::connect_async(&uri).await?;

        let msg = SubscriberSourcedMessage::OptionsUpdate(opts);
        let msg_s = serde_json::to_string(&msg).expect("serialization should not fail");

        ws.send(Message::Text(msg_s)).await?;

        Ok(Self {
            ws,
            decode,
            _uri: uri,
        })
    }

    /// Updates the subscription options for this subscriber.
    #[tracing::instrument(skip(self))]
    pub async fn update_options(&mut self, new_opts: Options) -> Result<(), Error> {
        let msg = SubscriberSourcedMessage::OptionsUpdate(new_opts);
        let msg_s = serde_json::to_string(&msg).expect("serialization should not fail");

        self.ws.send(Message::Text(msg_s)).await?;

        Ok(())
    }
}

impl Stream for Subscriber {
    type Item = Result<Event, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let msg = match ready!(self.ws.poll_next_unpin(cx)) {
            Some(Ok(msg)) => msg,
            Some(Err(e)) => return Poll::Ready(Some(Err(Error::WebSocket(e)))),
            None => return Poll::Ready(None),
        };

        let bytes = match msg {
            Message::Binary(b) => self
                .decode
                .decompress(&b, DEFAULT_DECOMPRESS_LIMIT)
                .unwrap(),

            Message::Close(c) => return Poll::Ready(Some(Err(Error::Closed(c)))),

            _ => {
                tracing::debug!("unexpected message type");
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        let event = serde_json::from_slice(&bytes).unwrap();

        Poll::Ready(Some(Ok(event)))
    }
}

/// A Jetstream event.
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The DID of the repo where the event occurred.
    pub did: Did,
    /// The Unix timestamp of the event in microseconds.
    pub time_us: u64,
    /// The event kind.
    #[serde(flatten)]
    pub kind: EventKind,
}

/// An enumeration of Jetstream event kinds.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// A commit of a record to a repo.
    Commit(Commit),
    /// An identity update.
    Identity(Identity),
    /// An account status update.
    Account(Account),
}

/// An event representing a commit of a record to a repo.
#[derive(Debug)]
pub struct Commit {
    pub rev: String,
    pub collection: Nsid,
    pub rkey: RecordKey,
    pub operation: Operation,
}

impl<'de> Deserialize<'de> for Commit {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = CommitRaw::deserialize(des)?;

        let operation = match raw.operation {
            OperationTag::Create => Operation::Create(
                raw.commit_data
                    .ok_or_else(|| D::Error::custom("missing commit data"))?,
            ),
            OperationTag::Update => Operation::Update(
                raw.commit_data
                    .ok_or_else(|| D::Error::custom("missing commit data"))?,
            ),
            OperationTag::Delete => Operation::Delete,
        };

        Ok(Commit {
            rev: raw.rev,
            collection: raw.collection,
            rkey: raw.rkey,
            operation,
        })
    }
}

/// An enumeration of commit operations.
#[derive(Debug)]
pub enum Operation {
    Create(CommitData),
    Update(CommitData),
    Delete,
}

#[derive(Deserialize)]
struct CommitRaw {
    rev: String,
    collection: Nsid,
    rkey: RecordKey,
    operation: OperationTag,
    #[serde(flatten)]
    commit_data: Option<CommitData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum OperationTag {
    Create,
    Update,
    Delete,
}

/// The commit data associated with a `Create` or `Update` operation.
#[derive(Debug, Deserialize)]
pub struct CommitData {
    pub record: Unknown,
    pub cid: CidString,
}

/// A message sent from a subscriber to the Jetstream server.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum SubscriberSourcedMessage {
    OptionsUpdate(Options),
}

/// Subscription options for a subscriber.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub wanted_collections: Vec<String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub wanted_dids: Vec<Did>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub max_message_size_bytes: Option<NonZeroU32>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn example_commit_create_like() {
        let input = json!({
            "did": "did:plc:eygmaihciaxprqvxpfvl6flk",
            "time_us": 1725911162329308_u64,
            "kind": "commit",
            "commit": {
                "rev": "3l3qo2vutsw2b",
                "operation": "create",
                "collection": "app.bsky.feed.like",
                "rkey": "3l3qo2vuowo2b",
                "record": {
                "$type": "app.bsky.feed.like",
                "createdAt": "2024-09-09T19:46:02.102Z",
                "subject": {
                    "cid": "bafyreidc6sydkkbchcyg62v77wbhzvb2mvytlmsychqgwf2xojjtirmzj4",
                    "uri": "at://did:plc:wa7b35aakoll7hugkrjtf3xf/app.bsky.feed.post/3l3pte3p2e325"
                }
                },
                "cid": "bafyreidwaivazkwu67xztlmuobx35hs2lnfh3kolmgfmucldvhd3sgzcqi"
            }
        });

        let evt: Event = serde_json::from_value(input).unwrap();

        let commit = match evt.kind {
            EventKind::Commit(commit) => commit,
            e => panic!("wrong event kind: expected Commit, got {e:?}"),
        };

        let create = match commit.operation {
            Operation::Create(create) => create,
            o => panic!("wrong operation: expected Create, got {o:?}"),
        };

        use atmo_api::app::bsky::feed::Like;
        let _like = create
            .record
            .downcast::<Like>()
            .expect("couldn't downcast to Like");
    }

    #[test]
    fn example_commit_delete_follow() {
        let input = json!({
            "did": "did:plc:rfov6bpyztcnedeyyzgfq42k",
            "time_us": 1725516666833633_u64,
            "kind": "commit",
            "commit": {
                "rev": "3l3f6nzl3cv2s",
                "operation": "delete",
                "collection": "app.bsky.graph.follow",
                "rkey": "3l3dn7tku762u"
            }
        });

        let evt: Event = serde_json::from_value(input).unwrap();

        let commit = match evt.kind {
            EventKind::Commit(commit) => commit,
            e => panic!("wrong event kind: expected Commit, got {e:?}"),
        };

        assert!(matches!(commit.operation, Operation::Delete));
    }

    #[test]
    fn example_identity() {
        let input = json!({
            "did": "did:plc:ufbl4k27gp6kzas5glhz7fim",
            "time_us": 1725516665234703_u64,
            "kind": "identity",
            "identity": {
                "did": "did:plc:ufbl4k27gp6kzas5glhz7fim",
                "handle": "yohenrique.bsky.social",
                "seq": 1409752997,
                "time": "2024-09-05T06:11:04.870Z"
            }
        });

        let evt: Event = serde_json::from_value(input).unwrap();

        let _id = match evt.kind {
            EventKind::Identity(id) => id,
            e => panic!("wrong event kind: expected Identity, got {e:?}"),
        };
    }

    #[test]
    fn example_account() {
        let input = json!({
            "did": "did:plc:ufbl4k27gp6kzas5glhz7fim",
            "time_us": 1725516665333808_u64,
            "kind": "account",
            "account": {
                "active": true,
                "did": "did:plc:ufbl4k27gp6kzas5glhz7fim",
                "seq": 1409753013,
                "time": "2024-09-05T06:11:04.870Z"
            }
        });

        let evt: Event = serde_json::from_value(input).unwrap();

        let _acct = match evt.kind {
            EventKind::Account(acct) => acct,
            e => panic!("wrong event kind: expected Account, got {e:?}"),
        };
    }

    #[test]
    fn example_subscriber_options_update() {
        let expected = json!({
            "type": "options_update",
            "payload": {
                "wantedCollections": ["app.bsky.feed.post"],
                "wantedDids": ["did:plc:q6gjnaw2blty4crticxkmujt"],
                "maxMessageSizeBytes": 1000000
            }
        });

        let serialized = serde_json::to_value(SubscriberSourcedMessage::OptionsUpdate(Options {
            wanted_collections: vec!["app.bsky.feed.post".into()],
            wanted_dids: vec!["did:plc:q6gjnaw2blty4crticxkmujt".parse().unwrap()],
            max_message_size_bytes: NonZeroU32::new(1000000),
        }))
        .unwrap();

        assert_eq!(expected, serialized);
    }
}
