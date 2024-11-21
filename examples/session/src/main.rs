use atmo::{
    api::{
        app::bsky::actor::GetPreferences,
        com::atproto::server::{create_session, CreateSession, DeleteSession},
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

    let session_resp = cl
        .request(&url, CreateSession)
        .input(&create_session::Input {
            identifier,
            password,
            auth_factor_token: None,
        })
        .send()
        .await
        .unwrap();

    let session = session_resp.result().unwrap();

    let did_doc: DidDoc = session.did_doc.as_ref().unwrap().downcast().unwrap();
    println!("DID doc: {did_doc:?}");

    let prefs_resp = cl
        .request(&url, GetPreferences)
        .bearer_auth(&session.access_jwt)
        .send()
        .await
        .unwrap();

    let prefs = prefs_resp.result().unwrap();

    for pref in &prefs.preferences {
        println!("pref: {pref:#?}");
    }

    let del_resp = cl
        .request(&url, DeleteSession)
        .bearer_auth(&session.refresh_jwt)
        .send()
        .await
        .unwrap();

    if let Err(e) = del_resp.result() {
        eprintln!("{e:?}");
    }
}
