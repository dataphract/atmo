pub trait Subscription {
    type Params;
    type Message;
    type Error;
}
