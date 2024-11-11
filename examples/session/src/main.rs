use atmo::{
    api::{
        app::bsky::actor::{defs::Preferences, GetPreferences},
        com::atproto::server::{create_session, CreateSession},
    },
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

    let session = cl
        .request(&url, CreateSession)
        .input(&create_session::Input {
            identifier,
            password,
            auth_factor_token: None,
        })
        .send()
        .await
        .unwrap();

    let did_doc: DidDoc = session.did_doc.as_ref().unwrap().downcast().unwrap();

    let resp = cl
        .request(&url, GetPreferences)
        .bearer_auth(&session.access_jwt)
        .send()
        .await
        .unwrap();

    for pref in resp.preferences {
        println!("pref: {pref:#?}");
    }
    //println!("preferences: {resp:#?}");
}
