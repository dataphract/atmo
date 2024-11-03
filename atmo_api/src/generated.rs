pub mod app {
    pub mod bsky {
        pub mod actor {
            pub struct GetPreferences;
            impl atmo_core::xrpc::Request for GetPreferences {
                type Params = crate::app::bsky::actor::get_preferences::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::get_preferences::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.getPreferences"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetProfile;
            impl atmo_core::xrpc::Request for GetProfile {
                type Params = crate::app::bsky::actor::get_profile::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::defs::ProfileViewDetailed;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.getProfile"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetProfiles;
            impl atmo_core::xrpc::Request for GetProfiles {
                type Params = crate::app::bsky::actor::get_profiles::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::get_profiles::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.getProfiles"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSuggestions;
            impl atmo_core::xrpc::Request for GetSuggestions {
                type Params = crate::app::bsky::actor::get_suggestions::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::get_suggestions::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.getSuggestions"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct PutPreferences;
            impl atmo_core::xrpc::Request for PutPreferences {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::actor::put_preferences::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.putPreferences"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct SearchActors;
            impl atmo_core::xrpc::Request for SearchActors {
                type Params = crate::app::bsky::actor::search_actors::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::search_actors::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.searchActors"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchActorsTypeahead;
            impl atmo_core::xrpc::Request for SearchActorsTypeahead {
                type Params = crate::app::bsky::actor::search_actors_typeahead::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::actor::search_actors_typeahead::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.actor.searchActorsTypeahead"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ActorTarget {
                    #[serde(rename = "all")]
                    All,
                    #[serde(rename = "exclude-following")]
                    ExcludeFollowing,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AdultContentPref {
                    pub enabled: bool,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum AllowIncoming {
                    #[serde(rename = "all")]
                    All,
                    #[serde(rename = "none")]
                    None,
                    #[serde(rename = "following")]
                    Following,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BskyAppProgressGuide {
                    pub guide: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BskyAppStatePref {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active_progress_guide:
                        std::option::Option<crate::app::bsky::actor::defs::BskyAppProgressGuide>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub nuxs:
                        std::option::Option<std::vec::Vec<crate::app::bsky::actor::defs::Nux>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub queued_nudges: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ContentLabelPref {
                    pub label: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labeler_did: std::option::Option<atmo_core::Did>,
                    pub visibility: crate::app::bsky::actor::defs::Visibility,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FeedViewPref {
                    pub feed: std::string::String,
                    #[doc = "Hide quote posts in the feed."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_quote_posts: std::option::Option<bool>,
                    #[doc = "Hide replies in the feed."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_like_count: std::option::Option<i64>,
                    #[doc = "Hide replies in the feed if they are not by followed users."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_unfollowed: std::option::Option<bool>,
                    #[doc = "Hide reposts in the feed."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_reposts: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct HiddenPostsPref {
                    pub items: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InterestsPref {
                    pub tags: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct KnownFollowers {
                    pub count: i64,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPrefItem {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelersPref {
                    pub labelers: std::vec::Vec<crate::app::bsky::actor::defs::LabelerPrefItem>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MutedWord {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub actor_target:
                        std::option::Option<crate::app::bsky::actor::defs::ActorTarget>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub id: std::option::Option<std::string::String>,
                    pub targets: std::vec::Vec<crate::app::bsky::actor::defs::MutedWordTarget>,
                    pub value: std::string::String,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum MutedWordTarget {
                    #[serde(rename = "content")]
                    Content,
                    #[serde(rename = "tag")]
                    Tag,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MutedWordsPref {
                    pub items: std::vec::Vec<crate::app::bsky::actor::defs::MutedWord>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Nux {
                    pub completed: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub data: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo_core::DateTimeString>,
                    pub id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct PersonalDetailsPref {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub birth_date: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Preferences {
                    AdultContentPref(crate::app::bsky::actor::defs::AdultContentPref),
                    BskyAppStatePref(crate::app::bsky::actor::defs::BskyAppStatePref),
                    ContentLabelPref(crate::app::bsky::actor::defs::ContentLabelPref),
                    FeedViewPref(crate::app::bsky::actor::defs::FeedViewPref),
                    HiddenPostsPref(crate::app::bsky::actor::defs::HiddenPostsPref),
                    InterestsPref(crate::app::bsky::actor::defs::InterestsPref),
                    LabelersPref(crate::app::bsky::actor::defs::LabelersPref),
                    MutedWordsPref(crate::app::bsky::actor::defs::MutedWordsPref),
                    PersonalDetailsPref(crate::app::bsky::actor::defs::PersonalDetailsPref),
                    SavedFeedsPref(crate::app::bsky::actor::defs::SavedFeedsPref),
                    SavedFeedsPrefV2(crate::app::bsky::actor::defs::SavedFeedsPrefV2),
                    ThreadViewPref(crate::app::bsky::actor::defs::ThreadViewPref),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileAssociated {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociatedChat>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feedgens: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labeler: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lists: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub starter_packs: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileAssociatedChat {
                    pub allow_incoming: crate::app::bsky::actor::defs::AllowIncoming,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewDetailed {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub banner: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followers_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub follows_count: std::option::Option<i64>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_via_starter_pack:
                        std::option::Option<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned_post: std::option::Option<crate::com::atproto::repo::StrongRef>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub posts_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeed {
                    pub id: std::string::String,
                    pub pinned: bool,
                    pub ty: crate::app::bsky::actor::defs::Type,
                    pub value: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeedsPref {
                    pub pinned: std::vec::Vec<std::string::String>,
                    pub saved: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub timeline_index: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeedsPrefV2 {
                    pub items: std::vec::Vec<crate::app::bsky::actor::defs::SavedFeed>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Sort {
                    #[serde(rename = "oldest")]
                    Oldest,
                    #[serde(rename = "newest")]
                    Newest,
                    #[serde(rename = "most-likes")]
                    MostLikes,
                    #[serde(rename = "random")]
                    Random,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadViewPref {
                    #[doc = "Show followed users at the top of all replies."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prioritize_followed_users: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort: std::option::Option<crate::app::bsky::actor::defs::Sort>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Type {
                    #[serde(rename = "feed")]
                    Feed,
                    #[serde(rename = "list")]
                    List,
                    #[serde(rename = "timeline")]
                    Timeline,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked_by: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking_by_list:
                        std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub known_followers:
                        std::option::Option<crate::app::bsky::actor::defs::KnownFollowers>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted_by_list:
                        std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Visibility {
                    #[serde(rename = "ignore")]
                    Ignore,
                    #[serde(rename = "show")]
                    Show,
                    #[serde(rename = "warn")]
                    Warn,
                    #[serde(rename = "hide")]
                    Hide,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod get_preferences {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub preferences: std::vec::Vec<crate::app::bsky::actor::defs::Preferences>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {}
            }
            pub mod get_profile {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod get_profiles {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub profiles: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewDetailed>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actors: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_suggestions {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod put_preferences {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub preferences: std::vec::Vec<crate::app::bsky::actor::defs::Preferences>,
                }
            }
            pub mod search_actors {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub q: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub term: std::option::Option<std::string::String>,
                }
            }
            pub mod search_actors_typeahead {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub q: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub term: std::option::Option<std::string::String>,
                }
            }
        }
        pub mod embed {
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct External {
                pub external: crate::app::bsky::embed::external::External,
            }
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct Images {
                pub images: std::vec::Vec<crate::app::bsky::embed::images::Image>,
            }
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct Record {
                pub record: crate::com::atproto::repo::StrongRef,
            }
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct RecordWithMedia {
                pub media: crate::app::bsky::embed::record_with_media::Media,
                pub record: crate::app::bsky::embed::Record,
            }
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct Video {
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub alt: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub aspect_ratio: std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub captions:
                    std::option::Option<std::vec::Vec<crate::app::bsky::embed::video::Caption>>,
                pub video: atmo_core::Blob,
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AspectRatio {
                    pub height: i64,
                    pub width: i64,
                }
            }
            pub mod external {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct External {
                    pub description: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumb: std::option::Option<atmo_core::Blob>,
                    pub title: std::string::String,
                    pub uri: url::Url,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub external: crate::app::bsky::embed::external::ViewExternal,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewExternal {
                    pub description: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumb: std::option::Option<url::Url>,
                    pub title: std::string::String,
                    pub uri: url::Url,
                }
            }
            pub mod images {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Image {
                    pub alt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub image: atmo_core::Blob,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub images: std::vec::Vec<crate::app::bsky::embed::images::ViewImage>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewImage {
                    pub alt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub fullsize: url::Url,
                    pub thumb: url::Url,
                }
            }
            pub mod record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Embeds {
                    ExternalView(crate::app::bsky::embed::external::View),
                    ImagesView(crate::app::bsky::embed::images::View),
                    RecordView(crate::app::bsky::embed::record::View),
                    RecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
                    VideoView(crate::app::bsky::embed::video::View),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Record {
                    GeneratorView(crate::app::bsky::feed::defs::GeneratorView),
                    LabelerView(crate::app::bsky::labeler::defs::LabelerView),
                    ListView(crate::app::bsky::graph::defs::ListView),
                    StarterPackViewBasic(crate::app::bsky::graph::defs::StarterPackViewBasic),
                    ViewBlocked(crate::app::bsky::embed::record::ViewBlocked),
                    ViewDetached(crate::app::bsky::embed::record::ViewDetached),
                    ViewNotFound(crate::app::bsky::embed::record::ViewNotFound),
                    ViewRecord(crate::app::bsky::embed::record::ViewRecord),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub record: crate::app::bsky::embed::record::Record,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewBlocked {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewDetached {
                    pub detached: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewNotFound {
                    pub not_found: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewRecord {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embeds:
                        std::option::Option<std::vec::Vec<crate::app::bsky::embed::record::Embeds>>,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
            }
            pub mod record_with_media {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Media {
                    External(crate::app::bsky::embed::External),
                    Images(crate::app::bsky::embed::Images),
                    Video(crate::app::bsky::embed::Video),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub media: crate::app::bsky::embed::record_with_media::ViewMedia,
                    pub record: crate::app::bsky::embed::record::View,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ViewMedia {
                    ExternalView(crate::app::bsky::embed::external::View),
                    ImagesView(crate::app::bsky::embed::images::View),
                    VideoView(crate::app::bsky::embed::video::View),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod video {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Caption {
                    pub file: atmo_core::Blob,
                    pub lang: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub alt: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub cid: atmo_core::CidString,
                    pub playlist: url::Url,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumbnail: std::option::Option<url::Url>,
                }
            }
        }
        pub mod feed {
            pub struct DescribeFeedGenerator;
            impl atmo_core::xrpc::Request for DescribeFeedGenerator {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::describe_feed_generator::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.describeFeedGenerator"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetActorFeeds;
            impl atmo_core::xrpc::Request for GetActorFeeds {
                type Params = crate::app::bsky::feed::get_actor_feeds::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_actor_feeds::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getActorFeeds"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetActorLikes;
            impl atmo_core::xrpc::Request for GetActorLikes {
                type Params = crate::app::bsky::feed::get_actor_likes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_actor_likes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getActorLikes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetAuthorFeed;
            impl atmo_core::xrpc::Request for GetAuthorFeed {
                type Params = crate::app::bsky::feed::get_author_feed::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_author_feed::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getAuthorFeed"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFeed;
            impl atmo_core::xrpc::Request for GetFeed {
                type Params = crate::app::bsky::feed::get_feed::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_feed::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getFeed"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFeedGenerator;
            impl atmo_core::xrpc::Request for GetFeedGenerator {
                type Params = crate::app::bsky::feed::get_feed_generator::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_feed_generator::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getFeedGenerator"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFeedGenerators;
            impl atmo_core::xrpc::Request for GetFeedGenerators {
                type Params = crate::app::bsky::feed::get_feed_generators::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_feed_generators::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getFeedGenerators"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFeedSkeleton;
            impl atmo_core::xrpc::Request for GetFeedSkeleton {
                type Params = crate::app::bsky::feed::get_feed_skeleton::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_feed_skeleton::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getFeedSkeleton"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetLikes;
            impl atmo_core::xrpc::Request for GetLikes {
                type Params = crate::app::bsky::feed::get_likes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_likes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getLikes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetListFeed;
            impl atmo_core::xrpc::Request for GetListFeed {
                type Params = crate::app::bsky::feed::get_list_feed::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_list_feed::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getListFeed"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetPostThread;
            impl atmo_core::xrpc::Request for GetPostThread {
                type Params = crate::app::bsky::feed::get_post_thread::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_post_thread::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getPostThread"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetPosts;
            impl atmo_core::xrpc::Request for GetPosts {
                type Params = crate::app::bsky::feed::get_posts::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_posts::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getPosts"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetQuotes;
            impl atmo_core::xrpc::Request for GetQuotes {
                type Params = crate::app::bsky::feed::get_quotes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_quotes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getQuotes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRepostedBy;
            impl atmo_core::xrpc::Request for GetRepostedBy {
                type Params = crate::app::bsky::feed::get_reposted_by::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_reposted_by::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getRepostedBy"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSuggestedFeeds;
            impl atmo_core::xrpc::Request for GetSuggestedFeeds {
                type Params = crate::app::bsky::feed::get_suggested_feeds::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_suggested_feeds::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getSuggestedFeeds"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetTimeline;
            impl atmo_core::xrpc::Request for GetTimeline {
                type Params = crate::app::bsky::feed::get_timeline::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::get_timeline::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.getTimeline"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchPosts;
            impl atmo_core::xrpc::Request for SearchPosts {
                type Params = crate::app::bsky::feed::search_posts::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::feed::search_posts::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.searchPosts"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SendInteractions;
            impl atmo_core::xrpc::Request for SendInteractions {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::feed::send_interactions::Input;
                type Output = crate::app::bsky::feed::send_interactions::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.feed.sendInteractions"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedAuthor {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedPost {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Embed {
                    ExternalView(crate::app::bsky::embed::external::View),
                    ImagesView(crate::app::bsky::embed::images::View),
                    RecordView(crate::app::bsky::embed::record::View),
                    RecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
                    VideoView(crate::app::bsky::embed::video::View),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Event {
                    #[doc = "Request that less content like the given feed item be shown in the feed"]
                    #[serde(rename = "app.bsky.feed.defs#requestLess")]
                    RequestLess,
                    #[doc = "Request that more content like the given feed item be shown in the feed"]
                    #[serde(rename = "app.bsky.feed.defs#requestMore")]
                    RequestMore,
                    #[doc = "User clicked through to the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#clickthroughItem")]
                    ClickthroughItem,
                    #[doc = "User clicked through to the author of the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#clickthroughAuthor")]
                    ClickthroughAuthor,
                    #[doc = "User clicked through to the reposter of the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#clickthroughReposter")]
                    ClickthroughReposter,
                    #[doc = "User clicked through to the embedded content of the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#clickthroughEmbed")]
                    ClickthroughEmbed,
                    #[doc = "Feed item was seen by user"]
                    #[serde(rename = "app.bsky.feed.defs#interactionSeen")]
                    InteractionSeen,
                    #[doc = "User liked the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#interactionLike")]
                    InteractionLike,
                    #[doc = "User reposted the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#interactionRepost")]
                    InteractionRepost,
                    #[doc = "User replied to the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#interactionReply")]
                    InteractionReply,
                    #[doc = "User quoted the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#interactionQuote")]
                    InteractionQuote,
                    #[doc = "User shared the feed item"]
                    #[serde(rename = "app.bsky.feed.defs#interactionShare")]
                    InteractionShare,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FeedViewPost {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<crate::app::bsky::feed::defs::Reason>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply: std::option::Option<crate::app::bsky::feed::defs::ReplyRef>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub accepts_interactions: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub did: atmo_core::Did,
                    pub display_name: std::string::String,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::feed::defs::GeneratorViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Interaction {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub event: std::option::Option<crate::app::bsky::feed::defs::Event>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub item: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundPost {
                    pub not_found: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Parent {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    PostView(crate::app::bsky::feed::defs::PostView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct PostView {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed: std::option::Option<crate::app::bsky::feed::defs::Embed>,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    pub record: atmo_core::Unknown,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threadgate:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadgateView>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::feed::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Reason {
                    ReasonPin(crate::app::bsky::feed::defs::ReasonPin),
                    ReasonRepost(crate::app::bsky::feed::defs::ReasonRepost),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonPin {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonRepost {
                    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub indexed_at: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Replies {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    ThreadViewPost(std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub grandparent_author:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileViewBasic>,
                    pub parent: crate::app::bsky::feed::defs::Parent,
                    pub root: crate::app::bsky::feed::defs::Root,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Root {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    PostView(crate::app::bsky::feed::defs::PostView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonFeedPost {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason:
                        std::option::Option<crate::app::bsky::feed::defs::SkeletonFeedPostReason>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum SkeletonFeedPostReason {
                    SkeletonReasonPin(crate::app::bsky::feed::defs::SkeletonReasonPin),
                    SkeletonReasonRepost(crate::app::bsky::feed::defs::SkeletonReasonRepost),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonPin {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonRepost {
                    pub repost: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadViewPost {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub parent:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadViewPostParent>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub replies:
                        std::option::Option<std::vec::Vec<crate::app::bsky::feed::defs::Replies>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ThreadViewPostParent {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    ThreadViewPost(std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadgateView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lists: std::option::Option<
                        std::vec::Vec<crate::app::bsky::graph::defs::ListViewBasic>,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub record: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub uri: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embedding_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thread_muted: std::option::Option<bool>,
                }
            }
            pub mod describe_feed_generator {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Feed {
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub did: atmo_core::Did,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::describe_feed_generator::Feed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub links:
                        std::option::Option<crate::app::bsky::feed::describe_feed_generator::Links>,
                }
            }
            pub mod get_actor_feeds {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_actor_likes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_author_feed {
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Filter {
                    #[serde(rename = "posts_with_replies")]
                    PostsWithReplies,
                    #[serde(rename = "posts_no_replies")]
                    PostsNoReplies,
                    #[serde(rename = "posts_with_media")]
                    PostsWithMedia,
                    #[serde(rename = "posts_and_author_threads")]
                    PostsAndAuthorThreads,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub filter:
                        std::option::Option<crate::app::bsky::feed::get_author_feed::Filter>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_pins: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_feed {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_feed_generator {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[doc = "Indicates whether the feed generator service has been online recently, or else seems to be inactive."]
                    pub is_online: bool,
                    #[doc = "Indicates whether the feed generator service is compatible with the record declaration."]
                    pub is_valid: bool,
                    pub view: crate::app::bsky::feed::defs::GeneratorView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub feed: atmo_core::AtUri,
                }
            }
            pub mod get_feed_generators {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub feeds: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_feed_skeleton {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::SkeletonFeedPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_likes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Like {
                    pub actor: crate::app::bsky::actor::defs::ProfileView,
                    pub created_at: atmo_core::DateTimeString,
                    pub indexed_at: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub likes: std::vec::Vec<crate::app::bsky::feed::get_likes::Like>,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_list_feed {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod get_post_thread {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub thread: crate::app::bsky::feed::get_post_thread::Thread,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threadgate:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadgateView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub depth: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub parent_height: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Thread {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    ThreadViewPost(crate::app::bsky::feed::defs::ThreadViewPost),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod get_posts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub posts: std::vec::Vec<crate::app::bsky::feed::defs::PostView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_quotes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub posts: std::vec::Vec<crate::app::bsky::feed::defs::PostView>,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_reposted_by {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub reposted_by: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_suggested_feeds {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_timeline {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub algorithm: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod post {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Entity {
                    pub index: crate::app::bsky::feed::post::TextSlice,
                    pub ty: std::string::String,
                    pub value: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
                    pub parent: crate::com::atproto::repo::StrongRef,
                    pub root: crate::com::atproto::repo::StrongRef,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct TextSlice {
                    pub end: i64,
                    pub start: i64,
                }
            }
            pub mod postgate {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct DisableRule {}
            }
            pub mod search_posts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                    pub posts: std::vec::Vec<crate::app::bsky::feed::defs::PostView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub author: std::option::Option<atmo_core::AtIdentifier>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub domain: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mentions: std::option::Option<atmo_core::AtIdentifier>,
                    pub q: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort: std::option::Option<crate::app::bsky::feed::search_posts::Sort>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tag: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub until: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub url: std::option::Option<url::Url>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Sort {
                    #[serde(rename = "top")]
                    Top,
                    #[serde(rename = "latest")]
                    Latest,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod send_interactions {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub interactions: std::vec::Vec<crate::app::bsky::feed::defs::Interaction>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
            pub mod threadgate {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FollowingRule {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListRule {
                    pub list: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MentionRule {}
            }
        }
        pub mod graph {
            pub struct GetActorStarterPacks;
            impl atmo_core::xrpc::Request for GetActorStarterPacks {
                type Params = crate::app::bsky::graph::get_actor_starter_packs::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_actor_starter_packs::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getActorStarterPacks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetBlocks;
            impl atmo_core::xrpc::Request for GetBlocks {
                type Params = crate::app::bsky::graph::get_blocks::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_blocks::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getBlocks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFollowers;
            impl atmo_core::xrpc::Request for GetFollowers {
                type Params = crate::app::bsky::graph::get_followers::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_followers::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getFollowers"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetFollows;
            impl atmo_core::xrpc::Request for GetFollows {
                type Params = crate::app::bsky::graph::get_follows::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_follows::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getFollows"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetKnownFollowers;
            impl atmo_core::xrpc::Request for GetKnownFollowers {
                type Params = crate::app::bsky::graph::get_known_followers::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_known_followers::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getKnownFollowers"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetList;
            impl atmo_core::xrpc::Request for GetList {
                type Params = crate::app::bsky::graph::get_list::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_list::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getList"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetListBlocks;
            impl atmo_core::xrpc::Request for GetListBlocks {
                type Params = crate::app::bsky::graph::get_list_blocks::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_list_blocks::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getListBlocks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetListMutes;
            impl atmo_core::xrpc::Request for GetListMutes {
                type Params = crate::app::bsky::graph::get_list_mutes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_list_mutes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getListMutes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetLists;
            impl atmo_core::xrpc::Request for GetLists {
                type Params = crate::app::bsky::graph::get_lists::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_lists::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getLists"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetMutes;
            impl atmo_core::xrpc::Request for GetMutes {
                type Params = crate::app::bsky::graph::get_mutes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_mutes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getMutes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRelationships;
            impl atmo_core::xrpc::Request for GetRelationships {
                type Params = crate::app::bsky::graph::get_relationships::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_relationships::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getRelationships"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetStarterPack;
            impl atmo_core::xrpc::Request for GetStarterPack {
                type Params = crate::app::bsky::graph::get_starter_pack::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_starter_pack::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getStarterPack"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetStarterPacks;
            impl atmo_core::xrpc::Request for GetStarterPacks {
                type Params = crate::app::bsky::graph::get_starter_packs::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_starter_packs::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getStarterPacks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSuggestedFollowsByActor;
            impl atmo_core::xrpc::Request for GetSuggestedFollowsByActor {
                type Params = crate::app::bsky::graph::get_suggested_follows_by_actor::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::graph::get_suggested_follows_by_actor::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.getSuggestedFollowsByActor"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct MuteActor;
            impl atmo_core::xrpc::Request for MuteActor {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::mute_actor::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.muteActor"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct MuteActorList;
            impl atmo_core::xrpc::Request for MuteActorList {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::mute_actor_list::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.muteActorList"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct MuteThread;
            impl atmo_core::xrpc::Request for MuteThread {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::mute_thread::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.muteThread"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UnmuteActor;
            impl atmo_core::xrpc::Request for UnmuteActor {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::unmute_actor::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.unmuteActor"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UnmuteActorList;
            impl atmo_core::xrpc::Request for UnmuteActorList {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::unmute_actor_list::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.unmuteActorList"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UnmuteThread;
            impl atmo_core::xrpc::Request for UnmuteThread {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::graph::unmute_thread::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.unmuteThread"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListItemView {
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ListPurpose {
                    #[doc = "A list of actors to apply an aggregate moderation action (mute/block) on."]
                    #[serde(rename = "app.bsky.graph.defs#modlist")]
                    Modlist,
                    #[doc = "A list of actors used for curation purposes such as list feeds or interaction gating."]
                    #[serde(rename = "app.bsky.graph.defs#curatelist")]
                    Curatelist,
                    #[doc = "A list of actors used for only for reference purposes such as within a starter pack."]
                    #[serde(rename = "app.bsky.graph.defs#referencelist")]
                    Referencelist,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundActor {
                    pub actor: atmo_core::AtIdentifier,
                    pub not_found: bool,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Relationship {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackView {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feeds: std::option::Option<
                        std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                    >,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_week_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list: std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_items_sample: std::option::Option<
                        std::vec::Vec<crate::app::bsky::graph::defs::ListItemView>,
                    >,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackViewBasic {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_week_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_actor_starter_packs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_blocks {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blocks: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_followers {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_follows {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub follows: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_known_followers {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_list {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub items: std::vec::Vec<crate::app::bsky::graph::defs::ListItemView>,
                    pub list: crate::app::bsky::graph::defs::ListView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod get_list_blocks {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_list_mutes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_lists {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_mutes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub mutes: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod get_relationships {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub actor: std::option::Option<atmo_core::Did>,
                    pub relationships:
                        std::vec::Vec<crate::app::bsky::graph::get_relationships::Relationships>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub others: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Relationships {
                    NotFoundActor(crate::app::bsky::graph::defs::NotFoundActor),
                    Relationship(crate::app::bsky::graph::defs::Relationship),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod get_starter_pack {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub starter_pack: crate::app::bsky::graph::defs::StarterPackView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub starter_pack: atmo_core::AtUri,
                }
            }
            pub mod get_starter_packs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_suggested_follows_by_actor {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[doc = "If true, response has fallen-back to generic results, and is not scoped using relativeToDid"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub is_fallback: std::option::Option<bool>,
                    pub suggestions: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod mute_actor {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod mute_actor_list {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod mute_thread {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub root: atmo_core::AtUri,
                }
            }
            pub mod starterpack {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FeedItem {
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod unmute_actor {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod unmute_actor_list {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod unmute_thread {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub root: atmo_core::AtUri,
                }
            }
        }
        pub mod labeler {
            pub struct GetServices;
            impl atmo_core::xrpc::Request for GetServices {
                type Params = crate::app::bsky::labeler::get_services::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::labeler::get_services::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.labeler.getServices"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPolicies {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub label_value_definitions: std::option::Option<
                        std::vec::Vec<crate::com::atproto::label::defs::LabelValueDefinition>,
                    >,
                    pub label_values: std::vec::Vec<crate::com::atproto::label::defs::LabelValue>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerView {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewDetailed {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub policies: crate::app::bsky::labeler::defs::LabelerPolicies,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                }
            }
            pub mod get_services {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub views: std::vec::Vec<crate::app::bsky::labeler::get_services::Views>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub detailed: std::option::Option<bool>,
                    pub dids: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Views {
                    LabelerView(crate::app::bsky::labeler::defs::LabelerView),
                    LabelerViewDetailed(crate::app::bsky::labeler::defs::LabelerViewDetailed),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
        }
        pub mod notification {
            pub struct GetUnreadCount;
            impl atmo_core::xrpc::Request for GetUnreadCount {
                type Params = crate::app::bsky::notification::get_unread_count::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::notification::get_unread_count::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.notification.getUnreadCount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListNotifications;
            impl atmo_core::xrpc::Request for ListNotifications {
                type Params = crate::app::bsky::notification::list_notifications::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::notification::list_notifications::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.notification.listNotifications"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct PutPreferences;
            impl atmo_core::xrpc::Request for PutPreferences {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::notification::put_preferences::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.notification.putPreferences"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct RegisterPush;
            impl atmo_core::xrpc::Request for RegisterPush {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::notification::register_push::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.notification.registerPush"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateSeen;
            impl atmo_core::xrpc::Request for UpdateSeen {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::app::bsky::notification::update_seen::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.notification.updateSeen"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod get_unread_count {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub count: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub priority: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
            }
            pub mod list_notifications {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Notification {
                    pub author: crate::app::bsky::actor::defs::ProfileView,
                    pub cid: atmo_core::CidString,
                    pub indexed_at: atmo_core::DateTimeString,
                    pub is_read: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    pub reason: crate::app::bsky::notification::list_notifications::Reason,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason_subject: std::option::Option<atmo_core::AtUri>,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub notifications: std::vec::Vec<
                        crate::app::bsky::notification::list_notifications::Notification,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub priority: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub priority: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Reason {
                    #[serde(rename = "like")]
                    Like,
                    #[serde(rename = "repost")]
                    Repost,
                    #[serde(rename = "follow")]
                    Follow,
                    #[serde(rename = "mention")]
                    Mention,
                    #[serde(rename = "reply")]
                    Reply,
                    #[serde(rename = "quote")]
                    Quote,
                    #[serde(rename = "starterpack-joined")]
                    StarterpackJoined,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod put_preferences {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub priority: bool,
                }
            }
            pub mod register_push {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub app_id: std::string::String,
                    pub platform: crate::app::bsky::notification::register_push::Platform,
                    pub service_did: atmo_core::Did,
                    pub token: std::string::String,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Platform {
                    #[serde(rename = "ios")]
                    Ios,
                    #[serde(rename = "android")]
                    Android,
                    #[serde(rename = "web")]
                    Web,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod update_seen {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub seen_at: atmo_core::DateTimeString,
                }
            }
        }
        pub mod richtext {
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct Facet {
                pub features: std::vec::Vec<crate::app::bsky::richtext::facet::Features>,
                pub index: crate::app::bsky::richtext::facet::ByteSlice,
            }
            pub mod facet {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ByteSlice {
                    pub byte_end: i64,
                    pub byte_start: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Features {
                    Link(crate::app::bsky::richtext::facet::Link),
                    Mention(crate::app::bsky::richtext::facet::Mention),
                    Tag(crate::app::bsky::richtext::facet::Tag),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Link {
                    pub uri: url::Url,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Mention {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Tag {
                    pub tag: std::string::String,
                }
            }
        }
        pub mod unspecced {
            pub struct GetPopularFeedGenerators;
            impl atmo_core::xrpc::Request for GetPopularFeedGenerators {
                type Params = crate::app::bsky::unspecced::get_popular_feed_generators::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::unspecced::get_popular_feed_generators::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.getPopularFeedGenerators"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSuggestionsSkeleton;
            impl atmo_core::xrpc::Request for GetSuggestionsSkeleton {
                type Params = crate::app::bsky::unspecced::get_suggestions_skeleton::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::unspecced::get_suggestions_skeleton::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.getSuggestionsSkeleton"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetTaggedSuggestions;
            impl atmo_core::xrpc::Request for GetTaggedSuggestions {
                type Params = crate::app::bsky::unspecced::get_tagged_suggestions::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::unspecced::get_tagged_suggestions::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.getTaggedSuggestions"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchActorsSkeleton;
            impl atmo_core::xrpc::Request for SearchActorsSkeleton {
                type Params = crate::app::bsky::unspecced::search_actors_skeleton::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::unspecced::search_actors_skeleton::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.searchActorsSkeleton"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchPostsSkeleton;
            impl atmo_core::xrpc::Request for SearchPostsSkeleton {
                type Params = crate::app::bsky::unspecced::search_posts_skeleton::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::unspecced::search_posts_skeleton::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.searchPostsSkeleton"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchActor {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchPost {
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_popular_feed_generators {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub query: std::option::Option<std::string::String>,
                }
            }
            pub mod get_suggestions_skeleton {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors:
                        std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchActor>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub relative_to_did: std::option::Option<atmo_core::Did>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub relative_to_did: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
            }
            pub mod get_tagged_suggestions {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub suggestions: std::vec::Vec<
                        crate::app::bsky::unspecced::get_tagged_suggestions::Suggestion,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {}
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SubjectType {
                    #[serde(rename = "actor")]
                    Actor,
                    #[serde(rename = "feed")]
                    Feed,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Suggestion {
                    pub subject: url::Url,
                    pub subject_type:
                        crate::app::bsky::unspecced::get_tagged_suggestions::SubjectType,
                    pub tag: std::string::String,
                }
            }
            pub mod search_actors_skeleton {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors:
                        std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchActor>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub q: std::string::String,
                    #[doc = "If true, acts as fast/simple 'typeahead' query."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub typeahead: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
            }
            pub mod search_posts_skeleton {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                    pub posts: std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchPost>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub author: std::option::Option<atmo_core::AtIdentifier>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub domain: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mentions: std::option::Option<atmo_core::AtIdentifier>,
                    pub q: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort: std::option::Option<
                        crate::app::bsky::unspecced::search_posts_skeleton::Sort,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tag: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub until: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub url: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Sort {
                    #[serde(rename = "top")]
                    Top,
                    #[serde(rename = "latest")]
                    Latest,
                    #[serde(untagged)]
                    Other(String),
                }
            }
        }
        pub mod video {
            pub struct GetJobStatus;
            impl atmo_core::xrpc::Request for GetJobStatus {
                type Params = crate::app::bsky::video::get_job_status::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::video::get_job_status::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.video.getJobStatus"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetUploadLimits;
            impl atmo_core::xrpc::Request for GetUploadLimits {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::video::get_upload_limits::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.video.getUploadLimits"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UploadVideo;
            impl atmo_core::xrpc::Request for UploadVideo {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::app::bsky::video::upload_video::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.video.uploadVideo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct JobStatus {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blob: std::option::Option<atmo_core::Blob>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub error: std::option::Option<std::string::String>,
                    pub job_id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub progress: std::option::Option<i64>,
                    pub state: crate::app::bsky::video::defs::State,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum State {
                    #[serde(rename = "JOB_STATE_COMPLETED")]
                    JobStateCompleted,
                    #[serde(rename = "JOB_STATE_FAILED")]
                    JobStateFailed,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod get_job_status {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub job_status: crate::app::bsky::video::defs::JobStatus,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub job_id: std::string::String,
                }
            }
            pub mod get_upload_limits {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub can_upload: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub error: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub remaining_daily_bytes: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub remaining_daily_videos: std::option::Option<i64>,
                }
            }
            pub mod upload_video {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub job_status: crate::app::bsky::video::defs::JobStatus,
                }
            }
        }
    }
}
pub mod chat {
    pub mod bsky {
        pub mod actor {
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::actor::delete_account::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.actor.deleteAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ExportAccountData;
            impl atmo_core::xrpc::Request for ExportAccountData {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.actor.exportAccountData"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/jsonl"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[doc = "Set to true when the actor cannot actively participate in converations"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat_disabled: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
            }
            pub mod delete_account {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
        }
        pub mod convo {
            pub struct DeleteMessageForSelf;
            impl atmo_core::xrpc::Request for DeleteMessageForSelf {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::delete_message_for_self::Input;
                type Output = crate::chat::bsky::convo::defs::DeletedMessageView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.deleteMessageForSelf"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetConvo;
            impl atmo_core::xrpc::Request for GetConvo {
                type Params = crate::chat::bsky::convo::get_convo::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::convo::get_convo::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.getConvo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetConvoForMembers;
            impl atmo_core::xrpc::Request for GetConvoForMembers {
                type Params = crate::chat::bsky::convo::get_convo_for_members::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::convo::get_convo_for_members::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.getConvoForMembers"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetLog;
            impl atmo_core::xrpc::Request for GetLog {
                type Params = crate::chat::bsky::convo::get_log::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::convo::get_log::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.getLog"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetMessages;
            impl atmo_core::xrpc::Request for GetMessages {
                type Params = crate::chat::bsky::convo::get_messages::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::convo::get_messages::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.getMessages"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct LeaveConvo;
            impl atmo_core::xrpc::Request for LeaveConvo {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::leave_convo::Input;
                type Output = crate::chat::bsky::convo::leave_convo::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.leaveConvo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListConvos;
            impl atmo_core::xrpc::Request for ListConvos {
                type Params = crate::chat::bsky::convo::list_convos::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::convo::list_convos::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.listConvos"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct MuteConvo;
            impl atmo_core::xrpc::Request for MuteConvo {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::mute_convo::Input;
                type Output = crate::chat::bsky::convo::mute_convo::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.muteConvo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SendMessage;
            impl atmo_core::xrpc::Request for SendMessage {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::send_message::Input;
                type Output = crate::chat::bsky::convo::defs::MessageView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.sendMessage"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SendMessageBatch;
            impl atmo_core::xrpc::Request for SendMessageBatch {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::send_message_batch::Input;
                type Output = crate::chat::bsky::convo::send_message_batch::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.sendMessageBatch"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UnmuteConvo;
            impl atmo_core::xrpc::Request for UnmuteConvo {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::unmute_convo::Input;
                type Output = crate::chat::bsky::convo::unmute_convo::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.unmuteConvo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpdateRead;
            impl atmo_core::xrpc::Request for UpdateRead {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::convo::update_read::Input;
                type Output = crate::chat::bsky::convo::update_read::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.convo.updateRead"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ConvoView {
                    pub id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_message:
                        std::option::Option<crate::chat::bsky::convo::defs::LastMessage>,
                    pub members: std::vec::Vec<crate::chat::bsky::actor::defs::ProfileViewBasic>,
                    pub muted: bool,
                    pub rev: std::string::String,
                    pub unread_count: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct DeletedMessageView {
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    pub sent_at: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Embed {
                    Record(crate::app::bsky::embed::Record),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum LastMessage {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LogBeginConvo {
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LogCreateMessage {
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::Message,
                    pub rev: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LogDeleteMessage {
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::LogDeleteMessageMessage,
                    pub rev: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum LogDeleteMessageMessage {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LogLeaveConvo {
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Message {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageInput {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed: std::option::Option<crate::chat::bsky::convo::defs::Embed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub text: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageRef {
                    pub convo_id: std::string::String,
                    pub did: atmo_core::Did,
                    pub message_id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed:
                        std::option::Option<crate::chat::bsky::convo::defs::MessageViewEmbed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    pub sent_at: atmo_core::DateTimeString,
                    pub text: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum MessageViewEmbed {
                    View(crate::app::bsky::embed::record::View),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageViewSender {
                    pub did: atmo_core::Did,
                }
            }
            pub mod delete_message_for_self {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                    pub message_id: std::string::String,
                }
            }
            pub mod get_convo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub convo_id: std::string::String,
                }
            }
            pub mod get_convo_for_members {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub members: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_log {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Logs {
                    LogBeginConvo(crate::chat::bsky::convo::defs::LogBeginConvo),
                    LogCreateMessage(crate::chat::bsky::convo::defs::LogCreateMessage),
                    LogDeleteMessage(crate::chat::bsky::convo::defs::LogDeleteMessage),
                    LogLeaveConvo(crate::chat::bsky::convo::defs::LogLeaveConvo),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub logs: std::vec::Vec<crate::chat::bsky::convo::get_log::Logs>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
            }
            pub mod get_messages {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Messages {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub messages: std::vec::Vec<crate::chat::bsky::convo::get_messages::Messages>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub convo_id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod leave_convo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
            }
            pub mod list_convos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convos: std::vec::Vec<crate::chat::bsky::convo::defs::ConvoView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod mute_convo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
            pub mod send_message {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::MessageInput,
                }
            }
            pub mod send_message_batch {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BatchItem {
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::MessageInput,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub items:
                        std::vec::Vec<crate::chat::bsky::convo::send_message_batch::BatchItem>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub items: std::vec::Vec<crate::chat::bsky::convo::defs::MessageView>,
                }
            }
            pub mod unmute_convo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
            pub mod update_read {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub convo_id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message_id: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
        }
        pub mod moderation {
            pub struct GetActorMetadata;
            impl atmo_core::xrpc::Request for GetActorMetadata {
                type Params = crate::chat::bsky::moderation::get_actor_metadata::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::moderation::get_actor_metadata::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.moderation.getActorMetadata"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetMessageContext;
            impl atmo_core::xrpc::Request for GetMessageContext {
                type Params = crate::chat::bsky::moderation::get_message_context::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::chat::bsky::moderation::get_message_context::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.moderation.getMessageContext"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpdateActorAccess;
            impl atmo_core::xrpc::Request for UpdateActorAccess {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::chat::bsky::moderation::update_actor_access::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "chat.bsky.moderation.updateActorAccess"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod get_actor_metadata {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Metadata {
                    pub convos: i64,
                    pub convos_started: i64,
                    pub messages_received: i64,
                    pub messages_sent: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub all: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                    pub day: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                    pub month: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::Did,
                }
            }
            pub mod get_message_context {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Messages {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub messages:
                        std::vec::Vec<crate::chat::bsky::moderation::get_message_context::Messages>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub after: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub before: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub convo_id: std::option::Option<std::string::String>,
                    pub message_id: std::string::String,
                }
            }
            pub mod update_actor_access {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::Did,
                    pub allow_access: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ref_: std::option::Option<std::string::String>,
                }
            }
        }
    }
}
pub mod com {
    pub mod atproto {
        pub mod admin {
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::delete_account::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.deleteAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DisableAccountInvites;
            impl atmo_core::xrpc::Request for DisableAccountInvites {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::disable_account_invites::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.disableAccountInvites"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DisableInviteCodes;
            impl atmo_core::xrpc::Request for DisableInviteCodes {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::disable_invite_codes::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.disableInviteCodes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct EnableAccountInvites;
            impl atmo_core::xrpc::Request for EnableAccountInvites {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::enable_account_invites::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.enableAccountInvites"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct GetAccountInfo;
            impl atmo_core::xrpc::Request for GetAccountInfo {
                type Params = crate::com::atproto::admin::get_account_info::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::admin::defs::AccountView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.getAccountInfo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetAccountInfos;
            impl atmo_core::xrpc::Request for GetAccountInfos {
                type Params = crate::com::atproto::admin::get_account_infos::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::admin::get_account_infos::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.getAccountInfos"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetInviteCodes;
            impl atmo_core::xrpc::Request for GetInviteCodes {
                type Params = crate::com::atproto::admin::get_invite_codes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::admin::get_invite_codes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.getInviteCodes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSubjectStatus;
            impl atmo_core::xrpc::Request for GetSubjectStatus {
                type Params = crate::com::atproto::admin::get_subject_status::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::admin::get_subject_status::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.getSubjectStatus"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchAccounts;
            impl atmo_core::xrpc::Request for SearchAccounts {
                type Params = crate::com::atproto::admin::search_accounts::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::admin::search_accounts::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.searchAccounts"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SendEmail;
            impl atmo_core::xrpc::Request for SendEmail {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::send_email::Input;
                type Output = crate::com::atproto::admin::send_email::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.sendEmail"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpdateAccountEmail;
            impl atmo_core::xrpc::Request for UpdateAccountEmail {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::update_account_email::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.updateAccountEmail"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateAccountHandle;
            impl atmo_core::xrpc::Request for UpdateAccountHandle {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::update_account_handle::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.updateAccountHandle"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateAccountPassword;
            impl atmo_core::xrpc::Request for UpdateAccountPassword {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::update_account_password::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.updateAccountPassword"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateSubjectStatus;
            impl atmo_core::xrpc::Request for UpdateSubjectStatus {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::admin::update_subject_status::Input;
                type Output = crate::com::atproto::admin::update_subject_status::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.admin.updateSubjectStatus"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AccountView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo_core::DateTimeString>,
                    pub handle: atmo_core::Handle,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites: std::option::Option<
                        std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub related_records: std::option::Option<std::vec::Vec<atmo_core::Unknown>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoBlobRef {
                    pub cid: atmo_core::CidString,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub record_uri: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoRef {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StatusAttr {
                    pub applied: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ref_: std::option::Option<std::string::String>,
                }
            }
            pub mod delete_account {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                }
            }
            pub mod disable_account_invites {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub note: std::option::Option<std::string::String>,
                }
            }
            pub mod disable_invite_codes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub accounts: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub codes: std::option::Option<std::vec::Vec<std::string::String>>,
                }
            }
            pub mod enable_account_invites {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub note: std::option::Option<std::string::String>,
                }
            }
            pub mod get_account_info {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_account_infos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub infos: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_invite_codes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort:
                        std::option::Option<crate::com::atproto::admin::get_invite_codes::Sort>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Sort {
                    #[serde(rename = "recent")]
                    Recent,
                    #[serde(rename = "usage")]
                    Usage,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod get_subject_status {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated:
                        std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                    pub subject: crate::com::atproto::admin::get_subject_status::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blob: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub uri: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod search_accounts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod send_email {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub content: std::string::String,
                    pub recipient_did: atmo_core::Did,
                    pub sender_did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub sent: bool,
                }
            }
            pub mod update_account_email {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::AtIdentifier,
                    pub email: std::string::String,
                }
            }
            pub mod update_account_handle {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub handle: atmo_core::Handle,
                }
            }
            pub mod update_account_password {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub password: std::string::String,
                }
            }
            pub mod update_subject_status {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated:
                        std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                    pub subject: crate::com::atproto::admin::update_subject_status::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub subject: crate::com::atproto::admin::update_subject_status::OutputSubject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum OutputSubject {
                    RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
        }
        pub mod identity {
            pub struct GetRecommendedDidCredentials;
            impl atmo_core::xrpc::Request for GetRecommendedDidCredentials {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output =
                    crate::com::atproto::identity::get_recommended_did_credentials::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.getRecommendedDidCredentials"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct RequestPlcOperationSignature;
            impl atmo_core::xrpc::Request for RequestPlcOperationSignature {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.requestPlcOperationSignature"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct ResolveHandle;
            impl atmo_core::xrpc::Request for ResolveHandle {
                type Params = crate::com::atproto::identity::resolve_handle::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::identity::resolve_handle::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.resolveHandle"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SignPlcOperation;
            impl atmo_core::xrpc::Request for SignPlcOperation {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::identity::sign_plc_operation::Input;
                type Output = crate::com::atproto::identity::sign_plc_operation::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.signPlcOperation"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SubmitPlcOperation;
            impl atmo_core::xrpc::Request for SubmitPlcOperation {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::identity::submit_plc_operation::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.submitPlcOperation"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateHandle;
            impl atmo_core::xrpc::Request for UpdateHandle {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::identity::update_handle::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.identity.updateHandle"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod get_recommended_did_credentials {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub also_known_as: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rotation_keys: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub services: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_methods: std::option::Option<atmo_core::Unknown>,
                }
            }
            pub mod resolve_handle {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub handle: atmo_core::Handle,
                }
            }
            pub mod sign_plc_operation {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub also_known_as: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rotation_keys: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub services: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub token: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_methods: std::option::Option<atmo_core::Unknown>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub operation: atmo_core::Unknown,
                }
            }
            pub mod submit_plc_operation {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub operation: atmo_core::Unknown,
                }
            }
            pub mod update_handle {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub handle: atmo_core::Handle,
                }
            }
        }
        pub mod label {
            pub struct QueryLabels;
            impl atmo_core::xrpc::Request for QueryLabels {
                type Params = crate::com::atproto::label::query_labels::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::label::query_labels::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.label.queryLabels"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Blurs {
                    #[serde(rename = "content")]
                    Content,
                    #[serde(rename = "media")]
                    Media,
                    #[serde(rename = "none")]
                    None,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum DefaultSetting {
                    #[serde(rename = "ignore")]
                    Ignore,
                    #[serde(rename = "warn")]
                    Warn,
                    #[serde(rename = "hide")]
                    Hide,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Label {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub cts: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exp: std::option::Option<atmo_core::DateTimeString>,
                    #[doc = "If true, this is a negation label, overwriting a previous label."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub neg: std::option::Option<bool>,
                    #[doc = "Signature of dag-cbor encoded label."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    #[serde(with = "atmo_core::bytes::serde::option")]
                    pub sig: std::option::Option<bytes::Bytes>,
                    pub src: atmo_core::Did,
                    pub uri: url::Url,
                    pub val: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ver: std::option::Option<i64>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum LabelValue {
                    #[serde(rename = "!hide")]
                    Hide,
                    #[serde(rename = "!no-promote")]
                    NoPromote,
                    #[serde(rename = "!warn")]
                    Warn,
                    #[serde(rename = "!no-unauthenticated")]
                    NoUnauthenticated,
                    #[serde(rename = "dmca-violation")]
                    DmcaViolation,
                    #[serde(rename = "doxxing")]
                    Doxxing,
                    #[serde(rename = "porn")]
                    Porn,
                    #[serde(rename = "sexual")]
                    Sexual,
                    #[serde(rename = "nudity")]
                    Nudity,
                    #[serde(rename = "nsfl")]
                    Nsfl,
                    #[serde(rename = "gore")]
                    Gore,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelValueDefinition {
                    #[doc = "Does the user need to have adult content enabled in order to configure this label?"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub adult_only: std::option::Option<bool>,
                    pub blurs: crate::com::atproto::label::defs::Blurs,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub default_setting:
                        std::option::Option<crate::com::atproto::label::defs::DefaultSetting>,
                    pub identifier: std::string::String,
                    pub locales: std::vec::Vec<
                        crate::com::atproto::label::defs::LabelValueDefinitionStrings,
                    >,
                    pub severity: crate::com::atproto::label::defs::Severity,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelValueDefinitionStrings {
                    pub description: std::string::String,
                    pub lang: std::string::String,
                    pub name: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SelfLabel {
                    pub val: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SelfLabels {
                    pub values: std::vec::Vec<crate::com::atproto::label::defs::SelfLabel>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Severity {
                    #[serde(rename = "inform")]
                    Inform,
                    #[serde(rename = "alert")]
                    Alert,
                    #[serde(rename = "none")]
                    None,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod query_labels {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sources: std::option::Option<std::vec::Vec<std::string::String>>,
                    pub uri_patterns: std::vec::Vec<std::string::String>,
                }
            }
            pub mod subscribe_labels {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::label::subscribe_labels::Name,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Labels {
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                    pub seq: i64,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Name {
                    #[serde(rename = "OutdatedCursor")]
                    OutdatedCursor,
                    #[serde(untagged)]
                    Other(String),
                }
            }
        }
        pub mod moderation {
            pub struct CreateReport;
            impl atmo_core::xrpc::Request for CreateReport {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::moderation::create_report::Input;
                type Output = crate::com::atproto::moderation::create_report::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.moderation.createReport"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod create_report {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<std::string::String>,
                    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
                    pub subject: crate::com::atproto::moderation::create_report::Subject,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub created_at: atmo_core::DateTimeString,
                    pub id: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<std::string::String>,
                    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
                    pub reported_by: atmo_core::Did,
                    pub subject: crate::com::atproto::moderation::create_report::OutputSubject,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum OutputSubject {
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod defs {
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ReasonType {
                    #[doc = "Spam: frequent unwanted promotion, replies, mentions"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonSpam")]
                    ReasonSpam,
                    #[doc = "Direct violation of server rules, laws, terms of service"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonViolation")]
                    ReasonViolation,
                    #[doc = "Misleading identity, affiliation, or content"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonMisleading")]
                    ReasonMisleading,
                    #[doc = "Unwanted or mislabeled sexual content"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonSexual")]
                    ReasonSexual,
                    #[doc = "Rude, harassing, explicit, or otherwise unwelcoming behavior"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonRude")]
                    ReasonRude,
                    #[doc = "Other: reports not falling under another report category"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonOther")]
                    ReasonOther,
                    #[doc = "Appeal: appeal a previously taken moderation action"]
                    #[serde(rename = "com.atproto.moderation.defs#reasonAppeal")]
                    ReasonAppeal,
                    #[serde(untagged)]
                    Other(String),
                }
            }
        }
        pub mod repo {
            pub struct ApplyWrites;
            impl atmo_core::xrpc::Request for ApplyWrites {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::repo::apply_writes::Input;
                type Output = crate::com::atproto::repo::apply_writes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.applyWrites"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct CreateRecord;
            impl atmo_core::xrpc::Request for CreateRecord {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::repo::create_record::Input;
                type Output = crate::com::atproto::repo::create_record::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.createRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DeleteRecord;
            impl atmo_core::xrpc::Request for DeleteRecord {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::repo::delete_record::Input;
                type Output = crate::com::atproto::repo::delete_record::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.deleteRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DescribeRepo;
            impl atmo_core::xrpc::Request for DescribeRepo {
                type Params = crate::com::atproto::repo::describe_repo::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::repo::describe_repo::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.describeRepo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::com::atproto::repo::get_record::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::repo::get_record::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.getRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ImportRepo;
            impl atmo_core::xrpc::Request for ImportRepo {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.importRepo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct ListMissingBlobs;
            impl atmo_core::xrpc::Request for ListMissingBlobs {
                type Params = crate::com::atproto::repo::list_missing_blobs::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::repo::list_missing_blobs::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.listMissingBlobs"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListRecords;
            impl atmo_core::xrpc::Request for ListRecords {
                type Params = crate::com::atproto::repo::list_records::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::repo::list_records::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.listRecords"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct PutRecord;
            impl atmo_core::xrpc::Request for PutRecord {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::repo::put_record::Input;
                type Output = crate::com::atproto::repo::put_record::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.putRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            #[derive(serde :: Deserialize, serde :: Serialize)]
            pub struct StrongRef {
                pub cid: atmo_core::CidString,
                pub uri: atmo_core::AtUri,
            }
            pub struct UploadBlob;
            impl atmo_core::xrpc::Request for UploadBlob {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::repo::upload_blob::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.repo.uploadBlob"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod apply_writes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Create {
                    pub collection: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey: std::option::Option<std::string::String>,
                    pub value: atmo_core::Unknown,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct CreateResult {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::apply_writes::ValidationStatus,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Delete {
                    pub collection: atmo_core::Nsid,
                    pub rkey: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct DeleteResult {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub repo: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[doc = "Can be set to 'false' to skip Lexicon schema validation of record data across all operations, 'true' to require it, or leave unset to validate only for known Lexicons."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                    pub writes: std::vec::Vec<crate::com::atproto::repo::apply_writes::Writes>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub results: std::option::Option<
                        std::vec::Vec<crate::com::atproto::repo::apply_writes::Results>,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Results {
                    CreateResult(crate::com::atproto::repo::apply_writes::CreateResult),
                    DeleteResult(crate::com::atproto::repo::apply_writes::DeleteResult),
                    UpdateResult(crate::com::atproto::repo::apply_writes::UpdateResult),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Update {
                    pub collection: atmo_core::Nsid,
                    pub rkey: std::string::String,
                    pub value: atmo_core::Unknown,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct UpdateResult {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::apply_writes::ValidationStatus,
                    >,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ValidationStatus {
                    #[serde(rename = "valid")]
                    Valid,
                    #[serde(rename = "unknown")]
                    Unknown,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Writes {
                    Create(crate::com::atproto::repo::apply_writes::Create),
                    Delete(crate::com::atproto::repo::apply_writes::Delete),
                    Update(crate::com::atproto::repo::apply_writes::Update),
                }
            }
            pub mod create_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub record: atmo_core::Unknown,
                    pub repo: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[doc = "Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::create_record::ValidationStatus,
                    >,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ValidationStatus {
                    #[serde(rename = "valid")]
                    Valid,
                    #[serde(rename = "unknown")]
                    Unknown,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct CommitMeta {
                    pub cid: atmo_core::CidString,
                    pub rev: std::string::String,
                }
            }
            pub mod delete_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub repo: atmo_core::AtIdentifier,
                    pub rkey: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_record: std::option::Option<atmo_core::CidString>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                }
            }
            pub mod describe_repo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub collections: std::vec::Vec<std::string::String>,
                    pub did: atmo_core::Did,
                    pub did_doc: atmo_core::Unknown,
                    pub handle: atmo_core::Handle,
                    #[doc = "Indicates if handle is currently valid (resolves bi-directionally)"]
                    pub handle_is_correct: bool,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub repo: atmo_core::AtIdentifier,
                }
            }
            pub mod get_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub collection: atmo_core::Nsid,
                    pub repo: atmo_core::AtIdentifier,
                    pub rkey: std::string::String,
                }
            }
            pub mod list_missing_blobs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blobs:
                        std::vec::Vec<crate::com::atproto::repo::list_missing_blobs::RecordBlob>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordBlob {
                    pub cid: atmo_core::CidString,
                    pub record_uri: atmo_core::AtUri,
                }
            }
            pub mod list_records {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub records: std::vec::Vec<crate::com::atproto::repo::list_records::Record>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub collection: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub repo: atmo_core::AtIdentifier,
                    #[doc = "Flag to reverse the order of the returned records."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reverse: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey_end: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey_start: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Record {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
            }
            pub mod put_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub record: atmo_core::Unknown,
                    pub repo: atmo_core::AtIdentifier,
                    pub rkey: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_record: std::option::Option<atmo_core::Nullable<atmo_core::CidString>>,
                    #[doc = "Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::put_record::ValidationStatus,
                    >,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum ValidationStatus {
                    #[serde(rename = "valid")]
                    Valid,
                    #[serde(rename = "unknown")]
                    Unknown,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod upload_blob {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blob: atmo_core::Blob,
                }
            }
        }
        pub mod server {
            pub struct ActivateAccount;
            impl atmo_core::xrpc::Request for ActivateAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.activateAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct CheckAccountStatus;
            impl atmo_core::xrpc::Request for CheckAccountStatus {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::check_account_status::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.checkAccountStatus"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ConfirmEmail;
            impl atmo_core::xrpc::Request for ConfirmEmail {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::confirm_email::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.confirmEmail"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct CreateAccount;
            impl atmo_core::xrpc::Request for CreateAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::create_account::Input;
                type Output = crate::com::atproto::server::create_account::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.createAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct CreateAppPassword;
            impl atmo_core::xrpc::Request for CreateAppPassword {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::create_app_password::Input;
                type Output = crate::com::atproto::server::create_app_password::AppPassword;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.createAppPassword"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct CreateInviteCode;
            impl atmo_core::xrpc::Request for CreateInviteCode {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::create_invite_code::Input;
                type Output = crate::com::atproto::server::create_invite_code::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.createInviteCode"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct CreateInviteCodes;
            impl atmo_core::xrpc::Request for CreateInviteCodes {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::create_invite_codes::Input;
                type Output = crate::com::atproto::server::create_invite_codes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.createInviteCodes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct CreateSession;
            impl atmo_core::xrpc::Request for CreateSession {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::create_session::Input;
                type Output = crate::com::atproto::server::create_session::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.createSession"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DeactivateAccount;
            impl atmo_core::xrpc::Request for DeactivateAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::deactivate_account::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.deactivateAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::delete_account::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.deleteAccount"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DeleteSession;
            impl atmo_core::xrpc::Request for DeleteSession {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.deleteSession"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DescribeServer;
            impl atmo_core::xrpc::Request for DescribeServer {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::describe_server::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.describeServer"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetAccountInviteCodes;
            impl atmo_core::xrpc::Request for GetAccountInviteCodes {
                type Params = crate::com::atproto::server::get_account_invite_codes::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::get_account_invite_codes::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.getAccountInviteCodes"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetServiceAuth;
            impl atmo_core::xrpc::Request for GetServiceAuth {
                type Params = crate::com::atproto::server::get_service_auth::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::get_service_auth::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.getServiceAuth"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetSession;
            impl atmo_core::xrpc::Request for GetSession {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::get_session::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.getSession"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListAppPasswords;
            impl atmo_core::xrpc::Request for ListAppPasswords {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::list_app_passwords::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.listAppPasswords"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct RefreshSession;
            impl atmo_core::xrpc::Request for RefreshSession {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::refresh_session::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.refreshSession"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct RequestAccountDelete;
            impl atmo_core::xrpc::Request for RequestAccountDelete {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.requestAccountDelete"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct RequestEmailConfirmation;
            impl atmo_core::xrpc::Request for RequestEmailConfirmation {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.requestEmailConfirmation"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct RequestEmailUpdate;
            impl atmo_core::xrpc::Request for RequestEmailUpdate {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::server::request_email_update::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.requestEmailUpdate"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct RequestPasswordReset;
            impl atmo_core::xrpc::Request for RequestPasswordReset {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::request_password_reset::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.requestPasswordReset"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct ReserveSigningKey;
            impl atmo_core::xrpc::Request for ReserveSigningKey {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::reserve_signing_key::Input;
                type Output = crate::com::atproto::server::reserve_signing_key::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.reserveSigningKey"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ResetPassword;
            impl atmo_core::xrpc::Request for ResetPassword {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::reset_password::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.resetPassword"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct RevokeAppPassword;
            impl atmo_core::xrpc::Request for RevokeAppPassword {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::revoke_app_password::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.revokeAppPassword"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct UpdateEmail;
            impl atmo_core::xrpc::Request for UpdateEmail {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::server::update_email::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.server.updateEmail"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod check_account_status {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub activated: bool,
                    pub expected_blobs: i64,
                    pub imported_blobs: i64,
                    pub indexed_records: i64,
                    pub private_state_values: i64,
                    pub repo_blocks: i64,
                    pub repo_commit: atmo_core::CidString,
                    pub repo_rev: std::string::String,
                    pub valid_did: bool,
                }
            }
            pub mod confirm_email {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod create_account {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_code: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub password: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub plc_op: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub recovery_key: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_code: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_phone: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub access_jwt: std::string::String,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    pub handle: atmo_core::Handle,
                    pub refresh_jwt: std::string::String,
                }
            }
            pub mod create_app_password {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    pub created_at: atmo_core::DateTimeString,
                    pub name: std::string::String,
                    pub password: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    #[doc = "If an app password has 'privileged' access to possibly sensitive account state. Meant for use with trusted clients."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
            }
            pub mod create_invite_code {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub for_account: std::option::Option<atmo_core::Did>,
                    pub use_count: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub code: std::string::String,
                }
            }
            pub mod create_invite_codes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AccountCodes {
                    pub account: std::string::String,
                    pub codes: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub code_count: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub for_accounts: std::option::Option<std::vec::Vec<std::string::String>>,
                    pub use_count: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<
                        crate::com::atproto::server::create_invite_codes::AccountCodes,
                    >,
                }
            }
            pub mod create_session {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub auth_factor_token: std::option::Option<std::string::String>,
                    pub identifier: std::string::String,
                    pub password: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub access_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed: std::option::Option<bool>,
                    pub handle: atmo_core::Handle,
                    pub refresh_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::server::create_session::Status>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod deactivate_account {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub delete_after: std::option::Option<atmo_core::DateTimeString>,
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCode {
                    pub available: i64,
                    pub code: std::string::String,
                    pub created_at: atmo_core::DateTimeString,
                    pub created_by: std::string::String,
                    pub disabled: bool,
                    pub for_account: std::string::String,
                    pub uses: std::vec::Vec<crate::com::atproto::server::defs::InviteCodeUse>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCodeUse {
                    pub used_at: atmo_core::DateTimeString,
                    pub used_by: atmo_core::Did,
                }
            }
            pub mod delete_account {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub password: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod describe_server {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Contact {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<url::Url>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub available_user_domains: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub contact:
                        std::option::Option<crate::com::atproto::server::describe_server::Contact>,
                    pub did: atmo_core::Did,
                    #[doc = "If true, an invite code must be supplied to create an account on this instance."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_code_required: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub links:
                        std::option::Option<crate::com::atproto::server::describe_server::Links>,
                    #[doc = "If true, a phone verification token must be supplied to create an account on this instance."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub phone_verification_required: std::option::Option<bool>,
                }
            }
            pub mod get_account_invite_codes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[doc = "Controls whether any new 'earned' but not 'created' invites should be created."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub create_available: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_used: std::option::Option<bool>,
                }
            }
            pub mod get_service_auth {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub token: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub aud: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exp: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lxm: std::option::Option<atmo_core::Nsid>,
                }
            }
            pub mod get_session {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed: std::option::Option<bool>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::server::get_session::Status>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod list_app_passwords {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    pub created_at: atmo_core::DateTimeString,
                    pub name: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub passwords:
                        std::vec::Vec<crate::com::atproto::server::list_app_passwords::AppPassword>,
                }
            }
            pub mod refresh_session {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub access_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    pub handle: atmo_core::Handle,
                    pub refresh_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::server::refresh_session::Status>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod request_email_update {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub token_required: bool,
                }
            }
            pub mod request_password_reset {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                }
            }
            pub mod reserve_signing_key {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did: std::option::Option<atmo_core::Did>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub signing_key: std::string::String,
                }
            }
            pub mod reset_password {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub password: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod revoke_app_password {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                }
            }
            pub mod update_email {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub token: std::option::Option<std::string::String>,
                }
            }
        }
        pub mod sync {
            pub struct GetBlob;
            impl atmo_core::xrpc::Request for GetBlob {
                type Params = crate::com::atproto::sync::get_blob::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getBlob"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct GetBlocks;
            impl atmo_core::xrpc::Request for GetBlocks {
                type Params = crate::com::atproto::sync::get_blocks::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getBlocks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/vnd.ipld.car"
                }
            }
            pub struct GetCheckout;
            impl atmo_core::xrpc::Request for GetCheckout {
                type Params = crate::com::atproto::sync::get_checkout::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getCheckout"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/vnd.ipld.car"
                }
            }
            pub struct GetHead;
            impl atmo_core::xrpc::Request for GetHead {
                type Params = crate::com::atproto::sync::get_head::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::sync::get_head::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getHead"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetLatestCommit;
            impl atmo_core::xrpc::Request for GetLatestCommit {
                type Params = crate::com::atproto::sync::get_latest_commit::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::sync::get_latest_commit::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getLatestCommit"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::com::atproto::sync::get_record::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/vnd.ipld.car"
                }
            }
            pub struct GetRepo;
            impl atmo_core::xrpc::Request for GetRepo {
                type Params = crate::com::atproto::sync::get_repo::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getRepo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/vnd.ipld.car"
                }
            }
            pub struct GetRepoStatus;
            impl atmo_core::xrpc::Request for GetRepoStatus {
                type Params = crate::com::atproto::sync::get_repo_status::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::sync::get_repo_status::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.getRepoStatus"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListBlobs;
            impl atmo_core::xrpc::Request for ListBlobs {
                type Params = crate::com::atproto::sync::list_blobs::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::sync::list_blobs::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.listBlobs"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct ListRepos;
            impl atmo_core::xrpc::Request for ListRepos {
                type Params = crate::com::atproto::sync::list_repos::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::sync::list_repos::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.listRepos"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct NotifyOfUpdate;
            impl atmo_core::xrpc::Request for NotifyOfUpdate {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::sync::notify_of_update::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.notifyOfUpdate"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct RequestCrawl;
            impl atmo_core::xrpc::Request for RequestCrawl {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::sync::request_crawl::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.sync.requestCrawl"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod get_blob {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub cid: atmo_core::CidString,
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_blocks {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub cids: std::vec::Vec<std::string::String>,
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_checkout {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_head {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub root: atmo_core::CidString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_latest_commit {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    pub rev: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub collection: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<atmo_core::CidString>,
                    pub did: atmo_core::Did,
                    pub rkey: std::string::String,
                }
            }
            pub mod get_repo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<std::string::String>,
                }
            }
            pub mod get_repo_status {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub active: bool,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rev: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::sync::get_repo_status::Status>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod list_blobs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cids: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<std::string::String>,
                }
            }
            pub mod list_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub repos: std::vec::Vec<crate::com::atproto::sync::list_repos::Repo>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Repo {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    pub head: atmo_core::CidString,
                    pub rev: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<crate::com::atproto::sync::list_repos::Status>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod notify_of_update {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub hostname: std::string::String,
                }
            }
            pub mod request_crawl {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub hostname: std::string::String,
                }
            }
            pub mod subscribe_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Account {
                    #[doc = "Indicates that the account has a repository which can be fetched from the host that emitted this event."]
                    pub active: bool,
                    pub did: atmo_core::Did,
                    pub seq: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::sync::subscribe_repos::Status>,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Action {
                    #[serde(rename = "create")]
                    Create,
                    #[serde(rename = "update")]
                    Update,
                    #[serde(rename = "delete")]
                    Delete,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Commit {
                    pub blobs: std::vec::Vec<atmo_core::CidString>,
                    #[doc = "CAR file containing relevant blocks, as a diff since the previous repo state."]
                    #[serde(with = "atmo_core::bytes::serde")]
                    pub blocks: bytes::Bytes,
                    pub commit: atmo_core::CidLink,
                    pub ops: std::vec::Vec<crate::com::atproto::sync::subscribe_repos::RepoOp>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prev: std::option::Option<atmo_core::Nullable<atmo_core::CidLink>>,
                    #[doc = "DEPRECATED -- unused"]
                    pub rebase: bool,
                    pub repo: atmo_core::Did,
                    pub rev: std::string::String,
                    pub seq: i64,
                    pub since: atmo_core::Nullable<std::string::String>,
                    pub time: atmo_core::DateTimeString,
                    #[doc = "Indicates that this commit contained too many ops, or data size was too large. Consumers will need to make a separate request to get missing data."]
                    pub too_big: bool,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Handle {
                    pub did: atmo_core::Did,
                    pub handle: atmo_core::Handle,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Identity {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub handle: std::option::Option<atmo_core::Handle>,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::sync::subscribe_repos::Name,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Migrate {
                    pub did: atmo_core::Did,
                    pub migrate_to: atmo_core::Nullable<std::string::String>,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Name {
                    #[serde(rename = "OutdatedCursor")]
                    OutdatedCursor,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoOp {
                    pub action: crate::com::atproto::sync::subscribe_repos::Action,
                    pub cid: atmo_core::Nullable<atmo_core::CidLink>,
                    pub path: std::string::String,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Status {
                    #[serde(rename = "takendown")]
                    Takendown,
                    #[serde(rename = "suspended")]
                    Suspended,
                    #[serde(rename = "deleted")]
                    Deleted,
                    #[serde(rename = "deactivated")]
                    Deactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Tombstone {
                    pub did: atmo_core::Did,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
            }
        }
        pub mod temp {
            pub struct CheckSignupQueue;
            impl atmo_core::xrpc::Request for CheckSignupQueue {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::temp::check_signup_queue::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.temp.checkSignupQueue"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct FetchLabels;
            impl atmo_core::xrpc::Request for FetchLabels {
                type Params = crate::com::atproto::temp::fetch_labels::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::com::atproto::temp::fetch_labels::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.temp.fetchLabels"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct RequestPhoneVerification;
            impl atmo_core::xrpc::Request for RequestPhoneVerification {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::com::atproto::temp::request_phone_verification::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "com.atproto.temp.requestPhoneVerification"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub mod check_signup_queue {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub activated: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub estimated_time_ms: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub place_in_queue: std::option::Option<i64>,
                }
            }
            pub mod fetch_labels {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<i64>,
                }
            }
            pub mod request_phone_verification {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub phone_number: std::string::String,
                }
            }
        }
    }
}
pub mod tools {
    pub mod ozone {
        pub mod communication {
            pub struct CreateTemplate;
            impl atmo_core::xrpc::Request for CreateTemplate {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::communication::create_template::Input;
                type Output = crate::tools::ozone::communication::defs::TemplateView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.communication.createTemplate"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DeleteTemplate;
            impl atmo_core::xrpc::Request for DeleteTemplate {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::communication::delete_template::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.communication.deleteTemplate"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct ListTemplates;
            impl atmo_core::xrpc::Request for ListTemplates {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::communication::list_templates::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.communication.listTemplates"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpdateTemplate;
            impl atmo_core::xrpc::Request for UpdateTemplate {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::communication::update_template::Input;
                type Output = crate::tools::ozone::communication::defs::TemplateView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.communication.updateTemplate"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod create_template {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub content_markdown: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                    pub subject: std::string::String,
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct TemplateView {
                    pub content_markdown: std::string::String,
                    pub created_at: atmo_core::DateTimeString,
                    pub disabled: bool,
                    pub id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    pub last_updated_by: atmo_core::Did,
                    pub name: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                    pub updated_at: atmo_core::DateTimeString,
                }
            }
            pub mod delete_template {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub id: std::string::String,
                }
            }
            pub mod list_templates {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub communication_templates:
                        std::vec::Vec<crate::tools::ozone::communication::defs::TemplateView>,
                }
            }
            pub mod update_template {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub content_markdown: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    pub id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub name: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_by: std::option::Option<atmo_core::Did>,
                }
            }
        }
        pub mod moderation {
            pub struct EmitEvent;
            impl atmo_core::xrpc::Request for EmitEvent {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::moderation::emit_event::Input;
                type Output = crate::tools::ozone::moderation::defs::ModEventView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.emitEvent"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetEvent;
            impl atmo_core::xrpc::Request for GetEvent {
                type Params = crate::tools::ozone::moderation::get_event::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::defs::ModEventViewDetail;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.getEvent"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::tools::ozone::moderation::get_record::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::defs::RecordViewDetail;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.getRecord"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRecords;
            impl atmo_core::xrpc::Request for GetRecords {
                type Params = crate::tools::ozone::moderation::get_records::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::get_records::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.getRecords"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRepo;
            impl atmo_core::xrpc::Request for GetRepo {
                type Params = crate::tools::ozone::moderation::get_repo::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::defs::RepoViewDetail;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.getRepo"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct GetRepos;
            impl atmo_core::xrpc::Request for GetRepos {
                type Params = crate::tools::ozone::moderation::get_repos::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::get_repos::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.getRepos"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct QueryEvents;
            impl atmo_core::xrpc::Request for QueryEvents {
                type Params = crate::tools::ozone::moderation::query_events::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::query_events::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.queryEvents"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct QueryStatuses;
            impl atmo_core::xrpc::Request for QueryStatuses {
                type Params = crate::tools::ozone::moderation::query_statuses::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::query_statuses::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.queryStatuses"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchRepos;
            impl atmo_core::xrpc::Request for SearchRepos {
                type Params = crate::tools::ozone::moderation::search_repos::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::moderation::search_repos::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.moderation.searchRepos"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlobView {
                    pub cid: atmo_core::CidString,
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub details:
                        std::option::Option<crate::tools::ozone::moderation::defs::Details>,
                    pub mime_type: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub moderation:
                        std::option::Option<crate::tools::ozone::moderation::defs::Moderation>,
                    pub size: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Details {
                    ImageDetails(crate::tools::ozone::moderation::defs::ImageDetails),
                    VideoDetails(crate::tools::ozone::moderation::defs::VideoDetails),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Event {
                    ModEventAcknowledge(crate::tools::ozone::moderation::defs::ModEventAcknowledge),
                    ModEventComment(crate::tools::ozone::moderation::defs::ModEventComment),
                    ModEventDivert(crate::tools::ozone::moderation::defs::ModEventDivert),
                    ModEventEmail(crate::tools::ozone::moderation::defs::ModEventEmail),
                    ModEventEscalate(crate::tools::ozone::moderation::defs::ModEventEscalate),
                    ModEventLabel(crate::tools::ozone::moderation::defs::ModEventLabel),
                    ModEventMute(crate::tools::ozone::moderation::defs::ModEventMute),
                    ModEventMuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventMuteReporter,
                    ),
                    ModEventReport(crate::tools::ozone::moderation::defs::ModEventReport),
                    ModEventResolveAppeal(
                        crate::tools::ozone::moderation::defs::ModEventResolveAppeal,
                    ),
                    ModEventReverseTakedown(
                        crate::tools::ozone::moderation::defs::ModEventReverseTakedown,
                    ),
                    ModEventTag(crate::tools::ozone::moderation::defs::ModEventTag),
                    ModEventTakedown(crate::tools::ozone::moderation::defs::ModEventTakedown),
                    ModEventUnmute(crate::tools::ozone::moderation::defs::ModEventUnmute),
                    ModEventUnmuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventUnmuteReporter,
                    ),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ImageDetails {
                    pub height: i64,
                    pub width: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventAcknowledge {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventComment {
                    pub comment: std::string::String,
                    #[doc = "Make the comment persistent on the subject"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sticky: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventDivert {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEmail {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub content: std::option::Option<std::string::String>,
                    pub subject_line: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEscalate {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventLabel {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub create_label_vals: std::vec::Vec<std::string::String>,
                    pub negate_label_vals: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMute {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub duration_in_hours: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMuteReporter {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub duration_in_hours: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReport {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[doc = "Set to true if the reporter was muted from reporting at the time of the event. These reports won't impact the reviewState of the subject."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub is_reporter_muted: std::option::Option<bool>,
                    pub report_type: crate::com::atproto::moderation::defs::ReasonType,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventResolveAppeal {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReverseTakedown {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTag {
                    pub add: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub remove: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTakedown {
                    #[doc = "If true, all other reports on content authored by this account will be resolved (acknowledged)."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub acknowledge_account_subjects: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub duration_in_hours: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmute {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmuteReporter {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventView {
                    pub created_at: atmo_core::DateTimeString,
                    pub created_by: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub creator_handle: std::option::Option<std::string::String>,
                    pub event: crate::tools::ozone::moderation::defs::Event,
                    pub id: i64,
                    pub subject: crate::tools::ozone::moderation::defs::Subject,
                    pub subject_blob_cids: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_handle: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventViewDetail {
                    pub created_at: atmo_core::DateTimeString,
                    pub created_by: atmo_core::Did,
                    pub event: crate::tools::ozone::moderation::defs::ModEventViewDetailEvent,
                    pub id: i64,
                    pub subject: crate::tools::ozone::moderation::defs::ModEventViewDetailSubject,
                    pub subject_blobs:
                        std::vec::Vec<crate::tools::ozone::moderation::defs::BlobView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ModEventViewDetailEvent {
                    ModEventAcknowledge(crate::tools::ozone::moderation::defs::ModEventAcknowledge),
                    ModEventComment(crate::tools::ozone::moderation::defs::ModEventComment),
                    ModEventDivert(crate::tools::ozone::moderation::defs::ModEventDivert),
                    ModEventEmail(crate::tools::ozone::moderation::defs::ModEventEmail),
                    ModEventEscalate(crate::tools::ozone::moderation::defs::ModEventEscalate),
                    ModEventLabel(crate::tools::ozone::moderation::defs::ModEventLabel),
                    ModEventMute(crate::tools::ozone::moderation::defs::ModEventMute),
                    ModEventMuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventMuteReporter,
                    ),
                    ModEventReport(crate::tools::ozone::moderation::defs::ModEventReport),
                    ModEventResolveAppeal(
                        crate::tools::ozone::moderation::defs::ModEventResolveAppeal,
                    ),
                    ModEventReverseTakedown(
                        crate::tools::ozone::moderation::defs::ModEventReverseTakedown,
                    ),
                    ModEventTag(crate::tools::ozone::moderation::defs::ModEventTag),
                    ModEventTakedown(crate::tools::ozone::moderation::defs::ModEventTakedown),
                    ModEventUnmute(crate::tools::ozone::moderation::defs::ModEventUnmute),
                    ModEventUnmuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventUnmuteReporter,
                    ),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ModEventViewDetailSubject {
                    RecordView(crate::tools::ozone::moderation::defs::RecordView),
                    RecordViewNotFound(crate::tools::ozone::moderation::defs::RecordViewNotFound),
                    RepoView(crate::tools::ozone::moderation::defs::RepoView),
                    RepoViewNotFound(crate::tools::ozone::moderation::defs::RepoViewNotFound),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Moderation {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModerationDetail {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordView {
                    pub blob_cids: std::vec::Vec<std::string::String>,
                    pub cid: atmo_core::CidString,
                    pub indexed_at: atmo_core::DateTimeString,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    pub repo: crate::tools::ozone::moderation::defs::RepoView,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewDetail {
                    pub blobs: std::vec::Vec<crate::tools::ozone::moderation::defs::BlobView>,
                    pub cid: atmo_core::CidString,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    pub moderation: crate::tools::ozone::moderation::defs::ModerationDetail,
                    pub repo: crate::tools::ozone::moderation::defs::RepoView,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewNotFound {
                    pub uri: atmo_core::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    pub related_records: std::vec::Vec<atmo_core::Unknown>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewDetail {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo_core::DateTimeString>,
                    pub handle: atmo_core::Handle,
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites: std::option::Option<
                        std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    pub moderation: crate::tools::ozone::moderation::defs::ModerationDetail,
                    pub related_records: std::vec::Vec<atmo_core::Unknown>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewNotFound {
                    pub did: atmo_core::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    MessageRef(crate::chat::bsky::convo::defs::MessageRef),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SubjectReviewState {
                    #[doc = "Moderator review status of a subject: Open. Indicates that the subject needs to be reviewed by a moderator"]
                    #[serde(rename = "#reviewOpen")]
                    ReviewOpen,
                    #[doc = "Moderator review status of a subject: Escalated. Indicates that the subject was escalated for review by a moderator"]
                    #[serde(rename = "#reviewEscalated")]
                    ReviewEscalated,
                    #[doc = "Moderator review status of a subject: Closed. Indicates that the subject was already reviewed and resolved by a moderator"]
                    #[serde(rename = "#reviewClosed")]
                    ReviewClosed,
                    #[doc = "Moderator review status of a subject: Unnecessary. Indicates that the subject does not need a review at the moment but there is probably some moderation related metadata available for it"]
                    #[serde(rename = "#reviewNone")]
                    ReviewNone,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SubjectStatusView {
                    #[doc = "True indicates that the a previously taken moderator action was appealed against, by the author of the content. False indicates last appeal was resolved by moderators."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appealed: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub created_at: atmo_core::DateTimeString,
                    pub id: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_appealed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reported_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_reporting_until: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_until: std::option::Option<atmo_core::DateTimeString>,
                    pub review_state: crate::tools::ozone::moderation::defs::SubjectReviewState,
                    pub subject: crate::tools::ozone::moderation::defs::SubjectStatusViewSubject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_blob_cids: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_repo_handle: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub suspend_until: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takendown: std::option::Option<bool>,
                    pub updated_at: atmo_core::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum SubjectStatusViewSubject {
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct VideoDetails {
                    pub height: i64,
                    pub length: i64,
                    pub width: i64,
                }
            }
            pub mod emit_event {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Event {
                    ModEventAcknowledge(crate::tools::ozone::moderation::defs::ModEventAcknowledge),
                    ModEventComment(crate::tools::ozone::moderation::defs::ModEventComment),
                    ModEventEmail(crate::tools::ozone::moderation::defs::ModEventEmail),
                    ModEventEscalate(crate::tools::ozone::moderation::defs::ModEventEscalate),
                    ModEventLabel(crate::tools::ozone::moderation::defs::ModEventLabel),
                    ModEventMute(crate::tools::ozone::moderation::defs::ModEventMute),
                    ModEventMuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventMuteReporter,
                    ),
                    ModEventReport(crate::tools::ozone::moderation::defs::ModEventReport),
                    ModEventResolveAppeal(
                        crate::tools::ozone::moderation::defs::ModEventResolveAppeal,
                    ),
                    ModEventReverseTakedown(
                        crate::tools::ozone::moderation::defs::ModEventReverseTakedown,
                    ),
                    ModEventTag(crate::tools::ozone::moderation::defs::ModEventTag),
                    ModEventTakedown(crate::tools::ozone::moderation::defs::ModEventTakedown),
                    ModEventUnmute(crate::tools::ozone::moderation::defs::ModEventUnmute),
                    ModEventUnmuteReporter(
                        crate::tools::ozone::moderation::defs::ModEventUnmuteReporter,
                    ),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub created_by: atmo_core::Did,
                    pub event: crate::tools::ozone::moderation::emit_event::Event,
                    pub subject: crate::tools::ozone::moderation::emit_event::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_blob_cids: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    StrongRef(crate::com::atproto::repo::StrongRef),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod get_event {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub id: i64,
                }
            }
            pub mod get_record {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_records {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub records:
                        std::vec::Vec<crate::tools::ozone::moderation::get_records::Records>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Records {
                    RecordViewDetail(crate::tools::ozone::moderation::defs::RecordViewDetail),
                    RecordViewNotFound(crate::tools::ozone::moderation::defs::RecordViewNotFound),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod get_repo {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub repos: std::vec::Vec<crate::tools::ozone::moderation::get_repos::Repos>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Repos {
                    RepoViewDetail(crate::tools::ozone::moderation::defs::RepoViewDetail),
                    RepoViewNotFound(crate::tools::ozone::moderation::defs::RepoViewNotFound),
                    #[serde(untagged)]
                    Other(atmo_core::Unknown),
                }
            }
            pub mod query_events {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub events: std::vec::Vec<crate::tools::ozone::moderation::defs::ModEventView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub added_labels: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub added_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[doc = "If true, only events with comments are returned"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub has_comment: std::option::Option<bool>,
                    #[doc = "If true, events on all record types (posts, lists, profile etc.) owned by the did are returned"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_all_user_records: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub removed_labels: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub removed_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub report_types: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction: std::option::Option<
                        crate::tools::ozone::moderation::query_events::SortDirection,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub types: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SortDirection {
                    #[serde(rename = "asc")]
                    Asc,
                    #[serde(rename = "desc")]
                    Desc,
                }
            }
            pub mod query_statuses {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub subject_statuses:
                        std::vec::Vec<crate::tools::ozone::moderation::defs::SubjectStatusView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[doc = "Get subjects in unresolved appealed status"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appealed: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exclude_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ignore_subjects: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[doc = "All subjects belonging to the account specified in the 'subject' param will be returned."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_all_user_records: std::option::Option<bool>,
                    #[doc = "By default, we don't include muted subjects in the results. Set this to true to include them."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[doc = "When set to true, only muted subjects and reporters will be returned."]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub only_muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reported_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reported_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub review_state: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reviewed_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reviewed_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction: std::option::Option<
                        crate::tools::ozone::moderation::query_statuses::SortDirection,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_field: std::option::Option<
                        crate::tools::ozone::moderation::query_statuses::SortField,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[doc = "Get subjects that were taken down"]
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takendown: std::option::Option<bool>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SortDirection {
                    #[serde(rename = "asc")]
                    Asc,
                    #[serde(rename = "desc")]
                    Desc,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SortField {
                    #[serde(rename = "lastReviewedAt")]
                    LastReviewedAt,
                    #[serde(rename = "lastReportedAt")]
                    LastReportedAt,
                }
            }
            pub mod search_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub repos: std::vec::Vec<crate::tools::ozone::moderation::defs::RepoView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub q: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub term: std::option::Option<std::string::String>,
                }
            }
        }
        pub mod server {
            pub struct GetConfig;
            impl atmo_core::xrpc::Request for GetConfig {
                type Params = atmo_core::xrpc::NoParams;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::server::get_config::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.server.getConfig"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod get_config {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appview:
                        std::option::Option<crate::tools::ozone::server::get_config::ServiceConfig>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blob_divert:
                        std::option::Option<crate::tools::ozone::server::get_config::ServiceConfig>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat:
                        std::option::Option<crate::tools::ozone::server::get_config::ServiceConfig>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pds:
                        std::option::Option<crate::tools::ozone::server::get_config::ServiceConfig>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::tools::ozone::server::get_config::ViewerConfig>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Role {
                    #[doc = "Admin role. Highest level of access, can perform all actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                    RoleAdmin,
                    #[doc = "Moderator role. Can perform most actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                    RoleModerator,
                    #[doc = "Triage role. Mostly intended for monitoring and escalating issues."]
                    #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                    RoleTriage,
                    #[serde(untagged)]
                    Other(String),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ServiceConfig {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub url: std::option::Option<url::Url>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerConfig {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub role: std::option::Option<crate::tools::ozone::server::get_config::Role>,
                }
            }
        }
        pub mod set {
            pub struct AddValues;
            impl atmo_core::xrpc::Request for AddValues {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::set::add_values::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.addValues"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct DeleteSet;
            impl atmo_core::xrpc::Request for DeleteSet {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::set::delete_set::Input;
                type Output = crate::tools::ozone::set::delete_set::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.deleteSet"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DeleteValues;
            impl atmo_core::xrpc::Request for DeleteValues {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::set::delete_values::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.deleteValues"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct GetValues;
            impl atmo_core::xrpc::Request for GetValues {
                type Params = crate::tools::ozone::set::get_values::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::set::get_values::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.getValues"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct QuerySets;
            impl atmo_core::xrpc::Request for QuerySets {
                type Params = crate::tools::ozone::set::query_sets::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::set::query_sets::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.querySets"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpsertSet;
            impl atmo_core::xrpc::Request for UpsertSet {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::set::defs::Set;
                type Output = crate::tools::ozone::set::defs::SetView;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.set.upsertSet"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod add_values {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    pub values: std::vec::Vec<std::string::String>,
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Set {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SetView {
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                    pub set_size: i64,
                    pub updated_at: atmo_core::DateTimeString,
                }
            }
            pub mod delete_set {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
            pub mod delete_values {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    pub values: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_values {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub set: crate::tools::ozone::set::defs::SetView,
                    pub values: std::vec::Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub name: std::string::String,
                }
            }
            pub mod query_sets {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub sets: std::vec::Vec<crate::tools::ozone::set::defs::SetView>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub name_prefix: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_by: std::option::Option<crate::tools::ozone::set::query_sets::SortBy>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction:
                        std::option::Option<crate::tools::ozone::set::query_sets::SortDirection>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SortBy {
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "createdAt")]
                    CreatedAt,
                    #[serde(rename = "updatedAt")]
                    UpdatedAt,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum SortDirection {
                    #[serde(rename = "asc")]
                    Asc,
                    #[serde(rename = "desc")]
                    Desc,
                }
            }
        }
        pub mod signature {
            pub struct FindCorrelation;
            impl atmo_core::xrpc::Request for FindCorrelation {
                type Params = crate::tools::ozone::signature::find_correlation::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::signature::find_correlation::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.signature.findCorrelation"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct FindRelatedAccounts;
            impl atmo_core::xrpc::Request for FindRelatedAccounts {
                type Params = crate::tools::ozone::signature::find_related_accounts::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::signature::find_related_accounts::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.signature.findRelatedAccounts"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct SearchAccounts;
            impl atmo_core::xrpc::Request for SearchAccounts {
                type Params = crate::tools::ozone::signature::search_accounts::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::signature::search_accounts::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.signature.searchAccounts"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SigDetail {
                    pub property: std::string::String,
                    pub value: std::string::String,
                }
            }
            pub mod find_correlation {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub details: std::vec::Vec<crate::tools::ozone::signature::defs::SigDetail>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<std::string::String>,
                }
            }
            pub mod find_related_accounts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<
                        crate::tools::ozone::signature::find_related_accounts::RelatedAccount,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RelatedAccount {
                    pub account: crate::com::atproto::admin::defs::AccountView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub similarities: std::option::Option<
                        std::vec::Vec<crate::tools::ozone::signature::defs::SigDetail>,
                    >,
                }
            }
            pub mod search_accounts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub values: std::vec::Vec<std::string::String>,
                }
            }
        }
        pub mod team {
            pub struct AddMember;
            impl atmo_core::xrpc::Request for AddMember {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::team::add_member::Input;
                type Output = crate::tools::ozone::team::defs::Member;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.team.addMember"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct DeleteMember;
            impl atmo_core::xrpc::Request for DeleteMember {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::team::delete_member::Input;
                type Output = atmo_core::xrpc::NoOutput;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.team.deleteMember"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "*/*"
                }
            }
            pub struct ListMembers;
            impl atmo_core::xrpc::Request for ListMembers {
                type Params = crate::tools::ozone::team::list_members::Params;
                type Input = atmo_core::xrpc::NoInput;
                type Output = crate::tools::ozone::team::list_members::Output;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.team.listMembers"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub struct UpdateMember;
            impl atmo_core::xrpc::Request for UpdateMember {
                type Params = atmo_core::xrpc::NoParams;
                type Input = crate::tools::ozone::team::update_member::Input;
                type Output = crate::tools::ozone::team::defs::Member;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.team.updateMember"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod add_member {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub role: crate::tools::ozone::team::add_member::Role,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Role {
                    #[doc = "Admin role. Highest level of access, can perform all actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                    RoleAdmin,
                    #[doc = "Moderator role. Can perform most actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                    RoleModerator,
                    #[doc = "Triage role. Mostly intended for monitoring and escalating issues."]
                    #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                    RoleTriage,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Member {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_updated_by: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub profile:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileViewDetailed>,
                    pub role: crate::tools::ozone::team::defs::Role,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Role {
                    #[doc = "Admin role. Highest level of access, can perform all actions."]
                    #[serde(rename = "#roleAdmin")]
                    RoleAdmin,
                    #[doc = "Moderator role. Can perform most actions."]
                    #[serde(rename = "#roleModerator")]
                    RoleModerator,
                    #[doc = "Triage role. Mostly intended for monitoring and escalating issues."]
                    #[serde(rename = "#roleTriage")]
                    RoleTriage,
                    #[serde(untagged)]
                    Other(String),
                }
            }
            pub mod delete_member {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                }
            }
            pub mod list_members {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub members: std::vec::Vec<crate::tools::ozone::team::defs::Member>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod update_member {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub role: std::option::Option<crate::tools::ozone::team::update_member::Role>,
                }
                #[derive(serde :: Serialize, serde :: Deserialize)]
                pub enum Role {
                    #[doc = "Admin role. Highest level of access, can perform all actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                    RoleAdmin,
                    #[doc = "Moderator role. Can perform most actions."]
                    #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                    RoleModerator,
                    #[doc = "Triage role. Mostly intended for monitoring and escalating issues."]
                    #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                    RoleTriage,
                    #[serde(untagged)]
                    Other(String),
                }
            }
        }
    }
}
