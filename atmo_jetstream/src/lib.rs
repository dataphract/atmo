use atmo_api::com::atproto::sync::subscribe_repos::{Account, Identity};
use atmo_core::{CidString, Did, Nsid, RecordKey, Unknown};
use serde::{de::Error as _, Deserialize};

#[derive(Deserialize)]
pub struct Event {
    pub did: Did,
    pub time_us: u64,
    #[serde(flatten)]
    pub kind: EventKind,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Commit(Commit),
    Identity(Identity),
    Account(Account),
}

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

#[derive(Debug, Deserialize)]
pub struct CommitData {
    pub record: Unknown,
    pub cid: CidString,
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
}
