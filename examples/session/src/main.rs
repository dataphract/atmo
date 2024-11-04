use atmo::{
    api::com::atproto::server::{create_session, CreateSession},
    core::did::DidDoc,
    XrpcClient,
};
use url::Url;

#[tokio::main]
async fn main() {
    let cl = XrpcClient::new();

    let url = Url::parse("https://bsky.social").unwrap();

    let identifier = dialoguer::Input::new()
        .with_prompt("Bluesky username")
        .interact_text()
        .unwrap();

    let password = dialoguer::Password::new()
        .with_prompt("Bluesky password")
        .interact()
        .unwrap();

    let resp = cl
        .request(&url, CreateSession)
        .input(&create_session::Input {
            identifier,
            password,
            auth_factor_token: None,
        })
        .send()
        .await
        .unwrap();

    let _access = resp.access_jwt;
    let _refresh = resp.refresh_jwt;

    let did_doc: DidDoc = resp.did_doc.as_ref().unwrap().downcast().unwrap();

    println!("{:#?}", did_doc);
}
