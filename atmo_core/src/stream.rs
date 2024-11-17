use serde::de::DeserializeOwned;

pub trait Subscription {
    type Params;
    type Message: DeserializeOwned;
}
