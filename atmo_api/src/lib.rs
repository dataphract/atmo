pub mod app {
    pub mod bsky {
        pub mod actor {
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active_progress_guide:
                        std::option::Option<crate::app::bsky::actor::defs::BskyAppProgressGuide>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub nuxs: std::option::Option<Vec<crate::app::bsky::actor::defs::Nux>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub queued_nudges: std::option::Option<Vec<std::string::String>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ContentLabelPref {
                    pub label: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labeler_did: std::option::Option<atmo::did::Did>,
                    pub visibility: crate::app::bsky::actor::defs::Visibility,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FeedViewPref {
                    pub feed: std::string::String,
                    #[doc = "Hide quote posts in the feed."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_quote_posts: std::option::Option<bool>,
                    #[doc = "Hide replies in the feed."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_like_count: std::option::Option<i64>,
                    #[doc = "Hide replies in the feed if they are not by followed users."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_unfollowed: std::option::Option<bool>,
                    #[doc = "Hide reposts in the feed."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_reposts: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct HiddenPostsPref {
                    pub items: Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InterestsPref {
                    pub tags: Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct KnownFollowers {
                    pub count: i64,
                    pub followers: Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPrefItem {
                    pub did: atmo::did::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelersPref {
                    pub labelers: Vec<crate::app::bsky::actor::defs::LabelerPrefItem>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MutedWord {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub actor_target:
                        std::option::Option<crate::app::bsky::actor::defs::ActorTarget>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub id: std::option::Option<std::string::String>,
                    pub targets: Vec<crate::app::bsky::actor::defs::MutedWordTarget>,
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
                    pub items: Vec<crate::app::bsky::actor::defs::MutedWord>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Nux {
                    pub completed: bool,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub data: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct PersonalDetailsPref {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub birth_date: std::option::Option<atmo::datetime::DateTimeString>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileAssociated {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociatedChat>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feedgens: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labeler: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lists: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub starter_packs: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileAssociatedChat {
                    pub allow_incoming: crate::app::bsky::actor::defs::AllowIncoming,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo::handle::Handle,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo::handle::Handle,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewDetailed {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub banner: std::option::Option<url::Url>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followers_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub follows_count: std::option::Option<i64>,
                    pub handle: atmo::handle::Handle,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_via_starter_pack:
                        std::option::Option<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned_post:
                        std::option::Option<crate::com::atproto::repo::strong_ref::Main>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub posts_count: std::option::Option<i64>,
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
                    pub pinned: Vec<std::string::String>,
                    pub saved: Vec<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub timeline_index: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeedsPrefV2 {
                    pub items: Vec<crate::app::bsky::actor::defs::SavedFeed>,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prioritize_followed_users: std::option::Option<bool>,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked_by: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking_by_list:
                        std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub known_followers:
                        std::option::Option<crate::app::bsky::actor::defs::KnownFollowers>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
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
            pub mod get_preferences {}
            pub mod get_profile {}
            pub mod get_profiles {}
            pub mod get_suggestions {}
            pub mod profile {}
            pub mod put_preferences {}
            pub mod search_actors {}
            pub mod search_actors_typeahead {}
        }
        pub mod embed {
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumb: std::option::Option<atmo::Blob>,
                    pub title: std::string::String,
                    pub uri: url::Url,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub external: crate::app::bsky::embed::external::External,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub external: crate::app::bsky::embed::external::ViewExternal,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewExternal {
                    pub description: std::string::String,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub image: atmo::Blob,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub images: Vec<crate::app::bsky::embed::images::Image>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub images: Vec<crate::app::bsky::embed::images::ViewImage>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewImage {
                    pub alt: std::string::String,
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub record: crate::com::atproto::repo::strong_ref::Main,
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub record: crate::app::bsky::embed::record::Record,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewBlocked {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewDetached {
                    pub detached: bool,
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewNotFound {
                    pub not_found: bool,
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewRecord {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo::CidString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embeds: std::option::Option<Vec<crate::app::bsky::embed::record::Embeds>>,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost_count: std::option::Option<i64>,
                    pub uri: atmo::at_uri::AtUri,
                    pub value: (),
                }
            }
            pub mod record_with_media {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub media: crate::app::bsky::embed::record_with_media::Media,
                    pub record: crate::app::bsky::embed::record::Main,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Media {
                    ExternalMain(std::boxed::Box<crate::app::bsky::embed::external::Main>),
                    ImagesMain(std::boxed::Box<crate::app::bsky::embed::images::Main>),
                    VideoMain(std::boxed::Box<crate::app::bsky::embed::video::Main>),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub media: crate::app::bsky::embed::record_with_media::ViewMedia,
                    pub record: crate::app::bsky::embed::record::View,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ViewMedia {
                    ExternalView(std::boxed::Box<crate::app::bsky::embed::external::View>),
                    ImagesView(std::boxed::Box<crate::app::bsky::embed::images::View>),
                    VideoView(std::boxed::Box<crate::app::bsky::embed::video::View>),
                    #[serde(other)]
                    Other,
                }
            }
            pub mod video {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Caption {
                    pub file: atmo::Blob,
                    pub lang: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub alt: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub captions: std::option::Option<Vec<crate::app::bsky::embed::video::Caption>>,
                    pub video: atmo::Blob,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub alt: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub cid: atmo::CidString,
                    pub playlist: url::Url,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumbnail: std::option::Option<url::Url>,
                }
            }
        }
        pub mod feed {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedAuthor {
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedPost {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Embed {
                    ExternalView(crate::app::bsky::embed::external::View),
                    ImagesView(crate::app::bsky::embed::images::View),
                    RecordView(crate::app::bsky::embed::record::View),
                    RecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
                    VideoView(crate::app::bsky::embed::video::View),
                    #[serde(other)]
                    Other,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<crate::app::bsky::feed::defs::Reason>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply: std::option::Option<crate::app::bsky::feed::defs::ReplyRef>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub accepts_interactions: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
                    pub did: atmo::did::Did,
                    pub display_name: std::string::String,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::feed::defs::GeneratorViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorViewerState {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo::at_uri::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Interaction {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub event: std::option::Option<crate::app::bsky::feed::defs::Event>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub item: std::option::Option<atmo::at_uri::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundPost {
                    pub not_found: bool,
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Parent {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    PostView(crate::app::bsky::feed::defs::PostView),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct PostView {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo::CidString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed: std::option::Option<crate::app::bsky::feed::defs::Embed>,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    pub record: (),
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threadgate:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadgateView>,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::feed::defs::ViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Reason {
                    ReasonPin(crate::app::bsky::feed::defs::ReasonPin),
                    ReasonRepost(crate::app::bsky::feed::defs::ReasonRepost),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonPin {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonRepost {
                    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub indexed_at: atmo::datetime::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Replies {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    ThreadViewPost(std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonFeedPost {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason:
                        std::option::Option<crate::app::bsky::feed::defs::SkeletonFeedPostReason>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum SkeletonFeedPostReason {
                    SkeletonReasonPin(crate::app::bsky::feed::defs::SkeletonReasonPin),
                    SkeletonReasonRepost(crate::app::bsky::feed::defs::SkeletonReasonRepost),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonPin {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonRepost {
                    pub repost: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadViewPost {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub parent:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadViewPostParent>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub replies: std::option::Option<Vec<crate::app::bsky::feed::defs::Replies>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ThreadViewPostParent {
                    BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                    NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                    ThreadViewPost(std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadgateView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo::CidString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lists:
                        std::option::Option<Vec<crate::app::bsky::graph::defs::ListViewBasic>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub record: std::option::Option<()>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub uri: std::option::Option<atmo::at_uri::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerState {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embedding_disabled: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_disabled: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thread_muted: std::option::Option<bool>,
                }
            }
            pub mod describe_feed_generator {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Feed {
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<std::string::String>,
                }
            }
            pub mod generator {}
            pub mod get_actor_feeds {}
            pub mod get_actor_likes {}
            pub mod get_author_feed {}
            pub mod get_feed {}
            pub mod get_feed_generator {}
            pub mod get_feed_generators {}
            pub mod get_feed_skeleton {}
            pub mod get_likes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Like {
                    pub actor: crate::app::bsky::actor::defs::ProfileView,
                    pub created_at: atmo::datetime::DateTimeString,
                    pub indexed_at: atmo::datetime::DateTimeString,
                }
            }
            pub mod get_list_feed {}
            pub mod get_post_thread {}
            pub mod get_posts {}
            pub mod get_quotes {}
            pub mod get_reposted_by {}
            pub mod get_suggested_feeds {}
            pub mod get_timeline {}
            pub mod like {}
            pub mod post {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Entity {
                    pub index: crate::app::bsky::feed::post::TextSlice,
                    pub ty: std::string::String,
                    pub value: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
                    pub parent: crate::com::atproto::repo::strong_ref::Main,
                    pub root: crate::com::atproto::repo::strong_ref::Main,
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
            pub mod repost {}
            pub mod search_posts {}
            pub mod send_interactions {}
            pub mod threadgate {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FollowingRule {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListRule {
                    pub list: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MentionRule {}
            }
        }
        pub mod graph {
            pub mod block {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListItemView {
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                    pub uri: atmo::at_uri::AtUri,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewBasic {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo::CidString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewerState {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundActor {
                    pub actor: atmo::AtIdentifier,
                    pub not_found: bool,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Relationship {
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo::at_uri::AtUri>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo::at_uri::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackView {
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feeds:
                        std::option::Option<Vec<crate::app::bsky::feed::defs::GeneratorView>>,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_week_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list: std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_items_sample:
                        std::option::Option<Vec<crate::app::bsky::graph::defs::ListItemView>>,
                    pub record: (),
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackViewBasic {
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_week_count: std::option::Option<i64>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub record: (),
                    pub uri: atmo::at_uri::AtUri,
                }
            }
            pub mod follow {}
            pub mod get_actor_starter_packs {}
            pub mod get_blocks {}
            pub mod get_followers {}
            pub mod get_follows {}
            pub mod get_known_followers {}
            pub mod get_list {}
            pub mod get_list_blocks {}
            pub mod get_list_mutes {}
            pub mod get_lists {}
            pub mod get_mutes {}
            pub mod get_relationships {}
            pub mod get_starter_pack {}
            pub mod get_starter_packs {}
            pub mod get_suggested_follows_by_actor {}
            pub mod list {}
            pub mod listblock {}
            pub mod listitem {}
            pub mod mute_actor {}
            pub mod mute_actor_list {}
            pub mod mute_thread {}
            pub mod starterpack {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct FeedItem {
                    pub uri: atmo::at_uri::AtUri,
                }
            }
            pub mod unmute_actor {}
            pub mod unmute_actor_list {}
            pub mod unmute_thread {}
        }
        pub mod labeler {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPolicies {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub label_value_definitions: std::option::Option<
                        Vec<crate::com::atproto::label::defs::LabelValueDefinition>,
                    >,
                    pub label_values: Vec<crate::com::atproto::label::defs::LabelValue>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerView {
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewDetailed {
                    pub cid: atmo::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub policies: crate::app::bsky::labeler::defs::LabelerPolicies,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewerState {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo::at_uri::AtUri>,
                }
            }
            pub mod get_services {}
            pub mod service {}
        }
        pub mod notification {
            pub mod get_unread_count {}
            pub mod list_notifications {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Notification {
                    pub author: crate::app::bsky::actor::defs::ProfileView,
                    pub cid: atmo::CidString,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    pub is_read: bool,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    pub reason: crate::app::bsky::notification::list_notifications::Reason,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason_subject: std::option::Option<atmo::at_uri::AtUri>,
                    pub record: (),
                    pub uri: atmo::at_uri::AtUri,
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
            pub mod put_preferences {}
            pub mod register_push {}
            pub mod update_seen {}
        }
        pub mod richtext {
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Link {
                    pub uri: url::Url,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub features: Vec<crate::app::bsky::richtext::facet::Features>,
                    pub index: crate::app::bsky::richtext::facet::ByteSlice,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Mention {
                    pub did: atmo::did::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Tag {
                    pub tag: std::string::String,
                }
            }
        }
        pub mod unspecced {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchActor {
                    pub did: atmo::did::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchPost {
                    pub uri: atmo::at_uri::AtUri,
                }
            }
            pub mod get_popular_feed_generators {}
            pub mod get_suggestions_skeleton {}
            pub mod get_tagged_suggestions {
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
            pub mod search_actors_skeleton {}
            pub mod search_posts_skeleton {}
        }
        pub mod video {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct JobStatus {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blob: std::option::Option<atmo::Blob>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub error: std::option::Option<std::string::String>,
                    pub job_id: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
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
            pub mod get_job_status {}
            pub mod get_upload_limits {}
            pub mod upload_video {}
        }
    }
}
pub mod chat {
    pub mod bsky {
        pub mod actor {
            pub mod declaration {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[doc = "Set to true when the actor cannot actively participate in converations"]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat_disabled: std::option::Option<bool>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo::handle::Handle,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
            }
            pub mod delete_account {}
            pub mod export_account_data {}
        }
        pub mod convo {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ConvoView {
                    pub id: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_message:
                        std::option::Option<crate::chat::bsky::convo::defs::LastMessage>,
                    pub members: Vec<crate::chat::bsky::actor::defs::ProfileViewBasic>,
                    pub muted: bool,
                    pub rev: std::string::String,
                    pub unread_count: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct DeletedMessageView {
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    pub sent_at: atmo::datetime::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Embed {
                    Main(crate::app::bsky::embed::record::Main),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum LastMessage {
                    DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                    MessageView(crate::chat::bsky::convo::defs::MessageView),
                    #[serde(other)]
                    Other,
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
                    #[serde(other)]
                    Other,
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageInput {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed: std::option::Option<crate::chat::bsky::convo::defs::Embed>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets: std::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
                    pub text: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageRef {
                    pub convo_id: std::string::String,
                    pub did: atmo::did::Did,
                    pub message_id: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed:
                        std::option::Option<crate::chat::bsky::convo::defs::MessageViewEmbed>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets: std::option::Option<Vec<crate::app::bsky::richtext::facet::Main>>,
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    pub sent_at: atmo::datetime::DateTimeString,
                    pub text: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum MessageViewEmbed {
                    View(crate::app::bsky::embed::record::View),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct MessageViewSender {
                    pub did: atmo::did::Did,
                }
            }
            pub mod delete_message_for_self {}
            pub mod get_convo {}
            pub mod get_convo_for_members {}
            pub mod get_log {}
            pub mod get_messages {}
            pub mod leave_convo {}
            pub mod list_convos {}
            pub mod mute_convo {}
            pub mod send_message {}
            pub mod send_message_batch {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BatchItem {
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::MessageInput,
                }
            }
            pub mod unmute_convo {}
            pub mod update_read {}
        }
        pub mod moderation {
            pub mod get_actor_metadata {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Metadata {
                    pub convos: i64,
                    pub convos_started: i64,
                    pub messages_received: i64,
                    pub messages_sent: i64,
                }
            }
            pub mod get_message_context {}
            pub mod update_actor_access {}
        }
    }
}
pub mod com {
    pub mod atproto {
        pub mod admin {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AccountView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub handle: atmo::handle::Handle,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites:
                        std::option::Option<Vec<crate::com::atproto::server::defs::InviteCode>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub related_records: std::option::Option<Vec<atmo::Unknown>>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoBlobRef {
                    pub cid: atmo::CidString,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub record_uri: std::option::Option<atmo::at_uri::AtUri>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoRef {
                    pub did: atmo::did::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct StatusAttr {
                    pub applied: bool,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ref_: std::option::Option<std::string::String>,
                }
            }
            pub mod delete_account {}
            pub mod disable_account_invites {}
            pub mod disable_invite_codes {}
            pub mod enable_account_invites {}
            pub mod get_account_info {}
            pub mod get_account_infos {}
            pub mod get_invite_codes {}
            pub mod get_subject_status {}
            pub mod search_accounts {}
            pub mod send_email {}
            pub mod update_account_email {}
            pub mod update_account_handle {}
            pub mod update_account_password {}
            pub mod update_subject_status {}
        }
        pub mod identity {
            pub mod get_recommended_did_credentials {}
            pub mod request_plc_operation_signature {}
            pub mod resolve_handle {}
            pub mod sign_plc_operation {}
            pub mod submit_plc_operation {}
            pub mod update_handle {}
        }
        pub mod label {
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo::CidString>,
                    pub cts: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exp: std::option::Option<atmo::datetime::DateTimeString>,
                    #[doc = "If true, this is a negation label, overwriting a previous label."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub neg: std::option::Option<bool>,
                    #[doc = "Signature of dag-cbor encoded label."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sig: std::option::Option<Vec<u8>>,
                    pub src: atmo::did::Did,
                    pub uri: url::Url,
                    pub val: std::string::String,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub adult_only: std::option::Option<bool>,
                    pub blurs: crate::com::atproto::label::defs::Blurs,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub default_setting:
                        std::option::Option<crate::com::atproto::label::defs::DefaultSetting>,
                    pub identifier: std::string::String,
                    pub locales: Vec<crate::com::atproto::label::defs::LabelValueDefinitionStrings>,
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
                    pub values: Vec<crate::com::atproto::label::defs::SelfLabel>,
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
            pub mod query_labels {}
            pub mod subscribe_labels {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::label::subscribe_labels::Name,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Labels {
                    pub labels: Vec<crate::com::atproto::label::defs::Label>,
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
            pub mod create_report {}
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
            pub mod apply_writes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Create {
                    pub collection: atmo::nsid::Nsid,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey: std::option::Option<std::string::String>,
                    pub value: (),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct CreateResult {
                    pub cid: atmo::CidString,
                    pub uri: atmo::at_uri::AtUri,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::apply_writes::ValidationStatus,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Delete {
                    pub collection: atmo::nsid::Nsid,
                    pub rkey: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct DeleteResult {}
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Update {
                    pub collection: atmo::nsid::Nsid,
                    pub rkey: std::string::String,
                    pub value: (),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct UpdateResult {
                    pub cid: atmo::CidString,
                    pub uri: atmo::at_uri::AtUri,
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
            }
            pub mod create_record {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct CommitMeta {
                    pub cid: atmo::CidString,
                    pub rev: std::string::String,
                }
            }
            pub mod delete_record {}
            pub mod describe_repo {}
            pub mod get_record {}
            pub mod import_repo {}
            pub mod list_missing_blobs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordBlob {
                    pub cid: atmo::CidString,
                    pub record_uri: atmo::at_uri::AtUri,
                }
            }
            pub mod list_records {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Record {
                    pub cid: atmo::CidString,
                    pub uri: atmo::at_uri::AtUri,
                    pub value: (),
                }
            }
            pub mod put_record {}
            pub mod strong_ref {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Main {
                    pub cid: atmo::CidString,
                    pub uri: atmo::at_uri::AtUri,
                }
            }
            pub mod upload_blob {}
        }
        pub mod server {
            pub mod activate_account {}
            pub mod check_account_status {}
            pub mod confirm_email {}
            pub mod create_account {}
            pub mod create_app_password {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    pub created_at: atmo::datetime::DateTimeString,
                    pub name: std::string::String,
                    pub password: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
            }
            pub mod create_invite_code {}
            pub mod create_invite_codes {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AccountCodes {
                    pub account: std::string::String,
                    pub codes: Vec<std::string::String>,
                }
            }
            pub mod create_session {}
            pub mod deactivate_account {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCode {
                    pub available: i64,
                    pub code: std::string::String,
                    pub created_at: atmo::datetime::DateTimeString,
                    pub created_by: std::string::String,
                    pub disabled: bool,
                    pub for_account: std::string::String,
                    pub uses: Vec<crate::com::atproto::server::defs::InviteCodeUse>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCodeUse {
                    pub used_at: atmo::datetime::DateTimeString,
                    pub used_by: atmo::did::Did,
                }
            }
            pub mod delete_account {}
            pub mod delete_session {}
            pub mod describe_server {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Contact {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<url::Url>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<url::Url>,
                }
            }
            pub mod get_account_invite_codes {}
            pub mod get_service_auth {}
            pub mod get_session {}
            pub mod list_app_passwords {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    pub created_at: atmo::datetime::DateTimeString,
                    pub name: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
            }
            pub mod refresh_session {}
            pub mod request_account_delete {}
            pub mod request_email_confirmation {}
            pub mod request_email_update {}
            pub mod request_password_reset {}
            pub mod reserve_signing_key {}
            pub mod reset_password {}
            pub mod revoke_app_password {}
            pub mod update_email {}
        }
        pub mod sync {
            pub mod get_blob {}
            pub mod get_blocks {}
            pub mod get_checkout {}
            pub mod get_head {}
            pub mod get_latest_commit {}
            pub mod get_record {}
            pub mod get_repo {}
            pub mod get_repo_status {}
            pub mod list_blobs {}
            pub mod list_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Repo {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo::did::Did,
                    pub head: atmo::CidString,
                    pub rev: std::string::String,
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
            pub mod notify_of_update {}
            pub mod request_crawl {}
            pub mod subscribe_repos {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Account {
                    #[doc = "Indicates that the account has a repository which can be fetched from the host that emitted this event."]
                    pub active: bool,
                    pub did: atmo::did::Did,
                    pub seq: i64,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::sync::subscribe_repos::Status>,
                    pub time: atmo::datetime::DateTimeString,
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
                    pub blobs: Vec<atmo::CidLink>,
                    #[doc = "CAR file containing relevant blocks, as a diff since the previous repo state."]
                    pub blocks: Vec<u8>,
                    pub commit: atmo::CidLink,
                    pub ops: Vec<crate::com::atproto::sync::subscribe_repos::RepoOp>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prev: std::option::Option<atmo::Nullable<atmo::CidLink>>,
                    #[doc = "DEPRECATED -- unused"]
                    pub rebase: bool,
                    pub repo: atmo::did::Did,
                    pub rev: std::string::String,
                    pub seq: i64,
                    pub since: atmo::Nullable<std::string::String>,
                    pub time: atmo::datetime::DateTimeString,
                    #[doc = "Indicates that this commit contained too many ops, or data size was too large. Consumers will need to make a separate request to get missing data."]
                    pub too_big: bool,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Handle {
                    pub did: atmo::did::Did,
                    pub handle: atmo::handle::Handle,
                    pub seq: i64,
                    pub time: atmo::datetime::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Identity {
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub handle: std::option::Option<atmo::handle::Handle>,
                    pub seq: i64,
                    pub time: atmo::datetime::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::sync::subscribe_repos::Name,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Migrate {
                    pub did: atmo::did::Did,
                    pub migrate_to: atmo::Nullable<std::string::String>,
                    pub seq: i64,
                    pub time: atmo::datetime::DateTimeString,
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
                    pub cid: atmo::Nullable<atmo::CidLink>,
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
                    pub did: atmo::did::Did,
                    pub seq: i64,
                    pub time: atmo::datetime::DateTimeString,
                }
            }
        }
        pub mod temp {
            pub mod check_signup_queue {}
            pub mod fetch_labels {}
            pub mod request_phone_verification {}
        }
    }
}
pub mod tools {
    pub mod ozone {
        pub mod communication {
            pub mod create_template {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct TemplateView {
                    pub content_markdown: std::string::String,
                    pub created_at: atmo::datetime::DateTimeString,
                    pub disabled: bool,
                    pub id: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    pub last_updated_by: atmo::did::Did,
                    pub name: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                    pub updated_at: atmo::datetime::DateTimeString,
                }
            }
            pub mod delete_template {}
            pub mod list_templates {}
            pub mod update_template {}
        }
        pub mod moderation {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct BlobView {
                    pub cid: atmo::CidString,
                    pub created_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub details:
                        std::option::Option<crate::tools::ozone::moderation::defs::Details>,
                    pub mime_type: std::string::String,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub moderation:
                        std::option::Option<crate::tools::ozone::moderation::defs::Moderation>,
                    pub size: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Details {
                    ImageDetails(crate::tools::ozone::moderation::defs::ImageDetails),
                    VideoDetails(crate::tools::ozone::moderation::defs::VideoDetails),
                    #[serde(other)]
                    Other,
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ImageDetails {
                    pub height: i64,
                    pub width: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventAcknowledge {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventComment {
                    pub comment: std::string::String,
                    #[doc = "Make the comment persistent on the subject"]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sticky: std::option::Option<bool>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventDivert {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEmail {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub content: std::option::Option<std::string::String>,
                    pub subject_line: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEscalate {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventLabel {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub create_label_vals: Vec<std::string::String>,
                    pub negate_label_vals: Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMute {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub duration_in_hours: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMuteReporter {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub duration_in_hours: i64,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReport {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[doc = "Set to true if the reporter was muted from reporting at the time of the event. These reports won't impact the reviewState of the subject."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub is_reporter_muted: std::option::Option<bool>,
                    pub report_type: crate::com::atproto::moderation::defs::ReasonType,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventResolveAppeal {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReverseTakedown {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTag {
                    pub add: Vec<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub remove: Vec<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTakedown {
                    #[doc = "If true, all other reports on content authored by this account will be resolved (acknowledged)."]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub acknowledge_account_subjects: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub duration_in_hours: std::option::Option<i64>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmute {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmuteReporter {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventView {
                    pub created_at: atmo::datetime::DateTimeString,
                    pub created_by: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub creator_handle: std::option::Option<std::string::String>,
                    pub event: crate::tools::ozone::moderation::defs::Event,
                    pub id: i64,
                    pub subject: crate::tools::ozone::moderation::defs::Subject,
                    pub subject_blob_cids: Vec<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_handle: std::option::Option<std::string::String>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventViewDetail {
                    pub created_at: atmo::datetime::DateTimeString,
                    pub created_by: atmo::did::Did,
                    pub event: crate::tools::ozone::moderation::defs::ModEventViewDetailEvent,
                    pub id: i64,
                    pub subject: crate::tools::ozone::moderation::defs::ModEventViewDetailSubject,
                    pub subject_blobs: Vec<crate::tools::ozone::moderation::defs::BlobView>,
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
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum ModEventViewDetailSubject {
                    RecordView(crate::tools::ozone::moderation::defs::RecordView),
                    RecordViewNotFound(crate::tools::ozone::moderation::defs::RecordViewNotFound),
                    RepoView(crate::tools::ozone::moderation::defs::RepoView),
                    RepoViewNotFound(crate::tools::ozone::moderation::defs::RepoViewNotFound),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Moderation {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ModerationDetail {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordView {
                    pub blob_cids: Vec<std::string::String>,
                    pub cid: atmo::CidString,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    pub repo: crate::tools::ozone::moderation::defs::RepoView,
                    pub uri: atmo::at_uri::AtUri,
                    pub value: (),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewDetail {
                    pub blobs: Vec<crate::tools::ozone::moderation::defs::BlobView>,
                    pub cid: atmo::CidString,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    pub moderation: crate::tools::ozone::moderation::defs::ModerationDetail,
                    pub repo: crate::tools::ozone::moderation::defs::RepoView,
                    pub uri: atmo::at_uri::AtUri,
                    pub value: (),
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewNotFound {
                    pub uri: atmo::at_uri::AtUri,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoView {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    pub handle: atmo::handle::Handle,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    pub related_records: Vec<atmo::Unknown>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewDetail {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub handle: atmo::handle::Handle,
                    pub indexed_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites:
                        std::option::Option<Vec<crate::com::atproto::server::defs::InviteCode>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels: std::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
                    pub moderation: crate::tools::ozone::moderation::defs::ModerationDetail,
                    pub related_records: Vec<atmo::Unknown>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewNotFound {
                    pub did: atmo::did::Did,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum Subject {
                    Main(crate::com::atproto::repo::strong_ref::Main),
                    MessageRef(crate::chat::bsky::convo::defs::MessageRef),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    #[serde(other)]
                    Other,
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appealed: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub created_at: atmo::datetime::DateTimeString,
                    pub id: i64,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_appealed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reported_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_at: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_by: std::option::Option<atmo::did::Did>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_reporting_until: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_until: std::option::Option<atmo::datetime::DateTimeString>,
                    pub review_state: crate::tools::ozone::moderation::defs::SubjectReviewState,
                    pub subject: crate::tools::ozone::moderation::defs::SubjectStatusViewSubject,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_blob_cids: std::option::Option<Vec<std::string::String>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_repo_handle: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub suspend_until: std::option::Option<atmo::datetime::DateTimeString>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tags: std::option::Option<Vec<std::string::String>>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takendown: std::option::Option<bool>,
                    pub updated_at: atmo::datetime::DateTimeString,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub enum SubjectStatusViewSubject {
                    Main(crate::com::atproto::repo::strong_ref::Main),
                    RepoRef(crate::com::atproto::admin::defs::RepoRef),
                    #[serde(other)]
                    Other,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct VideoDetails {
                    pub height: i64,
                    pub length: i64,
                    pub width: i64,
                }
            }
            pub mod emit_event {}
            pub mod get_event {}
            pub mod get_record {}
            pub mod get_records {}
            pub mod get_repo {}
            pub mod get_repos {}
            pub mod query_events {}
            pub mod query_statuses {}
            pub mod search_repos {}
        }
        pub mod server {
            pub mod get_config {
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
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub url: std::option::Option<url::Url>,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerConfig {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub role: std::option::Option<crate::tools::ozone::server::get_config::Role>,
                }
            }
        }
        pub mod set {
            pub mod add_values {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Set {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                }
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SetView {
                    pub created_at: atmo::datetime::DateTimeString,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                    pub set_size: i64,
                    pub updated_at: atmo::datetime::DateTimeString,
                }
            }
            pub mod delete_set {}
            pub mod delete_values {}
            pub mod get_values {}
            pub mod query_sets {}
            pub mod upsert_set {}
        }
        pub mod signature {
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct SigDetail {
                    pub property: std::string::String,
                    pub value: std::string::String,
                }
            }
            pub mod find_correlation {}
            pub mod find_related_accounts {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct RelatedAccount {
                    pub account: crate::com::atproto::admin::defs::AccountView,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub similarities:
                        std::option::Option<Vec<crate::tools::ozone::signature::defs::SigDetail>>,
                }
            }
            pub mod search_accounts {}
        }
        pub mod team {
            pub mod add_member {}
            pub mod defs {
                #[derive(serde :: Deserialize, serde :: Serialize)]
                pub struct Member {
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo::datetime::DateTimeString>,
                    pub did: atmo::did::Did,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_updated_by: std::option::Option<std::string::String>,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub profile:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileViewDetailed>,
                    pub role: crate::tools::ozone::team::defs::Role,
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo::datetime::DateTimeString>,
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
            pub mod delete_member {}
            pub mod list_members {}
            pub mod update_member {}
        }
    }
}
