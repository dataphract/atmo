pub mod app {
    pub mod bsky {
        pub mod actor {
            #[derive(Debug)]
            pub struct GetPreferences;
            impl atmo_core::xrpc::Request for GetPreferences {
                type Params = crate::app::bsky::actor::get_preferences::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::get_preferences::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetProfile;
            impl atmo_core::xrpc::Request for GetProfile {
                type Params = crate::app::bsky::actor::get_profile::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::defs::ProfileViewDetailed;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetProfiles;
            impl atmo_core::xrpc::Request for GetProfiles {
                type Params = crate::app::bsky::actor::get_profiles::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::get_profiles::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSuggestions;
            impl atmo_core::xrpc::Request for GetSuggestions {
                type Params = crate::app::bsky::actor::get_suggestions::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::get_suggestions::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Profile {
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub avatar: std::option::Option<atmo_core::Blob>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub banner: std::option::Option<atmo_core::Blob>,
                #[serde(default)]
                #[serde(rename = "createdAt")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub created_at: std::option::Option<atmo_core::DateTimeString>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "displayName")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub display_name: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "joinedViaStarterPack")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub joined_via_starter_pack:
                    std::option::Option<crate::com::atproto::repo::StrongRef>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub labels: std::option::Option<crate::app::bsky::actor::profile::main::Labels>,
                #[serde(default)]
                #[serde(rename = "pinnedPost")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub pinned_post: std::option::Option<crate::com::atproto::repo::StrongRef>,
            }
            #[derive(Debug)]
            pub struct PutPreferences;
            impl atmo_core::xrpc::Request for PutPreferences {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::actor::put_preferences::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct SearchActors;
            impl atmo_core::xrpc::Request for SearchActors {
                type Params = crate::app::bsky::actor::search_actors::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::search_actors::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchActorsTypeahead;
            impl atmo_core::xrpc::Request for SearchActorsTypeahead {
                type Params = crate::app::bsky::actor::search_actors_typeahead::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::actor::search_actors_typeahead::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AdultContentPref {
                    pub enabled: bool,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BskyAppProgressGuide {
                    pub guide: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BskyAppStatePref {
                    #[serde(default)]
                    #[serde(rename = "activeProgressGuide")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active_progress_guide:
                        std::option::Option<crate::app::bsky::actor::defs::BskyAppProgressGuide>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub nuxs:
                        std::option::Option<std::vec::Vec<crate::app::bsky::actor::defs::Nux>>,
                    #[serde(default)]
                    #[serde(rename = "queuedNudges")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub queued_nudges: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ContentLabelPref {
                    pub label: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "labelerDid")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labeler_did: std::option::Option<atmo_core::Did>,
                    pub visibility: crate::app::bsky::actor::defs::content_label_pref::Visibility,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct FeedViewPref {
                    pub feed: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "hideQuotePosts")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_quote_posts: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "hideReplies")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "hideRepliesByLikeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_like_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "hideRepliesByUnfollowed")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_replies_by_unfollowed: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "hideReposts")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hide_reposts: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct HiddenPostsPref {
                    pub items: std::vec::Vec<atmo_core::AtUri>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct InterestsPref {
                    pub tags: std::vec::Vec<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct KnownFollowers {
                    pub count: i64,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPrefItem {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelersPref {
                    pub labelers: std::vec::Vec<crate::app::bsky::actor::defs::LabelerPrefItem>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MutedWord {
                    #[serde(default)]
                    #[serde(rename = "actorTarget")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub actor_target:
                        std::option::Option<crate::app::bsky::actor::defs::muted_word::ActorTarget>,
                    #[serde(default)]
                    #[serde(rename = "expiresAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub id: std::option::Option<std::string::String>,
                    pub targets: std::vec::Vec<crate::app::bsky::actor::defs::MutedWordTarget>,
                    pub value: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum MutedWordTarget {
                    #[serde(rename = "content")]
                    Content,
                    #[serde(rename = "tag")]
                    Tag,
                    #[serde(untagged)]
                    Other(String),
                }
                impl MutedWordTarget {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::Content => "content",
                            Self::Tag => "tag",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for MutedWordTarget {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MutedWordsPref {
                    pub items: std::vec::Vec<crate::app::bsky::actor::defs::MutedWord>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Nux {
                    pub completed: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub data: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "expiresAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub expires_at: std::option::Option<atmo_core::DateTimeString>,
                    pub id: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct PersonalDetailsPref {
                    #[serde(default)]
                    #[serde(rename = "birthDate")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub birth_date: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
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
                impl<'de> serde::Deserialize<'de> for Preferences {
                    fn deserialize<D>(des: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        use serde::de::Error as _;
                        if des.is_human_readable() {
                            let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                Default::default();
                            let union_ = des.deserialize_map(visitor)?;
                            let map_des = serde::de::value::MapDeserializer::new(
                                union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                            );
                            let res = match union_.ty.as_ref() {
                                "app.bsky.actor.defs#adultContentPref" => {
                                    crate::app::bsky::actor::defs::AdultContentPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::AdultContentPref)
                                }
                                "app.bsky.actor.defs#bskyAppStatePref" => {
                                    crate::app::bsky::actor::defs::BskyAppStatePref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::BskyAppStatePref)
                                }
                                "app.bsky.actor.defs#contentLabelPref" => {
                                    crate::app::bsky::actor::defs::ContentLabelPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::ContentLabelPref)
                                }
                                "app.bsky.actor.defs#feedViewPref" => {
                                    crate::app::bsky::actor::defs::FeedViewPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::FeedViewPref)
                                }
                                "app.bsky.actor.defs#hiddenPostsPref" => {
                                    crate::app::bsky::actor::defs::HiddenPostsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::HiddenPostsPref)
                                }
                                "app.bsky.actor.defs#interestsPref" => {
                                    crate::app::bsky::actor::defs::InterestsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::InterestsPref)
                                }
                                "app.bsky.actor.defs#labelersPref" => {
                                    crate::app::bsky::actor::defs::LabelersPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::LabelersPref)
                                }
                                "app.bsky.actor.defs#mutedWordsPref" => {
                                    crate::app::bsky::actor::defs::MutedWordsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::MutedWordsPref)
                                }
                                "app.bsky.actor.defs#personalDetailsPref" => {
                                    crate::app::bsky::actor::defs::PersonalDetailsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::PersonalDetailsPref)
                                }
                                "app.bsky.actor.defs#savedFeedsPref" => {
                                    crate::app::bsky::actor::defs::SavedFeedsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::SavedFeedsPref)
                                }
                                "app.bsky.actor.defs#savedFeedsPrefV2" => {
                                    crate::app::bsky::actor::defs::SavedFeedsPrefV2::deserialize(
                                        map_des,
                                    )
                                    .map(Self::SavedFeedsPrefV2)
                                }
                                "app.bsky.actor.defs#threadViewPref" => {
                                    crate::app::bsky::actor::defs::ThreadViewPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::ThreadViewPref)
                                }
                                _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                            };
                            res.map_err(D::Error::custom)
                        } else {
                            let visitor: atmo_core::union_::UnionVisitor<ipld_core::ipld::Ipld> =
                                Default::default();
                            let union_ = des.deserialize_map(visitor)?;
                            let map_des = serde::de::value::MapDeserializer::new(
                                union_.map.iter().map(|(k, v)| {
                                    (
                                        k.as_ref(),
                                        atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                    )
                                }),
                            );
                            let res = match union_.ty.as_ref() {
                                "app.bsky.actor.defs#adultContentPref" => {
                                    crate::app::bsky::actor::defs::AdultContentPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::AdultContentPref)
                                }
                                "app.bsky.actor.defs#bskyAppStatePref" => {
                                    crate::app::bsky::actor::defs::BskyAppStatePref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::BskyAppStatePref)
                                }
                                "app.bsky.actor.defs#contentLabelPref" => {
                                    crate::app::bsky::actor::defs::ContentLabelPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::ContentLabelPref)
                                }
                                "app.bsky.actor.defs#feedViewPref" => {
                                    crate::app::bsky::actor::defs::FeedViewPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::FeedViewPref)
                                }
                                "app.bsky.actor.defs#hiddenPostsPref" => {
                                    crate::app::bsky::actor::defs::HiddenPostsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::HiddenPostsPref)
                                }
                                "app.bsky.actor.defs#interestsPref" => {
                                    crate::app::bsky::actor::defs::InterestsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::InterestsPref)
                                }
                                "app.bsky.actor.defs#labelersPref" => {
                                    crate::app::bsky::actor::defs::LabelersPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::LabelersPref)
                                }
                                "app.bsky.actor.defs#mutedWordsPref" => {
                                    crate::app::bsky::actor::defs::MutedWordsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::MutedWordsPref)
                                }
                                "app.bsky.actor.defs#personalDetailsPref" => {
                                    crate::app::bsky::actor::defs::PersonalDetailsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::PersonalDetailsPref)
                                }
                                "app.bsky.actor.defs#savedFeedsPref" => {
                                    crate::app::bsky::actor::defs::SavedFeedsPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::SavedFeedsPref)
                                }
                                "app.bsky.actor.defs#savedFeedsPrefV2" => {
                                    crate::app::bsky::actor::defs::SavedFeedsPrefV2::deserialize(
                                        map_des,
                                    )
                                    .map(Self::SavedFeedsPrefV2)
                                }
                                "app.bsky.actor.defs#threadViewPref" => {
                                    crate::app::bsky::actor::defs::ThreadViewPref::deserialize(
                                        map_des,
                                    )
                                    .map(Self::ThreadViewPref)
                                }
                                _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                            };
                            res.map_err(D::Error::custom)
                        }
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    #[serde(rename = "starterPacks")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub starter_packs: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileAssociatedChat {
                    #[serde(rename = "allowIncoming")]
                    pub allow_incoming:
                        crate::app::bsky::actor::defs::profile_associated_chat::AllowIncoming,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "displayName")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(rename = "indexedAt")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "displayName")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "displayName")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub display_name: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "followersCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followers_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "followsCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub follows_count: std::option::Option<i64>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(rename = "indexedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "joinedViaStarterPack")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_via_starter_pack:
                        std::option::Option<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "pinnedPost")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned_post: std::option::Option<crate::com::atproto::repo::StrongRef>,
                    #[serde(default)]
                    #[serde(rename = "postsCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub posts_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeed {
                    pub id: std::string::String,
                    pub pinned: bool,
                    #[serde(rename = "type")]
                    pub ty: crate::app::bsky::actor::defs::saved_feed::Type,
                    pub value: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeedsPref {
                    pub pinned: std::vec::Vec<atmo_core::AtUri>,
                    pub saved: std::vec::Vec<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(rename = "timelineIndex")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub timeline_index: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SavedFeedsPrefV2 {
                    pub items: std::vec::Vec<crate::app::bsky::actor::defs::SavedFeed>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadViewPref {
                    #[serde(default)]
                    #[serde(rename = "prioritizeFollowedUsers")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prioritize_followed_users: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort:
                        std::option::Option<crate::app::bsky::actor::defs::thread_view_pref::Sort>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerState {
                    #[serde(default)]
                    #[serde(rename = "blockedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked_by: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(rename = "blockingByList")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocking_by_list:
                        std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                    #[serde(default)]
                    #[serde(rename = "followedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(rename = "knownFollowers")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub known_followers:
                        std::option::Option<crate::app::bsky::actor::defs::KnownFollowers>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "mutedByList")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted_by_list:
                        std::option::Option<crate::app::bsky::graph::defs::ListViewBasic>,
                }
                pub mod content_label_pref {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Visibility {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Ignore => "ignore",
                                Self::Show => "show",
                                Self::Warn => "warn",
                                Self::Hide => "hide",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Visibility {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod muted_word {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ActorTarget {
                        #[serde(rename = "all")]
                        All,
                        #[serde(rename = "exclude-following")]
                        ExcludeFollowing,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ActorTarget {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::All => "all",
                                Self::ExcludeFollowing => "exclude-following",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ActorTarget {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod profile_associated_chat {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl AllowIncoming {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::All => "all",
                                Self::None => "none",
                                Self::Following => "following",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for AllowIncoming {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod saved_feed {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Type {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Feed => "feed",
                                Self::List => "list",
                                Self::Timeline => "timeline",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Type {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod thread_view_pref {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Sort {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Oldest => "oldest",
                                Self::Newest => "newest",
                                Self::MostLikes => "most-likes",
                                Self::Random => "random",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Sort {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod get_preferences {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub preferences: std::vec::Vec<crate::app::bsky::actor::defs::Preferences>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {}
            }
            pub mod get_profile {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod get_profiles {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub profiles: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewDetailed>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actors: std::vec::Vec<atmo_core::AtIdentifier>,
                }
            }
            pub mod get_suggestions {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
            }
            pub mod profile {
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Labels {
                        SelfLabels(crate::com::atproto::label::defs::SelfLabels),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Labels {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod put_preferences {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub preferences: std::vec::Vec<crate::app::bsky::actor::defs::Preferences>,
                }
            }
            pub mod search_actors {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors: std::vec::Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct External {
                pub external: crate::app::bsky::embed::external::External,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Images {
                pub images: std::vec::Vec<crate::app::bsky::embed::images::Image>,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Record {
                pub record: crate::com::atproto::repo::StrongRef,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct RecordWithMedia {
                pub media: crate::app::bsky::embed::record_with_media::main::Media,
                pub record: crate::app::bsky::embed::Record,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Video {
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub alt: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "aspectRatio")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub aspect_ratio: std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub captions:
                    std::option::Option<std::vec::Vec<crate::app::bsky::embed::video::Caption>>,
                pub video: atmo_core::Blob,
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AspectRatio {
                    pub height: i64,
                    pub width: i64,
                }
            }
            pub mod external {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct External {
                    pub description: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thumb: std::option::Option<atmo_core::Blob>,
                    pub title: std::string::String,
                    pub uri: url::Url,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub external: crate::app::bsky::embed::external::ViewExternal,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Image {
                    pub alt: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "aspectRatio")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub image: atmo_core::Blob,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub images: std::vec::Vec<crate::app::bsky::embed::images::ViewImage>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewImage {
                    pub alt: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "aspectRatio")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub aspect_ratio:
                        std::option::Option<crate::app::bsky::embed::defs::AspectRatio>,
                    pub fullsize: url::Url,
                    pub thumb: url::Url,
                }
            }
            pub mod record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub record: crate::app::bsky::embed::record::view::Record,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewBlocked {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewDetached {
                    pub detached: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewNotFound {
                    #[serde(rename = "notFound")]
                    pub not_found: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewRecord {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embeds: std::option::Option<
                        std::vec::Vec<crate::app::bsky::embed::record::view_record::Embeds>,
                    >,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "likeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "quoteCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "replyCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "repostCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                pub mod view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
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
                    impl<'de> serde::Deserialize<'de> for Record {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.defs#generatorView" => crate :: app :: bsky :: feed :: defs :: GeneratorView :: deserialize (map_des) . map (Self :: GeneratorView) , "app.bsky.labeler.defs#labelerView" => crate :: app :: bsky :: labeler :: defs :: LabelerView :: deserialize (map_des) . map (Self :: LabelerView) , "app.bsky.graph.defs#listView" => crate :: app :: bsky :: graph :: defs :: ListView :: deserialize (map_des) . map (Self :: ListView) , "app.bsky.graph.defs#starterPackViewBasic" => crate :: app :: bsky :: graph :: defs :: StarterPackViewBasic :: deserialize (map_des) . map (Self :: StarterPackViewBasic) , "app.bsky.embed.record#viewBlocked" => crate :: app :: bsky :: embed :: record :: ViewBlocked :: deserialize (map_des) . map (Self :: ViewBlocked) , "app.bsky.embed.record#viewDetached" => crate :: app :: bsky :: embed :: record :: ViewDetached :: deserialize (map_des) . map (Self :: ViewDetached) , "app.bsky.embed.record#viewNotFound" => crate :: app :: bsky :: embed :: record :: ViewNotFound :: deserialize (map_des) . map (Self :: ViewNotFound) , "app.bsky.embed.record#viewRecord" => crate :: app :: bsky :: embed :: record :: ViewRecord :: deserialize (map_des) . map (Self :: ViewRecord) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.defs#generatorView" => crate :: app :: bsky :: feed :: defs :: GeneratorView :: deserialize (map_des) . map (Self :: GeneratorView) , "app.bsky.labeler.defs#labelerView" => crate :: app :: bsky :: labeler :: defs :: LabelerView :: deserialize (map_des) . map (Self :: LabelerView) , "app.bsky.graph.defs#listView" => crate :: app :: bsky :: graph :: defs :: ListView :: deserialize (map_des) . map (Self :: ListView) , "app.bsky.graph.defs#starterPackViewBasic" => crate :: app :: bsky :: graph :: defs :: StarterPackViewBasic :: deserialize (map_des) . map (Self :: StarterPackViewBasic) , "app.bsky.embed.record#viewBlocked" => crate :: app :: bsky :: embed :: record :: ViewBlocked :: deserialize (map_des) . map (Self :: ViewBlocked) , "app.bsky.embed.record#viewDetached" => crate :: app :: bsky :: embed :: record :: ViewDetached :: deserialize (map_des) . map (Self :: ViewDetached) , "app.bsky.embed.record#viewNotFound" => crate :: app :: bsky :: embed :: record :: ViewNotFound :: deserialize (map_des) . map (Self :: ViewNotFound) , "app.bsky.embed.record#viewRecord" => crate :: app :: bsky :: embed :: record :: ViewRecord :: deserialize (map_des) . map (Self :: ViewRecord) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod view_record {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Embeds {
                        ExternalView(crate::app::bsky::embed::external::View),
                        ImagesView(crate::app::bsky::embed::images::View),
                        RecordView(crate::app::bsky::embed::record::View),
                        RecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
                        VideoView(crate::app::bsky::embed::video::View),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Embeds {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.embed.external#view" => crate :: app :: bsky :: embed :: external :: View :: deserialize (map_des) . map (Self :: ExternalView) , "app.bsky.embed.images#view" => crate :: app :: bsky :: embed :: images :: View :: deserialize (map_des) . map (Self :: ImagesView) , "app.bsky.embed.record#view" => crate :: app :: bsky :: embed :: record :: View :: deserialize (map_des) . map (Self :: RecordView) , "app.bsky.embed.recordWithMedia#view" => crate :: app :: bsky :: embed :: record_with_media :: View :: deserialize (map_des) . map (Self :: RecordWithMediaView) , "app.bsky.embed.video#view" => crate :: app :: bsky :: embed :: video :: View :: deserialize (map_des) . map (Self :: VideoView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.embed.external#view" => crate :: app :: bsky :: embed :: external :: View :: deserialize (map_des) . map (Self :: ExternalView) , "app.bsky.embed.images#view" => crate :: app :: bsky :: embed :: images :: View :: deserialize (map_des) . map (Self :: ImagesView) , "app.bsky.embed.record#view" => crate :: app :: bsky :: embed :: record :: View :: deserialize (map_des) . map (Self :: RecordView) , "app.bsky.embed.recordWithMedia#view" => crate :: app :: bsky :: embed :: record_with_media :: View :: deserialize (map_des) . map (Self :: RecordWithMediaView) , "app.bsky.embed.video#view" => crate :: app :: bsky :: embed :: video :: View :: deserialize (map_des) . map (Self :: VideoView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod record_with_media {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    pub media: crate::app::bsky::embed::record_with_media::view::Media,
                    pub record: crate::app::bsky::embed::record::View,
                }
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Media {
                        External(crate::app::bsky::embed::External),
                        Images(crate::app::bsky::embed::Images),
                        Video(crate::app::bsky::embed::Video),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Media {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external" => {
                                        crate::app::bsky::embed::External::deserialize(map_des)
                                            .map(Self::External)
                                    }
                                    "app.bsky.embed.images" => {
                                        crate::app::bsky::embed::Images::deserialize(map_des)
                                            .map(Self::Images)
                                    }
                                    "app.bsky.embed.video" => {
                                        crate::app::bsky::embed::Video::deserialize(map_des)
                                            .map(Self::Video)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external" => {
                                        crate::app::bsky::embed::External::deserialize(map_des)
                                            .map(Self::External)
                                    }
                                    "app.bsky.embed.images" => {
                                        crate::app::bsky::embed::Images::deserialize(map_des)
                                            .map(Self::Images)
                                    }
                                    "app.bsky.embed.video" => {
                                        crate::app::bsky::embed::Video::deserialize(map_des)
                                            .map(Self::Video)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Media {
                        ExternalView(crate::app::bsky::embed::external::View),
                        ImagesView(crate::app::bsky::embed::images::View),
                        VideoView(crate::app::bsky::embed::video::View),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Media {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external#view" => {
                                        crate::app::bsky::embed::external::View::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ExternalView)
                                    }
                                    "app.bsky.embed.images#view" => {
                                        crate::app::bsky::embed::images::View::deserialize(map_des)
                                            .map(Self::ImagesView)
                                    }
                                    "app.bsky.embed.video#view" => {
                                        crate::app::bsky::embed::video::View::deserialize(map_des)
                                            .map(Self::VideoView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external#view" => {
                                        crate::app::bsky::embed::external::View::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ExternalView)
                                    }
                                    "app.bsky.embed.images#view" => {
                                        crate::app::bsky::embed::images::View::deserialize(map_des)
                                            .map(Self::ImagesView)
                                    }
                                    "app.bsky.embed.video#view" => {
                                        crate::app::bsky::embed::video::View::deserialize(map_des)
                                            .map(Self::VideoView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod video {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Caption {
                    pub file: atmo_core::Blob,
                    pub lang: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct View {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub alt: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "aspectRatio")]
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
            #[derive(Debug)]
            pub struct DescribeFeedGenerator;
            impl atmo_core::xrpc::Request for DescribeFeedGenerator {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::describe_feed_generator::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Generator {
                #[serde(default)]
                #[serde(rename = "acceptsInteractions")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub accepts_interactions: std::option::Option<bool>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub avatar: std::option::Option<atmo_core::Blob>,
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "descriptionFacets")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description_facets:
                    std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                pub did: atmo_core::Did,
                #[serde(rename = "displayName")]
                pub display_name: std::string::String,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub labels: std::option::Option<crate::app::bsky::feed::generator::main::Labels>,
            }
            #[derive(Debug)]
            pub struct GetActorFeeds;
            impl atmo_core::xrpc::Request for GetActorFeeds {
                type Params = crate::app::bsky::feed::get_actor_feeds::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_actor_feeds::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetActorLikes;
            impl atmo_core::xrpc::Request for GetActorLikes {
                type Params = crate::app::bsky::feed::get_actor_likes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_actor_likes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetAuthorFeed;
            impl atmo_core::xrpc::Request for GetAuthorFeed {
                type Params = crate::app::bsky::feed::get_author_feed::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_author_feed::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFeed;
            impl atmo_core::xrpc::Request for GetFeed {
                type Params = crate::app::bsky::feed::get_feed::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_feed::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFeedGenerator;
            impl atmo_core::xrpc::Request for GetFeedGenerator {
                type Params = crate::app::bsky::feed::get_feed_generator::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_feed_generator::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFeedGenerators;
            impl atmo_core::xrpc::Request for GetFeedGenerators {
                type Params = crate::app::bsky::feed::get_feed_generators::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_feed_generators::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFeedSkeleton;
            impl atmo_core::xrpc::Request for GetFeedSkeleton {
                type Params = crate::app::bsky::feed::get_feed_skeleton::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_feed_skeleton::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetLikes;
            impl atmo_core::xrpc::Request for GetLikes {
                type Params = crate::app::bsky::feed::get_likes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_likes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetListFeed;
            impl atmo_core::xrpc::Request for GetListFeed {
                type Params = crate::app::bsky::feed::get_list_feed::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_list_feed::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetPostThread;
            impl atmo_core::xrpc::Request for GetPostThread {
                type Params = crate::app::bsky::feed::get_post_thread::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_post_thread::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetPosts;
            impl atmo_core::xrpc::Request for GetPosts {
                type Params = crate::app::bsky::feed::get_posts::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_posts::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetQuotes;
            impl atmo_core::xrpc::Request for GetQuotes {
                type Params = crate::app::bsky::feed::get_quotes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_quotes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRepostedBy;
            impl atmo_core::xrpc::Request for GetRepostedBy {
                type Params = crate::app::bsky::feed::get_reposted_by::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_reposted_by::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSuggestedFeeds;
            impl atmo_core::xrpc::Request for GetSuggestedFeeds {
                type Params = crate::app::bsky::feed::get_suggested_feeds::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_suggested_feeds::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetTimeline;
            impl atmo_core::xrpc::Request for GetTimeline {
                type Params = crate::app::bsky::feed::get_timeline::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::get_timeline::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Like {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub subject: crate::com::atproto::repo::StrongRef,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Post {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub embed: std::option::Option<crate::app::bsky::feed::post::main::Embed>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub entities:
                    std::option::Option<std::vec::Vec<crate::app::bsky::feed::post::Entity>>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub facets: std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub labels: std::option::Option<crate::app::bsky::feed::post::main::Labels>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub langs: std::option::Option<std::vec::Vec<std::string::String>>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub reply: std::option::Option<crate::app::bsky::feed::post::ReplyRef>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub tags: std::option::Option<std::vec::Vec<std::string::String>>,
                pub text: std::string::String,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Postgate {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(rename = "detachedEmbeddingUris")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub detached_embedding_uris: std::option::Option<std::vec::Vec<atmo_core::AtUri>>,
                #[serde(default)]
                #[serde(rename = "embeddingRules")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub embedding_rules: std::option::Option<
                    std::vec::Vec<crate::app::bsky::feed::postgate::main::EmbeddingRules>,
                >,
                pub post: atmo_core::AtUri,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Repost {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub subject: crate::com::atproto::repo::StrongRef,
            }
            #[derive(Debug)]
            pub struct SearchPosts;
            impl atmo_core::xrpc::Request for SearchPosts {
                type Params = crate::app::bsky::feed::search_posts::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::feed::search_posts::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SendInteractions;
            impl atmo_core::xrpc::Request for SendInteractions {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::feed::send_interactions::Input;
                type Output = crate::app::bsky::feed::send_interactions::Output;
                type Error = String;
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Threadgate {
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub allow: std::option::Option<
                    std::vec::Vec<crate::app::bsky::feed::threadgate::main::Allow>,
                >,
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(rename = "hiddenReplies")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub hidden_replies: std::option::Option<std::vec::Vec<atmo_core::AtUri>>,
                pub post: atmo_core::AtUri,
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedAuthor {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::actor::defs::ViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BlockedPost {
                    pub author: crate::app::bsky::feed::defs::BlockedAuthor,
                    pub blocked: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct FeedViewPost {
                    #[serde(default)]
                    #[serde(rename = "feedContext")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason:
                        std::option::Option<crate::app::bsky::feed::defs::feed_view_post::Reason>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply: std::option::Option<crate::app::bsky::feed::defs::ReplyRef>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorView {
                    #[serde(default)]
                    #[serde(rename = "acceptsInteractions")]
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
                    #[serde(rename = "descriptionFacets")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub did: atmo_core::Did,
                    #[serde(rename = "displayName")]
                    pub display_name: std::string::String,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "likeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::feed::defs::GeneratorViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct GeneratorViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Interaction {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub event:
                        std::option::Option<crate::app::bsky::feed::defs::interaction::Event>,
                    #[serde(default)]
                    #[serde(rename = "feedContext")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub item: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundPost {
                    #[serde(rename = "notFound")]
                    pub not_found: bool,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct PostView {
                    pub author: crate::app::bsky::actor::defs::ProfileViewBasic,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed: std::option::Option<crate::app::bsky::feed::defs::post_view::Embed>,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "likeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "quoteCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub quote_count: std::option::Option<i64>,
                    pub record: atmo_core::Unknown,
                    #[serde(default)]
                    #[serde(rename = "replyCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "repostCount")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonPin {}
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ReasonRepost {
                    pub by: crate::app::bsky::actor::defs::ProfileViewBasic,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
                    #[serde(default)]
                    #[serde(rename = "grandparentAuthor")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub grandparent_author:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileViewBasic>,
                    pub parent: crate::app::bsky::feed::defs::reply_ref::Parent,
                    pub root: crate::app::bsky::feed::defs::reply_ref::Root,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonFeedPost {
                    #[serde(default)]
                    #[serde(rename = "feedContext")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feed_context: std::option::Option<std::string::String>,
                    pub post: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<
                        crate::app::bsky::feed::defs::skeleton_feed_post::Reason,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonPin {}
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonReasonRepost {
                    pub repost: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ThreadViewPost {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub parent:
                        std::option::Option<crate::app::bsky::feed::defs::thread_view_post::Parent>,
                    pub post: crate::app::bsky::feed::defs::PostView,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub replies: std::option::Option<
                        std::vec::Vec<crate::app::bsky::feed::defs::thread_view_post::Replies>,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerState {
                    #[serde(default)]
                    #[serde(rename = "embeddingDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embedding_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pinned: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "replyDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reply_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub repost: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(rename = "threadMuted")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub thread_muted: std::option::Option<bool>,
                }
                pub mod feed_view_post {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Reason {
                        ReasonPin(crate::app::bsky::feed::defs::ReasonPin),
                        ReasonRepost(crate::app::bsky::feed::defs::ReasonRepost),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Reason {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#reasonPin" => {
                                        crate::app::bsky::feed::defs::ReasonPin::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ReasonPin)
                                    }
                                    "app.bsky.feed.defs#reasonRepost" => {
                                        crate::app::bsky::feed::defs::ReasonRepost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ReasonRepost)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#reasonPin" => {
                                        crate::app::bsky::feed::defs::ReasonPin::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ReasonPin)
                                    }
                                    "app.bsky.feed.defs#reasonRepost" => {
                                        crate::app::bsky::feed::defs::ReasonRepost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ReasonRepost)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod interaction {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Event {
                        #[serde(rename = "app.bsky.feed.defs#requestLess")]
                        AppBskyFeedDefsRequestLess,
                        #[serde(rename = "app.bsky.feed.defs#requestMore")]
                        AppBskyFeedDefsRequestMore,
                        #[serde(rename = "app.bsky.feed.defs#clickthroughItem")]
                        AppBskyFeedDefsClickthroughItem,
                        #[serde(rename = "app.bsky.feed.defs#clickthroughAuthor")]
                        AppBskyFeedDefsClickthroughAuthor,
                        #[serde(rename = "app.bsky.feed.defs#clickthroughReposter")]
                        AppBskyFeedDefsClickthroughReposter,
                        #[serde(rename = "app.bsky.feed.defs#clickthroughEmbed")]
                        AppBskyFeedDefsClickthroughEmbed,
                        #[serde(rename = "app.bsky.feed.defs#interactionSeen")]
                        AppBskyFeedDefsInteractionSeen,
                        #[serde(rename = "app.bsky.feed.defs#interactionLike")]
                        AppBskyFeedDefsInteractionLike,
                        #[serde(rename = "app.bsky.feed.defs#interactionRepost")]
                        AppBskyFeedDefsInteractionRepost,
                        #[serde(rename = "app.bsky.feed.defs#interactionReply")]
                        AppBskyFeedDefsInteractionReply,
                        #[serde(rename = "app.bsky.feed.defs#interactionQuote")]
                        AppBskyFeedDefsInteractionQuote,
                        #[serde(rename = "app.bsky.feed.defs#interactionShare")]
                        AppBskyFeedDefsInteractionShare,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Event {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::AppBskyFeedDefsRequestLess => {
                                    "app.bsky.feed.defs#requestLess"
                                }
                                Self::AppBskyFeedDefsRequestMore => {
                                    "app.bsky.feed.defs#requestMore"
                                }
                                Self::AppBskyFeedDefsClickthroughItem => {
                                    "app.bsky.feed.defs#clickthroughItem"
                                }
                                Self::AppBskyFeedDefsClickthroughAuthor => {
                                    "app.bsky.feed.defs#clickthroughAuthor"
                                }
                                Self::AppBskyFeedDefsClickthroughReposter => {
                                    "app.bsky.feed.defs#clickthroughReposter"
                                }
                                Self::AppBskyFeedDefsClickthroughEmbed => {
                                    "app.bsky.feed.defs#clickthroughEmbed"
                                }
                                Self::AppBskyFeedDefsInteractionSeen => {
                                    "app.bsky.feed.defs#interactionSeen"
                                }
                                Self::AppBskyFeedDefsInteractionLike => {
                                    "app.bsky.feed.defs#interactionLike"
                                }
                                Self::AppBskyFeedDefsInteractionRepost => {
                                    "app.bsky.feed.defs#interactionRepost"
                                }
                                Self::AppBskyFeedDefsInteractionReply => {
                                    "app.bsky.feed.defs#interactionReply"
                                }
                                Self::AppBskyFeedDefsInteractionQuote => {
                                    "app.bsky.feed.defs#interactionQuote"
                                }
                                Self::AppBskyFeedDefsInteractionShare => {
                                    "app.bsky.feed.defs#interactionShare"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Event {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod post_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Embed {
                        ExternalView(crate::app::bsky::embed::external::View),
                        ImagesView(crate::app::bsky::embed::images::View),
                        RecordView(crate::app::bsky::embed::record::View),
                        RecordWithMediaView(crate::app::bsky::embed::record_with_media::View),
                        VideoView(crate::app::bsky::embed::video::View),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Embed {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.embed.external#view" => crate :: app :: bsky :: embed :: external :: View :: deserialize (map_des) . map (Self :: ExternalView) , "app.bsky.embed.images#view" => crate :: app :: bsky :: embed :: images :: View :: deserialize (map_des) . map (Self :: ImagesView) , "app.bsky.embed.record#view" => crate :: app :: bsky :: embed :: record :: View :: deserialize (map_des) . map (Self :: RecordView) , "app.bsky.embed.recordWithMedia#view" => crate :: app :: bsky :: embed :: record_with_media :: View :: deserialize (map_des) . map (Self :: RecordWithMediaView) , "app.bsky.embed.video#view" => crate :: app :: bsky :: embed :: video :: View :: deserialize (map_des) . map (Self :: VideoView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.embed.external#view" => crate :: app :: bsky :: embed :: external :: View :: deserialize (map_des) . map (Self :: ExternalView) , "app.bsky.embed.images#view" => crate :: app :: bsky :: embed :: images :: View :: deserialize (map_des) . map (Self :: ImagesView) , "app.bsky.embed.record#view" => crate :: app :: bsky :: embed :: record :: View :: deserialize (map_des) . map (Self :: RecordView) , "app.bsky.embed.recordWithMedia#view" => crate :: app :: bsky :: embed :: record_with_media :: View :: deserialize (map_des) . map (Self :: RecordWithMediaView) , "app.bsky.embed.video#view" => crate :: app :: bsky :: embed :: video :: View :: deserialize (map_des) . map (Self :: VideoView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod reply_ref {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Parent {
                        BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                        NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                        PostView(crate::app::bsky::feed::defs::PostView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Parent {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#postView" => {
                                        crate::app::bsky::feed::defs::PostView::deserialize(map_des)
                                            .map(Self::PostView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#postView" => {
                                        crate::app::bsky::feed::defs::PostView::deserialize(map_des)
                                            .map(Self::PostView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Root {
                        BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                        NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                        PostView(crate::app::bsky::feed::defs::PostView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Root {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#postView" => {
                                        crate::app::bsky::feed::defs::PostView::deserialize(map_des)
                                            .map(Self::PostView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#postView" => {
                                        crate::app::bsky::feed::defs::PostView::deserialize(map_des)
                                            .map(Self::PostView)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod skeleton_feed_post {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Reason {
                        SkeletonReasonPin(crate::app::bsky::feed::defs::SkeletonReasonPin),
                        SkeletonReasonRepost(crate::app::bsky::feed::defs::SkeletonReasonRepost),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Reason {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.defs#skeletonReasonPin" => crate :: app :: bsky :: feed :: defs :: SkeletonReasonPin :: deserialize (map_des) . map (Self :: SkeletonReasonPin) , "app.bsky.feed.defs#skeletonReasonRepost" => crate :: app :: bsky :: feed :: defs :: SkeletonReasonRepost :: deserialize (map_des) . map (Self :: SkeletonReasonRepost) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.defs#skeletonReasonPin" => crate :: app :: bsky :: feed :: defs :: SkeletonReasonPin :: deserialize (map_des) . map (Self :: SkeletonReasonPin) , "app.bsky.feed.defs#skeletonReasonRepost" => crate :: app :: bsky :: feed :: defs :: SkeletonReasonRepost :: deserialize (map_des) . map (Self :: SkeletonReasonRepost) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod thread_view_post {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Parent {
                        BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                        NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                        ThreadViewPost(
                            std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>,
                        ),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Parent {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(|val| Self::ThreadViewPost(std::boxed::Box::new(val)))
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(|val| Self::ThreadViewPost(std::boxed::Box::new(val)))
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Replies {
                        BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                        NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                        ThreadViewPost(
                            std::boxed::Box<crate::app::bsky::feed::defs::ThreadViewPost>,
                        ),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Replies {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(|val| Self::ThreadViewPost(std::boxed::Box::new(val)))
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(|val| Self::ThreadViewPost(std::boxed::Box::new(val)))
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod describe_feed_generator {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Feed {
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(default)]
                    #[serde(rename = "privacyPolicy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "termsOfService")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub did: atmo_core::Did,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::describe_feed_generator::Feed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub links:
                        std::option::Option<crate::app::bsky::feed::describe_feed_generator::Links>,
                }
            }
            pub mod generator {
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Labels {
                        SelfLabels(crate::com::atproto::label::defs::SelfLabels),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Labels {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_actor_feeds {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BlockedActor,
                    BlockedByActor,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BlockedActor => "BlockedActor",
                            Self::BlockedByActor => "BlockedByActor",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BlockedActor,
                    BlockedByActor,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BlockedActor => "BlockedActor",
                            Self::BlockedByActor => "BlockedByActor",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub filter: std::option::Option<
                        crate::app::bsky::feed::get_author_feed::params::Filter,
                    >,
                    #[serde(default)]
                    #[serde(rename = "includePins")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_pins: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Filter {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::PostsWithReplies => "posts_with_replies",
                                Self::PostsNoReplies => "posts_no_replies",
                                Self::PostsWithMedia => "posts_with_media",
                                Self::PostsAndAuthorThreads => "posts_and_author_threads",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Filter {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod get_feed {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    UnknownFeed,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::UnknownFeed => "UnknownFeed",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "isOnline")]
                    pub is_online: bool,
                    #[serde(rename = "isValid")]
                    pub is_valid: bool,
                    pub view: crate::app::bsky::feed::defs::GeneratorView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub feed: atmo_core::AtUri,
                }
            }
            pub mod get_feed_generators {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub feeds: std::vec::Vec<atmo_core::AtUri>,
                }
            }
            pub mod get_feed_skeleton {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    UnknownFeed,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::UnknownFeed => "UnknownFeed",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::SkeletonFeedPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Like {
                    pub actor: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    UnknownList,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::UnknownList => "UnknownList",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    NotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::NotFound => "NotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub thread: crate::app::bsky::feed::get_post_thread::output::Thread,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threadgate:
                        std::option::Option<crate::app::bsky::feed::defs::ThreadgateView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub depth: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "parentHeight")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub parent_height: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Thread {
                        BlockedPost(crate::app::bsky::feed::defs::BlockedPost),
                        NotFoundPost(crate::app::bsky::feed::defs::NotFoundPost),
                        ThreadViewPost(crate::app::bsky::feed::defs::ThreadViewPost),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Thread {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ThreadViewPost)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.defs#blockedPost" => {
                                        crate::app::bsky::feed::defs::BlockedPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::BlockedPost)
                                    }
                                    "app.bsky.feed.defs#notFoundPost" => {
                                        crate::app::bsky::feed::defs::NotFoundPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundPost)
                                    }
                                    "app.bsky.feed.defs#threadViewPost" => {
                                        crate::app::bsky::feed::defs::ThreadViewPost::deserialize(
                                            map_des,
                                        )
                                        .map(Self::ThreadViewPost)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_posts {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub posts: std::vec::Vec<crate::app::bsky::feed::defs::PostView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<atmo_core::AtUri>,
                }
            }
            pub mod get_quotes {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(rename = "repostedBy")]
                    pub reposted_by: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feed: std::vec::Vec<crate::app::bsky::feed::defs::FeedViewPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Entity {
                    pub index: crate::app::bsky::feed::post::TextSlice,
                    #[serde(rename = "type")]
                    pub ty: std::string::String,
                    pub value: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ReplyRef {
                    pub parent: crate::com::atproto::repo::StrongRef,
                    pub root: crate::com::atproto::repo::StrongRef,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct TextSlice {
                    pub end: i64,
                    pub start: i64,
                }
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Embed {
                        External(crate::app::bsky::embed::External),
                        Images(crate::app::bsky::embed::Images),
                        Record(crate::app::bsky::embed::Record),
                        RecordWithMedia(crate::app::bsky::embed::RecordWithMedia),
                        Video(crate::app::bsky::embed::Video),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Embed {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external" => {
                                        crate::app::bsky::embed::External::deserialize(map_des)
                                            .map(Self::External)
                                    }
                                    "app.bsky.embed.images" => {
                                        crate::app::bsky::embed::Images::deserialize(map_des)
                                            .map(Self::Images)
                                    }
                                    "app.bsky.embed.record" => {
                                        crate::app::bsky::embed::Record::deserialize(map_des)
                                            .map(Self::Record)
                                    }
                                    "app.bsky.embed.recordWithMedia" => {
                                        crate::app::bsky::embed::RecordWithMedia::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RecordWithMedia)
                                    }
                                    "app.bsky.embed.video" => {
                                        crate::app::bsky::embed::Video::deserialize(map_des)
                                            .map(Self::Video)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.external" => {
                                        crate::app::bsky::embed::External::deserialize(map_des)
                                            .map(Self::External)
                                    }
                                    "app.bsky.embed.images" => {
                                        crate::app::bsky::embed::Images::deserialize(map_des)
                                            .map(Self::Images)
                                    }
                                    "app.bsky.embed.record" => {
                                        crate::app::bsky::embed::Record::deserialize(map_des)
                                            .map(Self::Record)
                                    }
                                    "app.bsky.embed.recordWithMedia" => {
                                        crate::app::bsky::embed::RecordWithMedia::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RecordWithMedia)
                                    }
                                    "app.bsky.embed.video" => {
                                        crate::app::bsky::embed::Video::deserialize(map_des)
                                            .map(Self::Video)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Labels {
                        SelfLabels(crate::com::atproto::label::defs::SelfLabels),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Labels {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod postgate {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct DisableRule {}
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum EmbeddingRules {
                        DisableRule(crate::app::bsky::feed::postgate::DisableRule),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for EmbeddingRules {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.postgate#disableRule" => {
                                        crate::app::bsky::feed::postgate::DisableRule::deserialize(
                                            map_des,
                                        )
                                        .map(Self::DisableRule)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.feed.postgate#disableRule" => {
                                        crate::app::bsky::feed::postgate::DisableRule::deserialize(
                                            map_des,
                                        )
                                        .map(Self::DisableRule)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod search_posts {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BadQueryString,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BadQueryString => "BadQueryString",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "hitsTotal")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                    pub posts: std::vec::Vec<crate::app::bsky::feed::defs::PostView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    pub sort:
                        std::option::Option<crate::app::bsky::feed::search_posts::params::Sort>,
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
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Sort {
                        #[serde(rename = "top")]
                        Top,
                        #[serde(rename = "latest")]
                        Latest,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Sort {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Top => "top",
                                Self::Latest => "latest",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Sort {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod send_interactions {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub interactions: std::vec::Vec<crate::app::bsky::feed::defs::Interaction>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
            pub mod threadgate {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct FollowingRule {}
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ListRule {
                    pub list: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MentionRule {}
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Allow {
                        FollowingRule(crate::app::bsky::feed::threadgate::FollowingRule),
                        ListRule(crate::app::bsky::feed::threadgate::ListRule),
                        MentionRule(crate::app::bsky::feed::threadgate::MentionRule),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Allow {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.threadgate#followingRule" => crate :: app :: bsky :: feed :: threadgate :: FollowingRule :: deserialize (map_des) . map (Self :: FollowingRule) , "app.bsky.feed.threadgate#listRule" => crate :: app :: bsky :: feed :: threadgate :: ListRule :: deserialize (map_des) . map (Self :: ListRule) , "app.bsky.feed.threadgate#mentionRule" => crate :: app :: bsky :: feed :: threadgate :: MentionRule :: deserialize (map_des) . map (Self :: MentionRule) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.feed.threadgate#followingRule" => crate :: app :: bsky :: feed :: threadgate :: FollowingRule :: deserialize (map_des) . map (Self :: FollowingRule) , "app.bsky.feed.threadgate#listRule" => crate :: app :: bsky :: feed :: threadgate :: ListRule :: deserialize (map_des) . map (Self :: ListRule) , "app.bsky.feed.threadgate#mentionRule" => crate :: app :: bsky :: feed :: threadgate :: MentionRule :: deserialize (map_des) . map (Self :: MentionRule) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
        }
        pub mod graph {
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Block {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub subject: atmo_core::Did,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Follow {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub subject: atmo_core::Did,
            }
            #[derive(Debug)]
            pub struct GetActorStarterPacks;
            impl atmo_core::xrpc::Request for GetActorStarterPacks {
                type Params = crate::app::bsky::graph::get_actor_starter_packs::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_actor_starter_packs::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetBlocks;
            impl atmo_core::xrpc::Request for GetBlocks {
                type Params = crate::app::bsky::graph::get_blocks::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_blocks::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFollowers;
            impl atmo_core::xrpc::Request for GetFollowers {
                type Params = crate::app::bsky::graph::get_followers::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_followers::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetFollows;
            impl atmo_core::xrpc::Request for GetFollows {
                type Params = crate::app::bsky::graph::get_follows::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_follows::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetKnownFollowers;
            impl atmo_core::xrpc::Request for GetKnownFollowers {
                type Params = crate::app::bsky::graph::get_known_followers::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_known_followers::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetList;
            impl atmo_core::xrpc::Request for GetList {
                type Params = crate::app::bsky::graph::get_list::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_list::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetListBlocks;
            impl atmo_core::xrpc::Request for GetListBlocks {
                type Params = crate::app::bsky::graph::get_list_blocks::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_list_blocks::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetListMutes;
            impl atmo_core::xrpc::Request for GetListMutes {
                type Params = crate::app::bsky::graph::get_list_mutes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_list_mutes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetLists;
            impl atmo_core::xrpc::Request for GetLists {
                type Params = crate::app::bsky::graph::get_lists::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_lists::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetMutes;
            impl atmo_core::xrpc::Request for GetMutes {
                type Params = crate::app::bsky::graph::get_mutes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_mutes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRelationships;
            impl atmo_core::xrpc::Request for GetRelationships {
                type Params = crate::app::bsky::graph::get_relationships::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_relationships::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetStarterPack;
            impl atmo_core::xrpc::Request for GetStarterPack {
                type Params = crate::app::bsky::graph::get_starter_pack::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_starter_pack::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetStarterPacks;
            impl atmo_core::xrpc::Request for GetStarterPacks {
                type Params = crate::app::bsky::graph::get_starter_packs::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_starter_packs::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSuggestedFollowsByActor;
            impl atmo_core::xrpc::Request for GetSuggestedFollowsByActor {
                type Params = crate::app::bsky::graph::get_suggested_follows_by_actor::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::get_suggested_follows_by_actor::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct List {
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub avatar: std::option::Option<atmo_core::Blob>,
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "descriptionFacets")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description_facets:
                    std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub labels: std::option::Option<crate::app::bsky::graph::list::main::Labels>,
                pub name: std::string::String,
                pub purpose: crate::app::bsky::graph::defs::ListPurpose,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Listblock {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub subject: atmo_core::AtUri,
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Listitem {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                pub list: atmo_core::AtUri,
                pub subject: atmo_core::Did,
            }
            #[derive(Debug)]
            pub struct MuteActor;
            impl atmo_core::xrpc::Request for MuteActor {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::mute_actor::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct MuteActorList;
            impl atmo_core::xrpc::Request for MuteActorList {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::mute_actor_list::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct MuteThread;
            impl atmo_core::xrpc::Request for MuteThread {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::mute_thread::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct SearchStarterPacks;
            impl atmo_core::xrpc::Request for SearchStarterPacks {
                type Params = crate::app::bsky::graph::search_starter_packs::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::graph::search_starter_packs::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.graph.searchStarterPacks"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Starterpack {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description: std::option::Option<std::string::String>,
                #[serde(default)]
                #[serde(rename = "descriptionFacets")]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub description_facets:
                    std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub feeds: std::option::Option<
                    std::vec::Vec<crate::app::bsky::graph::starterpack::FeedItem>,
                >,
                pub list: atmo_core::AtUri,
                pub name: std::string::String,
            }
            #[derive(Debug)]
            pub struct UnmuteActor;
            impl atmo_core::xrpc::Request for UnmuteActor {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::unmute_actor::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UnmuteActorList;
            impl atmo_core::xrpc::Request for UnmuteActorList {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::unmute_actor_list::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UnmuteThread;
            impl atmo_core::xrpc::Request for UnmuteThread {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::graph::unmute_thread::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ListItemView {
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum ListPurpose {
                    #[serde(rename = "app.bsky.graph.defs#modlist")]
                    AppBskyGraphDefsModlist,
                    #[serde(rename = "app.bsky.graph.defs#curatelist")]
                    AppBskyGraphDefsCuratelist,
                    #[serde(rename = "app.bsky.graph.defs#referencelist")]
                    AppBskyGraphDefsReferencelist,
                    #[serde(untagged)]
                    Other(String),
                }
                impl ListPurpose {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AppBskyGraphDefsModlist => "app.bsky.graph.defs#modlist",
                            Self::AppBskyGraphDefsCuratelist => "app.bsky.graph.defs#curatelist",
                            Self::AppBskyGraphDefsReferencelist => {
                                "app.bsky.graph.defs#referencelist"
                            }
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for ListPurpose {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    #[serde(rename = "descriptionFacets")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description_facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "listItemCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(rename = "indexedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub indexed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "listItemCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub name: std::string::String,
                    pub purpose: crate::app::bsky::graph::defs::ListPurpose,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<crate::app::bsky::graph::defs::ListViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ListViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blocked: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub muted: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct NotFoundActor {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(rename = "notFound")]
                    pub not_found: bool,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Relationship {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "followedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub followed_by: std::option::Option<atmo_core::AtUri>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub following: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackView {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub feeds: std::option::Option<
                        std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                    >,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(rename = "joinedAllTimeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "joinedWeekCount")]
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
                    #[serde(rename = "listItemsSample")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_items_sample: std::option::Option<
                        std::vec::Vec<crate::app::bsky::graph::defs::ListItemView>,
                    >,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct StarterPackViewBasic {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileViewBasic,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(rename = "joinedAllTimeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_all_time_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "joinedWeekCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub joined_week_count: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "listItemCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub list_item_count: std::option::Option<i64>,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_actor_starter_packs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(rename = "starterPacks")]
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blocks: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub follows: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub followers: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                    pub subject: crate::app::bsky::actor::defs::ProfileView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub items: std::vec::Vec<crate::app::bsky::graph::defs::ListItemView>,
                    pub list: crate::app::bsky::graph::defs::ListView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub lists: std::vec::Vec<crate::app::bsky::graph::defs::ListView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub mutes: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    ActorNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ActorNotFound => "ActorNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub actor: std::option::Option<atmo_core::Did>,
                    pub relationships: std::vec::Vec<
                        crate::app::bsky::graph::get_relationships::output::Relationships,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub others: std::option::Option<std::vec::Vec<atmo_core::AtIdentifier>>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Relationships {
                        NotFoundActor(crate::app::bsky::graph::defs::NotFoundActor),
                        Relationship(crate::app::bsky::graph::defs::Relationship),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Relationships {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.graph.defs#notFoundActor" => {
                                        crate::app::bsky::graph::defs::NotFoundActor::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundActor)
                                    }
                                    "app.bsky.graph.defs#relationship" => {
                                        crate::app::bsky::graph::defs::Relationship::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Relationship)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.graph.defs#notFoundActor" => {
                                        crate::app::bsky::graph::defs::NotFoundActor::deserialize(
                                            map_des,
                                        )
                                        .map(Self::NotFoundActor)
                                    }
                                    "app.bsky.graph.defs#relationship" => {
                                        crate::app::bsky::graph::defs::Relationship::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Relationship)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_starter_pack {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "starterPack")]
                    pub starter_pack: crate::app::bsky::graph::defs::StarterPackView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(rename = "starterPack")]
                    pub starter_pack: atmo_core::AtUri,
                }
            }
            pub mod get_starter_packs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "starterPacks")]
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<atmo_core::AtUri>,
                }
            }
            pub mod get_suggested_follows_by_actor {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(rename = "isFallback")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub is_fallback: std::option::Option<bool>,
                    pub suggestions: std::vec::Vec<crate::app::bsky::actor::defs::ProfileView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod list {
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Labels {
                        SelfLabels(crate::com::atproto::label::defs::SelfLabels),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Labels {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod mute_actor {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod mute_actor_list {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod mute_thread {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub root: atmo_core::AtUri,
                }
            }
            pub mod search_starter_packs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(rename = "starterPacks")]
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::graph::defs::StarterPackViewBasic>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub q: std::string::String,
                }
            }
            pub mod starterpack {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct FeedItem {
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod unmute_actor {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::AtIdentifier,
                }
            }
            pub mod unmute_actor_list {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub list: atmo_core::AtUri,
                }
            }
            pub mod unmute_thread {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub root: atmo_core::AtUri,
                }
            }
        }
        pub mod labeler {
            #[derive(Debug)]
            pub struct GetServices;
            impl atmo_core::xrpc::Request for GetServices {
                type Params = crate::app::bsky::labeler::get_services::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::labeler::get_services::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Service {
                #[serde(rename = "createdAt")]
                pub created_at: atmo_core::DateTimeString,
                #[serde(default)]
                #[serde(skip_serializing_if = "std::option::Option::is_none")]
                pub labels: std::option::Option<crate::app::bsky::labeler::service::main::Labels>,
                pub policies: crate::app::bsky::labeler::defs::LabelerPolicies,
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerPolicies {
                    #[serde(default)]
                    #[serde(rename = "labelValueDefinitions")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub label_value_definitions: std::option::Option<
                        std::vec::Vec<crate::com::atproto::label::defs::LabelValueDefinition>,
                    >,
                    #[serde(rename = "labelValues")]
                    pub label_values: std::vec::Vec<crate::com::atproto::label::defs::LabelValue>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerView {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "likeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewDetailed {
                    pub cid: atmo_core::CidString,
                    pub creator: crate::app::bsky::actor::defs::ProfileView,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    #[serde(default)]
                    #[serde(rename = "likeCount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like_count: std::option::Option<i64>,
                    pub policies: crate::app::bsky::labeler::defs::LabelerPolicies,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer:
                        std::option::Option<crate::app::bsky::labeler::defs::LabelerViewerState>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelerViewerState {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub like: std::option::Option<atmo_core::AtUri>,
                }
            }
            pub mod get_services {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub views:
                        std::vec::Vec<crate::app::bsky::labeler::get_services::output::Views>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub detailed: std::option::Option<bool>,
                    pub dids: std::vec::Vec<atmo_core::Did>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Views {
                        LabelerView(crate::app::bsky::labeler::defs::LabelerView),
                        LabelerViewDetailed(crate::app::bsky::labeler::defs::LabelerViewDetailed),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Views {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.labeler.defs#labelerView" => crate :: app :: bsky :: labeler :: defs :: LabelerView :: deserialize (map_des) . map (Self :: LabelerView) , "app.bsky.labeler.defs#labelerViewDetailed" => crate :: app :: bsky :: labeler :: defs :: LabelerViewDetailed :: deserialize (map_des) . map (Self :: LabelerViewDetailed) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "app.bsky.labeler.defs#labelerView" => crate :: app :: bsky :: labeler :: defs :: LabelerView :: deserialize (map_des) . map (Self :: LabelerView) , "app.bsky.labeler.defs#labelerViewDetailed" => crate :: app :: bsky :: labeler :: defs :: LabelerViewDetailed :: deserialize (map_des) . map (Self :: LabelerViewDetailed) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod service {
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Labels {
                        SelfLabels(crate::com::atproto::label::defs::SelfLabels),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Labels {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.label.defs#selfLabels" => {
                                        crate::com::atproto::label::defs::SelfLabels::deserialize(
                                            map_des,
                                        )
                                        .map(Self::SelfLabels)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
        }
        pub mod notification {
            #[derive(Debug)]
            pub struct GetUnreadCount;
            impl atmo_core::xrpc::Request for GetUnreadCount {
                type Params = crate::app::bsky::notification::get_unread_count::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::notification::get_unread_count::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ListNotifications;
            impl atmo_core::xrpc::Request for ListNotifications {
                type Params = crate::app::bsky::notification::list_notifications::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::notification::list_notifications::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct PutPreferences;
            impl atmo_core::xrpc::Request for PutPreferences {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::notification::put_preferences::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RegisterPush;
            impl atmo_core::xrpc::Request for RegisterPush {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::notification::register_push::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateSeen;
            impl atmo_core::xrpc::Request for UpdateSeen {
                type Params = atmo_core::Nothing;
                type Input = crate::app::bsky::notification::update_seen::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub count: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub priority: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "seenAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
            }
            pub mod list_notifications {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Notification {
                    pub author: crate::app::bsky::actor::defs::ProfileView,
                    pub cid: atmo_core::CidString,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(rename = "isRead")]
                    pub is_read: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    pub reason:
                        crate::app::bsky::notification::list_notifications::notification::Reason,
                    #[serde(default)]
                    #[serde(rename = "reasonSubject")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason_subject: std::option::Option<atmo_core::AtUri>,
                    pub record: atmo_core::Unknown,
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    #[serde(rename = "seenAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                    #[serde(rename = "seenAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub seen_at: std::option::Option<atmo_core::DateTimeString>,
                }
                pub mod notification {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Reason {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Like => "like",
                                Self::Repost => "repost",
                                Self::Follow => "follow",
                                Self::Mention => "mention",
                                Self::Reply => "reply",
                                Self::Quote => "quote",
                                Self::StarterpackJoined => "starterpack-joined",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Reason {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod put_preferences {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub priority: bool,
                }
            }
            pub mod register_push {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "appId")]
                    pub app_id: std::string::String,
                    pub platform: crate::app::bsky::notification::register_push::input::Platform,
                    #[serde(rename = "serviceDid")]
                    pub service_did: atmo_core::Did,
                    pub token: std::string::String,
                }
                pub mod input {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Platform {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Ios => "ios",
                                Self::Android => "android",
                                Self::Web => "web",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Platform {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod update_seen {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "seenAt")]
                    pub seen_at: atmo_core::DateTimeString,
                }
            }
        }
        pub mod richtext {
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Facet {
                pub features: std::vec::Vec<crate::app::bsky::richtext::facet::main::Features>,
                pub index: crate::app::bsky::richtext::facet::ByteSlice,
            }
            pub mod facet {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ByteSlice {
                    #[serde(rename = "byteEnd")]
                    pub byte_end: i64,
                    #[serde(rename = "byteStart")]
                    pub byte_start: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Link {
                    pub uri: url::Url,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Mention {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Tag {
                    pub tag: std::string::String,
                }
                pub mod main {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Features {
                        Link(crate::app::bsky::richtext::facet::Link),
                        Mention(crate::app::bsky::richtext::facet::Mention),
                        Tag(crate::app::bsky::richtext::facet::Tag),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Features {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.richtext.facet#link" => {
                                        crate::app::bsky::richtext::facet::Link::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Link)
                                    }
                                    "app.bsky.richtext.facet#mention" => {
                                        crate::app::bsky::richtext::facet::Mention::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Mention)
                                    }
                                    "app.bsky.richtext.facet#tag" => {
                                        crate::app::bsky::richtext::facet::Tag::deserialize(map_des)
                                            .map(Self::Tag)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.richtext.facet#link" => {
                                        crate::app::bsky::richtext::facet::Link::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Link)
                                    }
                                    "app.bsky.richtext.facet#mention" => {
                                        crate::app::bsky::richtext::facet::Mention::deserialize(
                                            map_des,
                                        )
                                        .map(Self::Mention)
                                    }
                                    "app.bsky.richtext.facet#tag" => {
                                        crate::app::bsky::richtext::facet::Tag::deserialize(map_des)
                                            .map(Self::Tag)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
        }
        pub mod unspecced {
            #[derive(Debug)]
            pub struct GetConfig;
            impl atmo_core::xrpc::Request for GetConfig {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::get_config::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.getConfig"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            #[derive(Debug)]
            pub struct GetPopularFeedGenerators;
            impl atmo_core::xrpc::Request for GetPopularFeedGenerators {
                type Params = crate::app::bsky::unspecced::get_popular_feed_generators::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::get_popular_feed_generators::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSuggestionsSkeleton;
            impl atmo_core::xrpc::Request for GetSuggestionsSkeleton {
                type Params = crate::app::bsky::unspecced::get_suggestions_skeleton::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::get_suggestions_skeleton::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetTaggedSuggestions;
            impl atmo_core::xrpc::Request for GetTaggedSuggestions {
                type Params = crate::app::bsky::unspecced::get_tagged_suggestions::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::get_tagged_suggestions::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchActorsSkeleton;
            impl atmo_core::xrpc::Request for SearchActorsSkeleton {
                type Params = crate::app::bsky::unspecced::search_actors_skeleton::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::search_actors_skeleton::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchPostsSkeleton;
            impl atmo_core::xrpc::Request for SearchPostsSkeleton {
                type Params = crate::app::bsky::unspecced::search_posts_skeleton::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::search_posts_skeleton::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchStarterPacksSkeleton;
            impl atmo_core::xrpc::Request for SearchStarterPacksSkeleton {
                type Params = crate::app::bsky::unspecced::search_starter_packs_skeleton::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::unspecced::search_starter_packs_skeleton::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
                }
                #[inline]
                fn nsid() -> &'static str {
                    "app.bsky.unspecced.searchStarterPacksSkeleton"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchActor {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchPost {
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SkeletonSearchStarterPack {
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_config {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(rename = "checkEmailConfirmed")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub check_email_confirmed: std::option::Option<bool>,
                }
            }
            pub mod get_popular_feed_generators {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub feeds: std::vec::Vec<crate::app::bsky::feed::defs::GeneratorView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors:
                        std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchActor>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "relativeToDid")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub relative_to_did: std::option::Option<atmo_core::Did>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "relativeToDid")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub relative_to_did: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
            }
            pub mod get_tagged_suggestions {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub suggestions: std::vec::Vec<
                        crate::app::bsky::unspecced::get_tagged_suggestions::Suggestion,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {}
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Suggestion { pub subject : url :: Url , # [serde (rename = "subjectType")] pub subject_type : crate :: app :: bsky :: unspecced :: get_tagged_suggestions :: suggestion :: SubjectType , pub tag : std :: string :: String , }
                pub mod suggestion {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        #[serde(rename = "actor")]
                        Actor,
                        #[serde(rename = "feed")]
                        Feed,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl SubjectType {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Actor => "actor",
                                Self::Feed => "feed",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for SubjectType {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod search_actors_skeleton {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BadQueryString,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BadQueryString => "BadQueryString",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub actors:
                        std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchActor>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "hitsTotal")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub q: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub typeahead: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
            }
            pub mod search_posts_skeleton {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BadQueryString,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BadQueryString => "BadQueryString",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "hitsTotal")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                    pub posts: std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchPost>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                        crate::app::bsky::unspecced::search_posts_skeleton::params::Sort,
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
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Sort {
                        #[serde(rename = "top")]
                        Top,
                        #[serde(rename = "latest")]
                        Latest,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Sort {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Top => "top",
                                Self::Latest => "latest",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Sort {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod search_starter_packs_skeleton {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BadQueryString,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BadQueryString => "BadQueryString",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "hitsTotal")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hits_total: std::option::Option<i64>,
                    #[serde(rename = "starterPacks")]
                    pub starter_packs:
                        std::vec::Vec<crate::app::bsky::unspecced::defs::SkeletonSearchStarterPack>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub q: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub viewer: std::option::Option<atmo_core::Did>,
                }
            }
        }
        pub mod video {
            #[derive(Debug)]
            pub struct GetJobStatus;
            impl atmo_core::xrpc::Request for GetJobStatus {
                type Params = crate::app::bsky::video::get_job_status::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::video::get_job_status::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetUploadLimits;
            impl atmo_core::xrpc::Request for GetUploadLimits {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::video::get_upload_limits::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct UploadVideo;
            impl atmo_core::xrpc::Request for UploadVideo {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::app::bsky::video::upload_video::Output;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct JobStatus {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub blob: std::option::Option<atmo_core::Blob>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub error: std::option::Option<std::string::String>,
                    #[serde(rename = "jobId")]
                    pub job_id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub progress: std::option::Option<i64>,
                    pub state: crate::app::bsky::video::defs::job_status::State,
                }
                pub mod job_status {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum State {
                        #[serde(rename = "JOB_STATE_COMPLETED")]
                        JobStateCompleted,
                        #[serde(rename = "JOB_STATE_FAILED")]
                        JobStateFailed,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl State {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::JobStateCompleted => "JOB_STATE_COMPLETED",
                                Self::JobStateFailed => "JOB_STATE_FAILED",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for State {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod get_job_status {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "jobStatus")]
                    pub job_status: crate::app::bsky::video::defs::JobStatus,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(rename = "jobId")]
                    pub job_id: std::string::String,
                }
            }
            pub mod get_upload_limits {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "canUpload")]
                    pub can_upload: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub error: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "remainingDailyBytes")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub remaining_daily_bytes: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "remainingDailyVideos")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub remaining_daily_videos: std::option::Option<i64>,
                }
            }
            pub mod upload_video {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "jobStatus")]
                    pub job_status: crate::app::bsky::video::defs::JobStatus,
                }
            }
        }
    }
}
pub mod chat {
    pub mod bsky {
        pub mod actor {
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct Declaration {
                #[serde(rename = "allowIncoming")]
                pub allow_incoming: crate::chat::bsky::actor::declaration::main::AllowIncoming,
            }
            #[derive(Debug)]
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::actor::delete_account::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ExportAccountData;
            impl atmo_core::xrpc::Request for ExportAccountData {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            pub mod declaration {
                pub mod main {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl AllowIncoming {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::All => "all",
                                Self::None => "none",
                                Self::Following => "following",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for AllowIncoming {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ProfileViewBasic {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub associated:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileAssociated>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub avatar: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "chatDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub chat_disabled: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "displayName")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
        }
        pub mod convo {
            #[derive(Debug)]
            pub struct DeleteMessageForSelf;
            impl atmo_core::xrpc::Request for DeleteMessageForSelf {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::delete_message_for_self::Input;
                type Output = crate::chat::bsky::convo::defs::DeletedMessageView;
                type Error = String;
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
            #[derive(Debug)]
            pub struct GetConvo;
            impl atmo_core::xrpc::Request for GetConvo {
                type Params = crate::chat::bsky::convo::get_convo::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::convo::get_convo::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetConvoForMembers;
            impl atmo_core::xrpc::Request for GetConvoForMembers {
                type Params = crate::chat::bsky::convo::get_convo_for_members::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::convo::get_convo_for_members::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetLog;
            impl atmo_core::xrpc::Request for GetLog {
                type Params = crate::chat::bsky::convo::get_log::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::convo::get_log::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetMessages;
            impl atmo_core::xrpc::Request for GetMessages {
                type Params = crate::chat::bsky::convo::get_messages::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::convo::get_messages::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct LeaveConvo;
            impl atmo_core::xrpc::Request for LeaveConvo {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::leave_convo::Input;
                type Output = crate::chat::bsky::convo::leave_convo::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ListConvos;
            impl atmo_core::xrpc::Request for ListConvos {
                type Params = crate::chat::bsky::convo::list_convos::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::convo::list_convos::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct MuteConvo;
            impl atmo_core::xrpc::Request for MuteConvo {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::mute_convo::Input;
                type Output = crate::chat::bsky::convo::mute_convo::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct SendMessage;
            impl atmo_core::xrpc::Request for SendMessage {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::send_message::Input;
                type Output = crate::chat::bsky::convo::defs::MessageView;
                type Error = String;
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
            #[derive(Debug)]
            pub struct SendMessageBatch;
            impl atmo_core::xrpc::Request for SendMessageBatch {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::send_message_batch::Input;
                type Output = crate::chat::bsky::convo::send_message_batch::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UnmuteConvo;
            impl atmo_core::xrpc::Request for UnmuteConvo {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::unmute_convo::Input;
                type Output = crate::chat::bsky::convo::unmute_convo::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateRead;
            impl atmo_core::xrpc::Request for UpdateRead {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::convo::update_read::Input;
                type Output = crate::chat::bsky::convo::update_read::Output;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ConvoView {
                    pub id: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "lastMessage")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_message: std::option::Option<
                        crate::chat::bsky::convo::defs::convo_view::LastMessage,
                    >,
                    pub members: std::vec::Vec<crate::chat::bsky::actor::defs::ProfileViewBasic>,
                    pub muted: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub opened: std::option::Option<bool>,
                    pub rev: std::string::String,
                    #[serde(rename = "unreadCount")]
                    pub unread_count: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct DeletedMessageView {
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    #[serde(rename = "sentAt")]
                    pub sent_at: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LogBeginConvo {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LogCreateMessage {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::log_create_message::Message,
                    pub rev: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LogDeleteMessage {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::log_delete_message::Message,
                    pub rev: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LogLeaveConvo {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MessageInput {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed:
                        std::option::Option<crate::chat::bsky::convo::defs::message_input::Embed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub text: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MessageRef {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub did: atmo_core::Did,
                    #[serde(rename = "messageId")]
                    pub message_id: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MessageView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub embed:
                        std::option::Option<crate::chat::bsky::convo::defs::message_view::Embed>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub facets:
                        std::option::Option<std::vec::Vec<crate::app::bsky::richtext::Facet>>,
                    pub id: std::string::String,
                    pub rev: std::string::String,
                    pub sender: crate::chat::bsky::convo::defs::MessageViewSender,
                    #[serde(rename = "sentAt")]
                    pub sent_at: atmo_core::DateTimeString,
                    pub text: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct MessageViewSender {
                    pub did: atmo_core::Did,
                }
                pub mod convo_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum LastMessage {
                        DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                        MessageView(crate::chat::bsky::convo::defs::MessageView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for LastMessage {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod log_create_message {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Message {
                        DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                        MessageView(crate::chat::bsky::convo::defs::MessageView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Message {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod log_delete_message {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Message {
                        DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                        MessageView(crate::chat::bsky::convo::defs::MessageView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Message {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod message_input {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Embed {
                        Record(crate::app::bsky::embed::Record),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Embed {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.record" => {
                                        crate::app::bsky::embed::Record::deserialize(map_des)
                                            .map(Self::Record)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.record" => {
                                        crate::app::bsky::embed::Record::deserialize(map_des)
                                            .map(Self::Record)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod message_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Embed {
                        View(crate::app::bsky::embed::record::View),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Embed {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.record#view" => {
                                        crate::app::bsky::embed::record::View::deserialize(map_des)
                                            .map(Self::View)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "app.bsky.embed.record#view" => {
                                        crate::app::bsky::embed::record::View::deserialize(map_des)
                                            .map(Self::View)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod delete_message_for_self {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    #[serde(rename = "messageId")]
                    pub message_id: std::string::String,
                }
            }
            pub mod get_convo {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                }
            }
            pub mod get_convo_for_members {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub members: std::vec::Vec<atmo_core::Did>,
                }
            }
            pub mod get_log {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub logs: std::vec::Vec<crate::chat::bsky::convo::get_log::output::Logs>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Logs {
                        LogBeginConvo(crate::chat::bsky::convo::defs::LogBeginConvo),
                        LogCreateMessage(crate::chat::bsky::convo::defs::LogCreateMessage),
                        LogDeleteMessage(crate::chat::bsky::convo::defs::LogDeleteMessage),
                        LogLeaveConvo(crate::chat::bsky::convo::defs::LogLeaveConvo),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Logs {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#logBeginConvo" => crate :: chat :: bsky :: convo :: defs :: LogBeginConvo :: deserialize (map_des) . map (Self :: LogBeginConvo) , "chat.bsky.convo.defs#logCreateMessage" => crate :: chat :: bsky :: convo :: defs :: LogCreateMessage :: deserialize (map_des) . map (Self :: LogCreateMessage) , "chat.bsky.convo.defs#logDeleteMessage" => crate :: chat :: bsky :: convo :: defs :: LogDeleteMessage :: deserialize (map_des) . map (Self :: LogDeleteMessage) , "chat.bsky.convo.defs#logLeaveConvo" => crate :: chat :: bsky :: convo :: defs :: LogLeaveConvo :: deserialize (map_des) . map (Self :: LogLeaveConvo) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#logBeginConvo" => crate :: chat :: bsky :: convo :: defs :: LogBeginConvo :: deserialize (map_des) . map (Self :: LogBeginConvo) , "chat.bsky.convo.defs#logCreateMessage" => crate :: chat :: bsky :: convo :: defs :: LogCreateMessage :: deserialize (map_des) . map (Self :: LogCreateMessage) , "chat.bsky.convo.defs#logDeleteMessage" => crate :: chat :: bsky :: convo :: defs :: LogDeleteMessage :: deserialize (map_des) . map (Self :: LogDeleteMessage) , "chat.bsky.convo.defs#logLeaveConvo" => crate :: chat :: bsky :: convo :: defs :: LogLeaveConvo :: deserialize (map_des) . map (Self :: LogLeaveConvo) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_messages {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub messages:
                        std::vec::Vec<crate::chat::bsky::convo::get_messages::output::Messages>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Messages {
                        DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                        MessageView(crate::chat::bsky::convo::defs::MessageView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Messages {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod leave_convo {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub rev: std::string::String,
                }
            }
            pub mod list_convos {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convos: std::vec::Vec<crate::chat::bsky::convo::defs::ConvoView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
            pub mod send_message {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::MessageInput,
                }
            }
            pub mod send_message_batch {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BatchItem {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    pub message: crate::chat::bsky::convo::defs::MessageInput,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub items:
                        std::vec::Vec<crate::chat::bsky::convo::send_message_batch::BatchItem>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub items: std::vec::Vec<crate::chat::bsky::convo::defs::MessageView>,
                }
            }
            pub mod unmute_convo {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
            pub mod update_read {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "convoId")]
                    pub convo_id: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "messageId")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message_id: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub convo: crate::chat::bsky::convo::defs::ConvoView,
                }
            }
        }
        pub mod moderation {
            #[derive(Debug)]
            pub struct GetActorMetadata;
            impl atmo_core::xrpc::Request for GetActorMetadata {
                type Params = crate::chat::bsky::moderation::get_actor_metadata::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::moderation::get_actor_metadata::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetMessageContext;
            impl atmo_core::xrpc::Request for GetMessageContext {
                type Params = crate::chat::bsky::moderation::get_message_context::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::chat::bsky::moderation::get_message_context::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct UpdateActorAccess;
            impl atmo_core::xrpc::Request for UpdateActorAccess {
                type Params = atmo_core::Nothing;
                type Input = crate::chat::bsky::moderation::update_actor_access::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Metadata {
                    pub convos: i64,
                    #[serde(rename = "convosStarted")]
                    pub convos_started: i64,
                    #[serde(rename = "messagesReceived")]
                    pub messages_received: i64,
                    #[serde(rename = "messagesSent")]
                    pub messages_sent: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub all: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                    pub day: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                    pub month: crate::chat::bsky::moderation::get_actor_metadata::Metadata,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub actor: atmo_core::Did,
                }
            }
            pub mod get_message_context {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub messages: std::vec::Vec<
                        crate::chat::bsky::moderation::get_message_context::output::Messages,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub after: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub before: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "convoId")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub convo_id: std::option::Option<std::string::String>,
                    #[serde(rename = "messageId")]
                    pub message_id: std::string::String,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Messages {
                        DeletedMessageView(crate::chat::bsky::convo::defs::DeletedMessageView),
                        MessageView(crate::chat::bsky::convo::defs::MessageView),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Messages {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "chat.bsky.convo.defs#deletedMessageView" => crate :: chat :: bsky :: convo :: defs :: DeletedMessageView :: deserialize (map_des) . map (Self :: DeletedMessageView) , "chat.bsky.convo.defs#messageView" => crate :: chat :: bsky :: convo :: defs :: MessageView :: deserialize (map_des) . map (Self :: MessageView) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod update_actor_access {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub actor: atmo_core::Did,
                    #[serde(rename = "allowAccess")]
                    pub allow_access: bool,
                    #[serde(default)]
                    #[serde(rename = "ref")]
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
            #[derive(Debug)]
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::delete_account::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DisableAccountInvites;
            impl atmo_core::xrpc::Request for DisableAccountInvites {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::disable_account_invites::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DisableInviteCodes;
            impl atmo_core::xrpc::Request for DisableInviteCodes {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::disable_invite_codes::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct EnableAccountInvites;
            impl atmo_core::xrpc::Request for EnableAccountInvites {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::enable_account_invites::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct GetAccountInfo;
            impl atmo_core::xrpc::Request for GetAccountInfo {
                type Params = crate::com::atproto::admin::get_account_info::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::admin::defs::AccountView;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetAccountInfos;
            impl atmo_core::xrpc::Request for GetAccountInfos {
                type Params = crate::com::atproto::admin::get_account_infos::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::admin::get_account_infos::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetInviteCodes;
            impl atmo_core::xrpc::Request for GetInviteCodes {
                type Params = crate::com::atproto::admin::get_invite_codes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::admin::get_invite_codes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSubjectStatus;
            impl atmo_core::xrpc::Request for GetSubjectStatus {
                type Params = crate::com::atproto::admin::get_subject_status::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::admin::get_subject_status::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchAccounts;
            impl atmo_core::xrpc::Request for SearchAccounts {
                type Params = crate::com::atproto::admin::search_accounts::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::admin::search_accounts::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SendEmail;
            impl atmo_core::xrpc::Request for SendEmail {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::send_email::Input;
                type Output = crate::com::atproto::admin::send_email::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateAccountEmail;
            impl atmo_core::xrpc::Request for UpdateAccountEmail {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::update_account_email::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateAccountHandle;
            impl atmo_core::xrpc::Request for UpdateAccountHandle {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::update_account_handle::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateAccountPassword;
            impl atmo_core::xrpc::Request for UpdateAccountPassword {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::update_account_password::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateSubjectStatus;
            impl atmo_core::xrpc::Request for UpdateSubjectStatus {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::admin::update_subject_status::Input;
                type Output = crate::com::atproto::admin::update_subject_status::Output;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AccountView {
                    #[serde(default)]
                    #[serde(rename = "deactivatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "emailConfirmedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo_core::DateTimeString>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(rename = "inviteNote")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "invitedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites: std::option::Option<
                        std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    >,
                    #[serde(default)]
                    #[serde(rename = "invitesDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "relatedRecords")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub related_records: std::option::Option<std::vec::Vec<atmo_core::Unknown>>,
                    #[serde(default)]
                    #[serde(rename = "threatSignatures")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threat_signatures: std::option::Option<
                        std::vec::Vec<crate::com::atproto::admin::defs::ThreatSignature>,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoBlobRef {
                    pub cid: atmo_core::CidString,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "recordUri")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub record_uri: std::option::Option<atmo_core::AtUri>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoRef {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct StatusAttr {
                    pub applied: bool,
                    #[serde(default)]
                    #[serde(rename = "ref")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ref_: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ThreatSignature {
                    pub property: std::string::String,
                    pub value: std::string::String,
                }
            }
            pub mod delete_account {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                }
            }
            pub mod disable_account_invites {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub note: std::option::Option<std::string::String>,
                }
            }
            pub mod disable_invite_codes {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub note: std::option::Option<std::string::String>,
                }
            }
            pub mod get_account_info {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_account_infos {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub infos: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<atmo_core::Did>,
                }
            }
            pub mod get_invite_codes {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort: std::option::Option<
                        crate::com::atproto::admin::get_invite_codes::params::Sort,
                    >,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Sort {
                        #[serde(rename = "recent")]
                        Recent,
                        #[serde(rename = "usage")]
                        Usage,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Sort {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Recent => "recent",
                                Self::Usage => "usage",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Sort {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod get_subject_status {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated:
                        std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                    pub subject: crate::com::atproto::admin::get_subject_status::output::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod search_accounts {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub content: std::string::String,
                    #[serde(rename = "recipientDid")]
                    pub recipient_did: atmo_core::Did,
                    #[serde(rename = "senderDid")]
                    pub sender_did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub sent: bool,
                }
            }
            pub mod update_account_email {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub account: atmo_core::AtIdentifier,
                    pub email: std::string::String,
                }
            }
            pub mod update_account_handle {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub handle: atmo_core::Handle,
                }
            }
            pub mod update_account_password {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub password: std::string::String,
                }
            }
            pub mod update_subject_status {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated:
                        std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                    pub subject: crate::com::atproto::admin::update_subject_status::input::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub subject: crate::com::atproto::admin::update_subject_status::output::Subject,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takedown: std::option::Option<crate::com::atproto::admin::defs::StatusAttr>,
                }
                pub mod input {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoBlobRef(crate::com::atproto::admin::defs::RepoBlobRef),
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoBlobRef" => {
                                        crate::com::atproto::admin::defs::RepoBlobRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoBlobRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
        }
        pub mod identity {
            #[derive(Debug)]
            pub struct GetRecommendedDidCredentials;
            impl atmo_core::xrpc::Request for GetRecommendedDidCredentials {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output =
                    crate::com::atproto::identity::get_recommended_did_credentials::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct RequestPlcOperationSignature;
            impl atmo_core::xrpc::Request for RequestPlcOperationSignature {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ResolveHandle;
            impl atmo_core::xrpc::Request for ResolveHandle {
                type Params = crate::com::atproto::identity::resolve_handle::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::identity::resolve_handle::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SignPlcOperation;
            impl atmo_core::xrpc::Request for SignPlcOperation {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::identity::sign_plc_operation::Input;
                type Output = crate::com::atproto::identity::sign_plc_operation::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct SubmitPlcOperation;
            impl atmo_core::xrpc::Request for SubmitPlcOperation {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::identity::submit_plc_operation::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateHandle;
            impl atmo_core::xrpc::Request for UpdateHandle {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::identity::update_handle::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(rename = "alsoKnownAs")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub also_known_as: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "rotationKeys")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rotation_keys: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub services: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(rename = "verificationMethods")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_methods: std::option::Option<atmo_core::Unknown>,
                }
            }
            pub mod resolve_handle {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub handle: atmo_core::Handle,
                }
            }
            pub mod sign_plc_operation {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(rename = "alsoKnownAs")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub also_known_as: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "rotationKeys")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rotation_keys: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub services: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub token: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "verificationMethods")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_methods: std::option::Option<atmo_core::Unknown>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub operation: atmo_core::Unknown,
                }
            }
            pub mod submit_plc_operation {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub operation: atmo_core::Unknown,
                }
            }
            pub mod update_handle {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub handle: atmo_core::Handle,
                }
            }
        }
        pub mod label {
            #[derive(Debug)]
            pub struct QueryLabels;
            impl atmo_core::xrpc::Request for QueryLabels {
                type Params = crate::com::atproto::label::query_labels::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::label::query_labels::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Label {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub cts: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exp: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub neg: std::option::Option<bool>,
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
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
                impl LabelValue {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::Hide => "!hide",
                            Self::NoPromote => "!no-promote",
                            Self::Warn => "!warn",
                            Self::NoUnauthenticated => "!no-unauthenticated",
                            Self::DmcaViolation => "dmca-violation",
                            Self::Doxxing => "doxxing",
                            Self::Porn => "porn",
                            Self::Sexual => "sexual",
                            Self::Nudity => "nudity",
                            Self::Nsfl => "nsfl",
                            Self::Gore => "gore",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for LabelValue {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelValueDefinition {
                    #[serde(default)]
                    #[serde(rename = "adultOnly")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub adult_only: std::option::Option<bool>,
                    pub blurs: crate::com::atproto::label::defs::label_value_definition::Blurs,
                    #[serde(default)]
                    #[serde(rename = "defaultSetting")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub default_setting: std::option::Option<
                        crate::com::atproto::label::defs::label_value_definition::DefaultSetting,
                    >,
                    pub identifier: std::string::String,
                    pub locales: std::vec::Vec<
                        crate::com::atproto::label::defs::LabelValueDefinitionStrings,
                    >,
                    pub severity:
                        crate::com::atproto::label::defs::label_value_definition::Severity,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct LabelValueDefinitionStrings {
                    pub description: std::string::String,
                    pub lang: std::string::String,
                    pub name: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SelfLabel {
                    pub val: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SelfLabels {
                    pub values: std::vec::Vec<crate::com::atproto::label::defs::SelfLabel>,
                }
                pub mod label_value_definition {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Blurs {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Content => "content",
                                Self::Media => "media",
                                Self::None => "none",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Blurs {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl DefaultSetting {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Ignore => "ignore",
                                Self::Warn => "warn",
                                Self::Hide => "hide",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for DefaultSetting {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Severity {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Inform => "inform",
                                Self::Alert => "alert",
                                Self::None => "none",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Severity {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod query_labels {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sources: std::option::Option<std::vec::Vec<atmo_core::Did>>,
                    #[serde(rename = "uriPatterns")]
                    pub uri_patterns: std::vec::Vec<std::string::String>,
                }
            }
            pub mod subscribe_labels {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::label::subscribe_labels::info::Name,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Labels {
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                    pub seq: i64,
                }
                pub mod info {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Name {
                        OutdatedCursor,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Name {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::OutdatedCursor => "OutdatedCursor",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Name {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
        pub mod moderation {
            #[derive(Debug)]
            pub struct CreateReport;
            impl atmo_core::xrpc::Request for CreateReport {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::moderation::create_report::Input;
                type Output = crate::com::atproto::moderation::create_report::Output;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<std::string::String>,
                    #[serde(rename = "reasonType")]
                    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
                    pub subject: crate::com::atproto::moderation::create_report::input::Subject,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    pub id: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reason: std::option::Option<std::string::String>,
                    #[serde(rename = "reasonType")]
                    pub reason_type: crate::com::atproto::moderation::defs::ReasonType,
                    #[serde(rename = "reportedBy")]
                    pub reported_by: atmo_core::Did,
                    pub subject: crate::com::atproto::moderation::create_report::output::Subject,
                }
                pub mod input {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum ReasonType {
                    #[serde(rename = "com.atproto.moderation.defs#reasonSpam")]
                    ComAtprotoModerationDefsReasonSpam,
                    #[serde(rename = "com.atproto.moderation.defs#reasonViolation")]
                    ComAtprotoModerationDefsReasonViolation,
                    #[serde(rename = "com.atproto.moderation.defs#reasonMisleading")]
                    ComAtprotoModerationDefsReasonMisleading,
                    #[serde(rename = "com.atproto.moderation.defs#reasonSexual")]
                    ComAtprotoModerationDefsReasonSexual,
                    #[serde(rename = "com.atproto.moderation.defs#reasonRude")]
                    ComAtprotoModerationDefsReasonRude,
                    #[serde(rename = "com.atproto.moderation.defs#reasonOther")]
                    ComAtprotoModerationDefsReasonOther,
                    #[serde(rename = "com.atproto.moderation.defs#reasonAppeal")]
                    ComAtprotoModerationDefsReasonAppeal,
                    #[serde(untagged)]
                    Other(String),
                }
                impl ReasonType {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ComAtprotoModerationDefsReasonSpam => {
                                "com.atproto.moderation.defs#reasonSpam"
                            }
                            Self::ComAtprotoModerationDefsReasonViolation => {
                                "com.atproto.moderation.defs#reasonViolation"
                            }
                            Self::ComAtprotoModerationDefsReasonMisleading => {
                                "com.atproto.moderation.defs#reasonMisleading"
                            }
                            Self::ComAtprotoModerationDefsReasonSexual => {
                                "com.atproto.moderation.defs#reasonSexual"
                            }
                            Self::ComAtprotoModerationDefsReasonRude => {
                                "com.atproto.moderation.defs#reasonRude"
                            }
                            Self::ComAtprotoModerationDefsReasonOther => {
                                "com.atproto.moderation.defs#reasonOther"
                            }
                            Self::ComAtprotoModerationDefsReasonAppeal => {
                                "com.atproto.moderation.defs#reasonAppeal"
                            }
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for ReasonType {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
            }
        }
        pub mod repo {
            #[derive(Debug)]
            pub struct ApplyWrites;
            impl atmo_core::xrpc::Request for ApplyWrites {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::repo::apply_writes::Input;
                type Output = crate::com::atproto::repo::apply_writes::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateRecord;
            impl atmo_core::xrpc::Request for CreateRecord {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::repo::create_record::Input;
                type Output = crate::com::atproto::repo::create_record::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteRecord;
            impl atmo_core::xrpc::Request for DeleteRecord {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::repo::delete_record::Input;
                type Output = crate::com::atproto::repo::delete_record::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DescribeRepo;
            impl atmo_core::xrpc::Request for DescribeRepo {
                type Params = crate::com::atproto::repo::describe_repo::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::repo::describe_repo::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::com::atproto::repo::get_record::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::repo::get_record::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ImportRepo;
            impl atmo_core::xrpc::Request for ImportRepo {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ListMissingBlobs;
            impl atmo_core::xrpc::Request for ListMissingBlobs {
                type Params = crate::com::atproto::repo::list_missing_blobs::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::repo::list_missing_blobs::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ListRecords;
            impl atmo_core::xrpc::Request for ListRecords {
                type Params = crate::com::atproto::repo::list_records::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::repo::list_records::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct PutRecord;
            impl atmo_core::xrpc::Request for PutRecord {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::repo::put_record::Input;
                type Output = crate::com::atproto::repo::put_record::Output;
                type Error = String;
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
            #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
            pub struct StrongRef {
                pub cid: atmo_core::CidString,
                pub uri: atmo_core::AtUri,
            }
            #[derive(Debug)]
            pub struct UploadBlob;
            impl atmo_core::xrpc::Request for UploadBlob {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::repo::upload_blob::Output;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Create {
                    pub collection: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey: std::option::Option<std::string::String>,
                    pub value: atmo_core::Unknown,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct CreateResult {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(rename = "validationStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::apply_writes::create_result::ValidationStatus,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Delete {
                    pub collection: atmo_core::Nsid,
                    pub rkey: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct DeleteResult {}
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    InvalidSwap,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::InvalidSwap => "InvalidSwap",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub repo: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(rename = "swapCommit")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                    pub writes:
                        std::vec::Vec<crate::com::atproto::repo::apply_writes::input::Writes>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub results: std::option::Option<
                        std::vec::Vec<crate::com::atproto::repo::apply_writes::output::Results>,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Update {
                    pub collection: atmo_core::Nsid,
                    pub rkey: std::string::String,
                    pub value: atmo_core::Unknown,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct UpdateResult {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(rename = "validationStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::apply_writes::update_result::ValidationStatus,
                    >,
                }
                pub mod create_result {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ValidationStatus {
                        #[serde(rename = "valid")]
                        Valid,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ValidationStatus {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Valid => "valid",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ValidationStatus {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod input {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Writes {
                        Create(crate::com::atproto::repo::apply_writes::Create),
                        Delete(crate::com::atproto::repo::apply_writes::Delete),
                        Update(crate::com::atproto::repo::apply_writes::Update),
                    }
                    impl<'de> serde::Deserialize<'de> for Writes {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "com.atproto.repo.applyWrites#create" => crate :: com :: atproto :: repo :: apply_writes :: Create :: deserialize (map_des) . map (Self :: Create) , "com.atproto.repo.applyWrites#delete" => crate :: com :: atproto :: repo :: apply_writes :: Delete :: deserialize (map_des) . map (Self :: Delete) , "com.atproto.repo.applyWrites#update" => crate :: com :: atproto :: repo :: apply_writes :: Update :: deserialize (map_des) . map (Self :: Update) , other => return Err (D :: Error :: unknown_variant (other , & ["com.atproto.repo.applyWrites#create" , "com.atproto.repo.applyWrites#delete" , "com.atproto.repo.applyWrites#update" ,])) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "com.atproto.repo.applyWrites#create" => crate :: com :: atproto :: repo :: apply_writes :: Create :: deserialize (map_des) . map (Self :: Create) , "com.atproto.repo.applyWrites#delete" => crate :: com :: atproto :: repo :: apply_writes :: Delete :: deserialize (map_des) . map (Self :: Delete) , "com.atproto.repo.applyWrites#update" => crate :: com :: atproto :: repo :: apply_writes :: Update :: deserialize (map_des) . map (Self :: Update) , other => return Err (D :: Error :: unknown_variant (other , & ["com.atproto.repo.applyWrites#create" , "com.atproto.repo.applyWrites#delete" , "com.atproto.repo.applyWrites#update" ,])) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Results {
                        CreateResult(crate::com::atproto::repo::apply_writes::CreateResult),
                        DeleteResult(crate::com::atproto::repo::apply_writes::DeleteResult),
                        UpdateResult(crate::com::atproto::repo::apply_writes::UpdateResult),
                    }
                    impl<'de> serde::Deserialize<'de> for Results {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "com.atproto.repo.applyWrites#createResult" => crate :: com :: atproto :: repo :: apply_writes :: CreateResult :: deserialize (map_des) . map (Self :: CreateResult) , "com.atproto.repo.applyWrites#deleteResult" => crate :: com :: atproto :: repo :: apply_writes :: DeleteResult :: deserialize (map_des) . map (Self :: DeleteResult) , "com.atproto.repo.applyWrites#updateResult" => crate :: com :: atproto :: repo :: apply_writes :: UpdateResult :: deserialize (map_des) . map (Self :: UpdateResult) , other => return Err (D :: Error :: unknown_variant (other , & ["com.atproto.repo.applyWrites#createResult" , "com.atproto.repo.applyWrites#deleteResult" , "com.atproto.repo.applyWrites#updateResult" ,])) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "com.atproto.repo.applyWrites#createResult" => crate :: com :: atproto :: repo :: apply_writes :: CreateResult :: deserialize (map_des) . map (Self :: CreateResult) , "com.atproto.repo.applyWrites#deleteResult" => crate :: com :: atproto :: repo :: apply_writes :: DeleteResult :: deserialize (map_des) . map (Self :: DeleteResult) , "com.atproto.repo.applyWrites#updateResult" => crate :: com :: atproto :: repo :: apply_writes :: UpdateResult :: deserialize (map_des) . map (Self :: UpdateResult) , other => return Err (D :: Error :: unknown_variant (other , & ["com.atproto.repo.applyWrites#createResult" , "com.atproto.repo.applyWrites#deleteResult" , "com.atproto.repo.applyWrites#updateResult" ,])) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod update_result {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ValidationStatus {
                        #[serde(rename = "valid")]
                        Valid,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ValidationStatus {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Valid => "valid",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ValidationStatus {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod create_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    InvalidSwap,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::InvalidSwap => "InvalidSwap",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub record: atmo_core::Unknown,
                    pub repo: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "swapCommit")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(rename = "validationStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::create_record::output::ValidationStatus,
                    >,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ValidationStatus {
                        #[serde(rename = "valid")]
                        Valid,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ValidationStatus {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Valid => "valid",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ValidationStatus {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct CommitMeta {
                    pub cid: atmo_core::CidString,
                    pub rev: std::string::String,
                }
            }
            pub mod delete_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    InvalidSwap,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::InvalidSwap => "InvalidSwap",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub repo: atmo_core::AtIdentifier,
                    pub rkey: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "swapCommit")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(rename = "swapRecord")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_record: std::option::Option<atmo_core::CidString>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                }
            }
            pub mod describe_repo {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub collections: std::vec::Vec<atmo_core::Nsid>,
                    pub did: atmo_core::Did,
                    #[serde(rename = "didDoc")]
                    pub did_doc: atmo_core::Unknown,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "handleIsCorrect")]
                    pub handle_is_correct: bool,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub repo: atmo_core::AtIdentifier,
                }
            }
            pub mod get_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RecordNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RecordNotFound => "RecordNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blobs:
                        std::vec::Vec<crate::com::atproto::repo::list_missing_blobs::RecordBlob>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordBlob {
                    pub cid: atmo_core::CidString,
                    #[serde(rename = "recordUri")]
                    pub record_uri: atmo_core::AtUri,
                }
            }
            pub mod list_records {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub records: std::vec::Vec<crate::com::atproto::repo::list_records::Record>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub collection: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    pub repo: atmo_core::AtIdentifier,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reverse: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "rkeyEnd")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey_end: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "rkeyStart")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rkey_start: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Record {
                    pub cid: atmo_core::CidString,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
            }
            pub mod put_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    InvalidSwap,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::InvalidSwap => "InvalidSwap",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub collection: atmo_core::Nsid,
                    pub record: atmo_core::Unknown,
                    pub repo: atmo_core::AtIdentifier,
                    pub rkey: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "swapCommit")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_commit: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(rename = "swapRecord")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub swap_record: std::option::Option<atmo_core::Nullable<atmo_core::CidString>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validate: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub commit: std::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
                    pub uri: atmo_core::AtUri,
                    #[serde(default)]
                    #[serde(rename = "validationStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub validation_status: std::option::Option<
                        crate::com::atproto::repo::put_record::output::ValidationStatus,
                    >,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ValidationStatus {
                        #[serde(rename = "valid")]
                        Valid,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ValidationStatus {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Valid => "valid",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ValidationStatus {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod upload_blob {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub blob: atmo_core::Blob,
                }
            }
        }
        pub mod server {
            #[derive(Debug)]
            pub struct ActivateAccount;
            impl atmo_core::xrpc::Request for ActivateAccount {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CheckAccountStatus;
            impl atmo_core::xrpc::Request for CheckAccountStatus {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::check_account_status::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ConfirmEmail;
            impl atmo_core::xrpc::Request for ConfirmEmail {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::confirm_email::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateAccount;
            impl atmo_core::xrpc::Request for CreateAccount {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::create_account::Input;
                type Output = crate::com::atproto::server::create_account::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateAppPassword;
            impl atmo_core::xrpc::Request for CreateAppPassword {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::create_app_password::Input;
                type Output = crate::com::atproto::server::create_app_password::AppPassword;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateInviteCode;
            impl atmo_core::xrpc::Request for CreateInviteCode {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::create_invite_code::Input;
                type Output = crate::com::atproto::server::create_invite_code::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateInviteCodes;
            impl atmo_core::xrpc::Request for CreateInviteCodes {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::create_invite_codes::Input;
                type Output = crate::com::atproto::server::create_invite_codes::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct CreateSession;
            impl atmo_core::xrpc::Request for CreateSession {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::create_session::Input;
                type Output = crate::com::atproto::server::create_session::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeactivateAccount;
            impl atmo_core::xrpc::Request for DeactivateAccount {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::deactivate_account::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteAccount;
            impl atmo_core::xrpc::Request for DeleteAccount {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::delete_account::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteSession;
            impl atmo_core::xrpc::Request for DeleteSession {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DescribeServer;
            impl atmo_core::xrpc::Request for DescribeServer {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::describe_server::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetAccountInviteCodes;
            impl atmo_core::xrpc::Request for GetAccountInviteCodes {
                type Params = crate::com::atproto::server::get_account_invite_codes::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::get_account_invite_codes::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetServiceAuth;
            impl atmo_core::xrpc::Request for GetServiceAuth {
                type Params = crate::com::atproto::server::get_service_auth::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::get_service_auth::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetSession;
            impl atmo_core::xrpc::Request for GetSession {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::get_session::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ListAppPasswords;
            impl atmo_core::xrpc::Request for ListAppPasswords {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::list_app_passwords::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct RefreshSession;
            impl atmo_core::xrpc::Request for RefreshSession {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::refresh_session::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RequestAccountDelete;
            impl atmo_core::xrpc::Request for RequestAccountDelete {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RequestEmailConfirmation;
            impl atmo_core::xrpc::Request for RequestEmailConfirmation {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RequestEmailUpdate;
            impl atmo_core::xrpc::Request for RequestEmailUpdate {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::server::request_email_update::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RequestPasswordReset;
            impl atmo_core::xrpc::Request for RequestPasswordReset {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::request_password_reset::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ReserveSigningKey;
            impl atmo_core::xrpc::Request for ReserveSigningKey {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::reserve_signing_key::Input;
                type Output = crate::com::atproto::server::reserve_signing_key::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ResetPassword;
            impl atmo_core::xrpc::Request for ResetPassword {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::reset_password::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RevokeAppPassword;
            impl atmo_core::xrpc::Request for RevokeAppPassword {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::revoke_app_password::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct UpdateEmail;
            impl atmo_core::xrpc::Request for UpdateEmail {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::server::update_email::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub activated: bool,
                    #[serde(rename = "expectedBlobs")]
                    pub expected_blobs: i64,
                    #[serde(rename = "importedBlobs")]
                    pub imported_blobs: i64,
                    #[serde(rename = "indexedRecords")]
                    pub indexed_records: i64,
                    #[serde(rename = "privateStateValues")]
                    pub private_state_values: i64,
                    #[serde(rename = "repoBlocks")]
                    pub repo_blocks: i64,
                    #[serde(rename = "repoCommit")]
                    pub repo_commit: atmo_core::CidString,
                    #[serde(rename = "repoRev")]
                    pub repo_rev: std::string::String,
                    #[serde(rename = "validDid")]
                    pub valid_did: bool,
                }
            }
            pub mod confirm_email {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    AccountNotFound,
                    ExpiredToken,
                    InvalidToken,
                    InvalidEmail,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AccountNotFound => "AccountNotFound",
                            Self::ExpiredToken => "ExpiredToken",
                            Self::InvalidToken => "InvalidToken",
                            Self::InvalidEmail => "InvalidEmail",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod create_account {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    InvalidHandle,
                    InvalidPassword,
                    InvalidInviteCode,
                    HandleNotAvailable,
                    UnsupportedDomain,
                    UnresolvableDid,
                    IncompatibleDidDoc,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::InvalidHandle => "InvalidHandle",
                            Self::InvalidPassword => "InvalidPassword",
                            Self::InvalidInviteCode => "InvalidInviteCode",
                            Self::HandleNotAvailable => "HandleNotAvailable",
                            Self::UnsupportedDomain => "UnsupportedDomain",
                            Self::UnresolvableDid => "UnresolvableDid",
                            Self::IncompatibleDidDoc => "IncompatibleDidDoc",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(rename = "inviteCode")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_code: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub password: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "plcOp")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub plc_op: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(rename = "recoveryKey")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub recovery_key: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "verificationCode")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_code: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "verificationPhone")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub verification_phone: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "accessJwt")]
                    pub access_jwt: std::string::String,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "didDoc")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "refreshJwt")]
                    pub refresh_jwt: std::string::String,
                }
            }
            pub mod create_app_password {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    pub name: std::string::String,
                    pub password: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    AccountTakedown,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AccountTakedown => "AccountTakedown",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
            }
            pub mod create_invite_code {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(rename = "forAccount")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub for_account: std::option::Option<atmo_core::Did>,
                    #[serde(rename = "useCount")]
                    pub use_count: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub code: std::string::String,
                }
            }
            pub mod create_invite_codes {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AccountCodes {
                    pub account: std::string::String,
                    pub codes: std::vec::Vec<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "codeCount")]
                    pub code_count: i64,
                    #[serde(default)]
                    #[serde(rename = "forAccounts")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub for_accounts: std::option::Option<std::vec::Vec<atmo_core::Did>>,
                    #[serde(rename = "useCount")]
                    pub use_count: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<
                        crate::com::atproto::server::create_invite_codes::AccountCodes,
                    >,
                }
            }
            pub mod create_session {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    AccountTakedown,
                    AuthFactorTokenRequired,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AccountTakedown => "AccountTakedown",
                            Self::AuthFactorTokenRequired => "AuthFactorTokenRequired",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(rename = "authFactorToken")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub auth_factor_token: std::option::Option<std::string::String>,
                    pub identifier: std::string::String,
                    pub password: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "accessJwt")]
                    pub access_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "didDoc")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "emailAuthFactor")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "emailConfirmed")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed: std::option::Option<bool>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "refreshJwt")]
                    pub refresh_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::com::atproto::server::create_session::output::Status,
                    >,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod deactivate_account {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(rename = "deleteAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub delete_after: std::option::Option<atmo_core::DateTimeString>,
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCode {
                    pub available: i64,
                    pub code: std::string::String,
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(rename = "createdBy")]
                    pub created_by: std::string::String,
                    pub disabled: bool,
                    #[serde(rename = "forAccount")]
                    pub for_account: std::string::String,
                    pub uses: std::vec::Vec<crate::com::atproto::server::defs::InviteCodeUse>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct InviteCodeUse {
                    #[serde(rename = "usedAt")]
                    pub used_at: atmo_core::DateTimeString,
                    #[serde(rename = "usedBy")]
                    pub used_by: atmo_core::Did,
                }
            }
            pub mod delete_account {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    ExpiredToken,
                    InvalidToken,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ExpiredToken => "ExpiredToken",
                            Self::InvalidToken => "InvalidToken",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub password: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod describe_server {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Contact {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Links {
                    #[serde(default)]
                    #[serde(rename = "privacyPolicy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privacy_policy: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "termsOfService")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub terms_of_service: std::option::Option<url::Url>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "availableUserDomains")]
                    pub available_user_domains: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub contact:
                        std::option::Option<crate::com::atproto::server::describe_server::Contact>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "inviteCodeRequired")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_code_required: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub links:
                        std::option::Option<crate::com::atproto::server::describe_server::Links>,
                    #[serde(default)]
                    #[serde(rename = "phoneVerificationRequired")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub phone_verification_required: std::option::Option<bool>,
                }
            }
            pub mod get_account_invite_codes {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    DuplicateCreate,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::DuplicateCreate => "DuplicateCreate",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub codes: std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(rename = "createAvailable")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub create_available: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "includeUsed")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_used: std::option::Option<bool>,
                }
            }
            pub mod get_service_auth {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BadExpiration,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BadExpiration => "BadExpiration",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub token: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "didDoc")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "emailAuthFactor")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "emailConfirmed")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed: std::option::Option<bool>,
                    pub handle: atmo_core::Handle,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::com::atproto::server::get_session::output::Status,
                    >,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod list_app_passwords {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AppPassword {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    pub name: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub privileged: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    AccountTakedown,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AccountTakedown => "AccountTakedown",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub passwords:
                        std::vec::Vec<crate::com::atproto::server::list_app_passwords::AppPassword>,
                }
            }
            pub mod refresh_session {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    AccountTakedown,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::AccountTakedown => "AccountTakedown",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "accessJwt")]
                    pub access_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "didDoc")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did_doc: std::option::Option<atmo_core::Unknown>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "refreshJwt")]
                    pub refresh_jwt: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::com::atproto::server::refresh_session::output::Status,
                    >,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod request_email_update {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "tokenRequired")]
                    pub token_required: bool,
                }
            }
            pub mod request_password_reset {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                }
            }
            pub mod reserve_signing_key {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub did: std::option::Option<atmo_core::Did>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "signingKey")]
                    pub signing_key: std::string::String,
                }
            }
            pub mod reset_password {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    ExpiredToken,
                    InvalidToken,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ExpiredToken => "ExpiredToken",
                            Self::InvalidToken => "InvalidToken",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub password: std::string::String,
                    pub token: std::string::String,
                }
            }
            pub mod revoke_app_password {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                }
            }
            pub mod update_email {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    ExpiredToken,
                    InvalidToken,
                    TokenRequired,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ExpiredToken => "ExpiredToken",
                            Self::InvalidToken => "InvalidToken",
                            Self::TokenRequired => "TokenRequired",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub email: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "emailAuthFactor")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_auth_factor: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub token: std::option::Option<std::string::String>,
                }
            }
        }
        pub mod sync {
            #[derive(Debug)]
            pub struct GetBlob;
            impl atmo_core::xrpc::Request for GetBlob {
                type Params = crate::com::atproto::sync::get_blob::Params;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetBlocks;
            impl atmo_core::xrpc::Request for GetBlocks {
                type Params = crate::com::atproto::sync::get_blocks::Params;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetCheckout;
            impl atmo_core::xrpc::Request for GetCheckout {
                type Params = crate::com::atproto::sync::get_checkout::Params;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetHead;
            impl atmo_core::xrpc::Request for GetHead {
                type Params = crate::com::atproto::sync::get_head::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::sync::get_head::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetLatestCommit;
            impl atmo_core::xrpc::Request for GetLatestCommit {
                type Params = crate::com::atproto::sync::get_latest_commit::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::sync::get_latest_commit::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::com::atproto::sync::get_record::Params;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRepo;
            impl atmo_core::xrpc::Request for GetRepo {
                type Params = crate::com::atproto::sync::get_repo::Params;
                type Input = atmo_core::Nothing;
                type Output = atmo_core::Nothing;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRepoStatus;
            impl atmo_core::xrpc::Request for GetRepoStatus {
                type Params = crate::com::atproto::sync::get_repo_status::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::sync::get_repo_status::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ListBlobs;
            impl atmo_core::xrpc::Request for ListBlobs {
                type Params = crate::com::atproto::sync::list_blobs::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::sync::list_blobs::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct ListRepos;
            impl atmo_core::xrpc::Request for ListRepos {
                type Params = crate::com::atproto::sync::list_repos::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::sync::list_repos::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct NotifyOfUpdate;
            impl atmo_core::xrpc::Request for NotifyOfUpdate {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::sync::notify_of_update::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct RequestCrawl;
            impl atmo_core::xrpc::Request for RequestCrawl {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::sync::request_crawl::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BlobNotFound,
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BlobNotFound => "BlobNotFound",
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub cid: atmo_core::CidString,
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_blocks {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    BlockNotFound,
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::BlockNotFound => "BlockNotFound",
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub cids: std::vec::Vec<atmo_core::CidString>,
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_checkout {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_head {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    HeadNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::HeadNotFound => "HeadNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub root: atmo_core::CidString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_latest_commit {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cid: atmo_core::CidString,
                    pub rev: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RecordNotFound,
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RecordNotFound => "RecordNotFound",
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub since: std::option::Option<std::string::String>,
                }
            }
            pub mod get_repo_status {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RepoNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RepoNotFound => "RepoNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub active: bool,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub rev: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::com::atproto::sync::get_repo_status::output::Status,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
                pub mod output {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod list_blobs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RepoNotFound,
                    RepoTakendown,
                    RepoSuspended,
                    RepoDeactivated,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RepoNotFound => "RepoNotFound",
                            Self::RepoTakendown => "RepoTakendown",
                            Self::RepoSuspended => "RepoSuspended",
                            Self::RepoDeactivated => "RepoDeactivated",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub cids: std::vec::Vec<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub repos: std::vec::Vec<crate::com::atproto::sync::list_repos::Repo>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Repo {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub active: std::option::Option<bool>,
                    pub did: atmo_core::Did,
                    pub head: atmo_core::CidString,
                    pub rev: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status:
                        std::option::Option<crate::com::atproto::sync::list_repos::repo::Status>,
                }
                pub mod repo {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod notify_of_update {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub hostname: std::string::String,
                }
            }
            pub mod request_crawl {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub hostname: std::string::String,
                }
            }
            pub mod subscribe_repos {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Account {
                    pub active: bool,
                    pub did: atmo_core::Did,
                    pub seq: i64,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::com::atproto::sync::subscribe_repos::account::Status,
                    >,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Commit {
                    pub blobs: std::vec::Vec<atmo_core::CidLink>,
                    #[serde(with = "atmo_core::bytes::serde")]
                    pub blocks: bytes::Bytes,
                    pub commit: atmo_core::CidLink,
                    pub ops: std::vec::Vec<crate::com::atproto::sync::subscribe_repos::RepoOp>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prev: std::option::Option<atmo_core::Nullable<atmo_core::CidLink>>,
                    pub rebase: bool,
                    pub repo: atmo_core::Did,
                    pub rev: std::string::String,
                    pub seq: i64,
                    pub since: atmo_core::Nullable<std::string::String>,
                    pub time: atmo_core::DateTimeString,
                    #[serde(rename = "tooBig")]
                    pub too_big: bool,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Handle {
                    pub did: atmo_core::Did,
                    pub handle: atmo_core::Handle,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Identity {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub handle: std::option::Option<atmo_core::Handle>,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Info {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub message: std::option::Option<std::string::String>,
                    pub name: crate::com::atproto::sync::subscribe_repos::info::Name,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Migrate {
                    pub did: atmo_core::Did,
                    #[serde(rename = "migrateTo")]
                    pub migrate_to: atmo_core::Nullable<std::string::String>,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoOp {
                    pub action: crate::com::atproto::sync::subscribe_repos::repo_op::Action,
                    pub cid: atmo_core::Nullable<atmo_core::CidLink>,
                    pub path: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Tombstone {
                    pub did: atmo_core::Did,
                    pub seq: i64,
                    pub time: atmo_core::DateTimeString,
                }
                pub mod account {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deleted => "deleted",
                                Self::Deactivated => "deactivated",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod info {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Name {
                        OutdatedCursor,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Name {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::OutdatedCursor => "OutdatedCursor",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Name {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod repo_op {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
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
                    impl Action {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Create => "create",
                                Self::Update => "update",
                                Self::Delete => "delete",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Action {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
        pub mod temp {
            #[derive(Debug)]
            pub struct CheckSignupQueue;
            impl atmo_core::xrpc::Request for CheckSignupQueue {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::temp::check_signup_queue::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct FetchLabels;
            impl atmo_core::xrpc::Request for FetchLabels {
                type Params = crate::com::atproto::temp::fetch_labels::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::com::atproto::temp::fetch_labels::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct RequestPhoneVerification;
            impl atmo_core::xrpc::Request for RequestPhoneVerification {
                type Params = atmo_core::Nothing;
                type Input = crate::com::atproto::temp::request_phone_verification::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub activated: bool,
                    #[serde(default)]
                    #[serde(rename = "estimatedTimeMs")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub estimated_time_ms: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "placeInQueue")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub place_in_queue: std::option::Option<i64>,
                }
            }
            pub mod fetch_labels {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub labels: std::vec::Vec<crate::com::atproto::label::defs::Label>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "phoneNumber")]
                    pub phone_number: std::string::String,
                }
            }
        }
    }
}
pub mod tools {
    pub mod ozone {
        pub mod communication {
            #[derive(Debug)]
            pub struct CreateTemplate;
            impl atmo_core::xrpc::Request for CreateTemplate {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::communication::create_template::Input;
                type Output = crate::tools::ozone::communication::defs::TemplateView;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteTemplate;
            impl atmo_core::xrpc::Request for DeleteTemplate {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::communication::delete_template::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ListTemplates;
            impl atmo_core::xrpc::Request for ListTemplates {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::communication::list_templates::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct UpdateTemplate;
            impl atmo_core::xrpc::Request for UpdateTemplate {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::communication::update_template::Input;
                type Output = crate::tools::ozone::communication::defs::TemplateView;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    DuplicateTemplateName,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::DuplicateTemplateName => "DuplicateTemplateName",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "contentMarkdown")]
                    pub content_markdown: std::string::String,
                    #[serde(default)]
                    #[serde(rename = "createdBy")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct TemplateView {
                    #[serde(rename = "contentMarkdown")]
                    pub content_markdown: std::string::String,
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    pub disabled: bool,
                    pub id: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub lang: std::option::Option<std::string::String>,
                    #[serde(rename = "lastUpdatedBy")]
                    pub last_updated_by: atmo_core::Did,
                    pub name: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<std::string::String>,
                    #[serde(rename = "updatedAt")]
                    pub updated_at: atmo_core::DateTimeString,
                }
            }
            pub mod delete_template {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub id: std::string::String,
                }
            }
            pub mod list_templates {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(rename = "communicationTemplates")]
                    pub communication_templates:
                        std::vec::Vec<crate::tools::ozone::communication::defs::TemplateView>,
                }
            }
            pub mod update_template {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    DuplicateTemplateName,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::DuplicateTemplateName => "DuplicateTemplateName",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(rename = "contentMarkdown")]
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
                    #[serde(rename = "updatedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_by: std::option::Option<atmo_core::Did>,
                }
            }
        }
        pub mod moderation {
            #[derive(Debug)]
            pub struct EmitEvent;
            impl atmo_core::xrpc::Request for EmitEvent {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::moderation::emit_event::Input;
                type Output = crate::tools::ozone::moderation::defs::ModEventView;
                type Error = String;
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
            #[derive(Debug)]
            pub struct GetEvent;
            impl atmo_core::xrpc::Request for GetEvent {
                type Params = crate::tools::ozone::moderation::get_event::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::defs::ModEventViewDetail;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRecord;
            impl atmo_core::xrpc::Request for GetRecord {
                type Params = crate::tools::ozone::moderation::get_record::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::defs::RecordViewDetail;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRecords;
            impl atmo_core::xrpc::Request for GetRecords {
                type Params = crate::tools::ozone::moderation::get_records::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::get_records::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRepo;
            impl atmo_core::xrpc::Request for GetRepo {
                type Params = crate::tools::ozone::moderation::get_repo::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::defs::RepoViewDetail;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct GetRepos;
            impl atmo_core::xrpc::Request for GetRepos {
                type Params = crate::tools::ozone::moderation::get_repos::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::get_repos::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct QueryEvents;
            impl atmo_core::xrpc::Request for QueryEvents {
                type Params = crate::tools::ozone::moderation::query_events::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::query_events::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct QueryStatuses;
            impl atmo_core::xrpc::Request for QueryStatuses {
                type Params = crate::tools::ozone::moderation::query_statuses::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::query_statuses::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchRepos;
            impl atmo_core::xrpc::Request for SearchRepos {
                type Params = crate::tools::ozone::moderation::search_repos::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::moderation::search_repos::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AccountEvent {
                    pub active: bool,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub status: std::option::Option<
                        crate::tools::ozone::moderation::defs::account_event::Status,
                    >,
                    pub timestamp: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct AccountHosting {
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "deactivatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "deletedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deleted_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "reactivatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub status: crate::tools::ozone::moderation::defs::account_hosting::Status,
                    #[serde(default)]
                    #[serde(rename = "updatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct BlobView {
                    pub cid: atmo_core::CidString,
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub details: std::option::Option<
                        crate::tools::ozone::moderation::defs::blob_view::Details,
                    >,
                    #[serde(rename = "mimeType")]
                    pub mime_type: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub moderation:
                        std::option::Option<crate::tools::ozone::moderation::defs::Moderation>,
                    pub size: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct IdentityEvent {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub handle: std::option::Option<atmo_core::Handle>,
                    #[serde(default)]
                    #[serde(rename = "pdsHost")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub pds_host: std::option::Option<url::Url>,
                    pub timestamp: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tombstone: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ImageDetails {
                    pub height: i64,
                    pub width: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventAcknowledge {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventComment {
                    pub comment: std::string::String,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sticky: std::option::Option<bool>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventDivert {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEmail {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub content: std::option::Option<std::string::String>,
                    #[serde(rename = "subjectLine")]
                    pub subject_line: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventEscalate {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventLabel {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(rename = "createLabelVals")]
                    pub create_label_vals: std::vec::Vec<std::string::String>,
                    #[serde(rename = "negateLabelVals")]
                    pub negate_label_vals: std::vec::Vec<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMute {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(rename = "durationInHours")]
                    pub duration_in_hours: i64,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventMuteReporter {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "durationInHours")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub duration_in_hours: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReport {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "isReporterMuted")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub is_reporter_muted: std::option::Option<bool>,
                    #[serde(rename = "reportType")]
                    pub report_type: crate::com::atproto::moderation::defs::ReasonType,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventResolveAppeal {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventReverseTakedown {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTag {
                    pub add: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub remove: std::vec::Vec<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventTakedown {
                    #[serde(default)]
                    #[serde(rename = "acknowledgeAccountSubjects")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub acknowledge_account_subjects: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "durationInHours")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub duration_in_hours: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmute {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventUnmuteReporter {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventView {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(rename = "createdBy")]
                    pub created_by: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "creatorHandle")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub creator_handle: std::option::Option<std::string::String>,
                    pub event: crate::tools::ozone::moderation::defs::mod_event_view::Event,
                    pub id: i64,
                    pub subject: crate::tools::ozone::moderation::defs::mod_event_view::Subject,
                    #[serde(rename = "subjectBlobCids")]
                    pub subject_blob_cids: std::vec::Vec<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "subjectHandle")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_handle: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModEventViewDetail {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(rename = "createdBy")]
                    pub created_by: atmo_core::Did,
                    pub event: crate::tools::ozone::moderation::defs::mod_event_view_detail::Event,
                    pub id: i64,
                    pub subject:
                        crate::tools::ozone::moderation::defs::mod_event_view_detail::Subject,
                    #[serde(rename = "subjectBlobs")]
                    pub subject_blobs:
                        std::vec::Vec<crate::tools::ozone::moderation::defs::BlobView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Moderation {
                    #[serde(default)]
                    #[serde(rename = "subjectStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ModerationDetail {
                    #[serde(default)]
                    #[serde(rename = "subjectStatus")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_status: std::option::Option<
                        crate::tools::ozone::moderation::defs::SubjectStatusView,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordEvent {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    pub op: crate::tools::ozone::moderation::defs::record_event::Op,
                    pub timestamp: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordHosting {
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "deletedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deleted_at: std::option::Option<atmo_core::DateTimeString>,
                    pub status: crate::tools::ozone::moderation::defs::record_hosting::Status,
                    #[serde(default)]
                    #[serde(rename = "updatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo_core::DateTimeString>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordView {
                    #[serde(rename = "blobCids")]
                    pub blob_cids: std::vec::Vec<atmo_core::CidString>,
                    pub cid: atmo_core::CidString,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    pub repo: crate::tools::ozone::moderation::defs::RepoView,
                    pub uri: atmo_core::AtUri,
                    pub value: atmo_core::Unknown,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewDetail {
                    pub blobs: std::vec::Vec<crate::tools::ozone::moderation::defs::BlobView>,
                    pub cid: atmo_core::CidString,
                    #[serde(rename = "indexedAt")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RecordViewNotFound {
                    pub uri: atmo_core::AtUri,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoView {
                    #[serde(default)]
                    #[serde(rename = "deactivatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(rename = "inviteNote")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "invitedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(rename = "invitesDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    pub moderation: crate::tools::ozone::moderation::defs::Moderation,
                    #[serde(rename = "relatedRecords")]
                    pub related_records: std::vec::Vec<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(rename = "threatSignatures")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threat_signatures: std::option::Option<
                        std::vec::Vec<crate::com::atproto::admin::defs::ThreatSignature>,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewDetail {
                    #[serde(default)]
                    #[serde(rename = "deactivatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub deactivated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "emailConfirmedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub email_confirmed_at: std::option::Option<atmo_core::DateTimeString>,
                    pub handle: atmo_core::Handle,
                    #[serde(rename = "indexedAt")]
                    pub indexed_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(rename = "inviteNote")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invite_note: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "invitedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invited_by:
                        std::option::Option<crate::com::atproto::server::defs::InviteCode>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites: std::option::Option<
                        std::vec::Vec<crate::com::atproto::server::defs::InviteCode>,
                    >,
                    #[serde(default)]
                    #[serde(rename = "invitesDisabled")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub invites_disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub labels:
                        std::option::Option<std::vec::Vec<crate::com::atproto::label::defs::Label>>,
                    pub moderation: crate::tools::ozone::moderation::defs::ModerationDetail,
                    #[serde(rename = "relatedRecords")]
                    pub related_records: std::vec::Vec<atmo_core::Unknown>,
                    #[serde(default)]
                    #[serde(rename = "threatSignatures")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub threat_signatures: std::option::Option<
                        std::vec::Vec<crate::com::atproto::admin::defs::ThreatSignature>,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct RepoViewNotFound {
                    pub did: atmo_core::Did,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum SubjectReviewState {
                    #[serde(rename = "#reviewOpen")]
                    ReviewOpen,
                    #[serde(rename = "#reviewEscalated")]
                    ReviewEscalated,
                    #[serde(rename = "#reviewClosed")]
                    ReviewClosed,
                    #[serde(rename = "#reviewNone")]
                    ReviewNone,
                    #[serde(untagged)]
                    Other(String),
                }
                impl SubjectReviewState {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::ReviewOpen => "#reviewOpen",
                            Self::ReviewEscalated => "#reviewEscalated",
                            Self::ReviewClosed => "#reviewClosed",
                            Self::ReviewNone => "#reviewNone",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for SubjectReviewState {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SubjectStatusView {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appealed: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting: std::option::Option<
                        crate::tools::ozone::moderation::defs::subject_status_view::Hosting,
                    >,
                    pub id: i64,
                    #[serde(default)]
                    #[serde(rename = "lastAppealedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_appealed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "lastReportedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reported_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "lastReviewedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "lastReviewedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(rename = "muteReportingUntil")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_reporting_until: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "muteUntil")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub mute_until: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(rename = "reviewState")]
                    pub review_state: crate::tools::ozone::moderation::defs::SubjectReviewState,
                    pub subject:
                        crate::tools::ozone::moderation::defs::subject_status_view::Subject,
                    #[serde(default)]
                    #[serde(rename = "subjectBlobCids")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_blob_cids: std::option::Option<std::vec::Vec<atmo_core::CidString>>,
                    #[serde(default)]
                    #[serde(rename = "subjectRepoHandle")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_repo_handle: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "suspendUntil")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub suspend_until: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takendown: std::option::Option<bool>,
                    #[serde(rename = "updatedAt")]
                    pub updated_at: atmo_core::DateTimeString,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct VideoDetails {
                    pub height: i64,
                    pub length: i64,
                    pub width: i64,
                }
                pub mod account_event {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Status {
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(rename = "deactivated")]
                        Deactivated,
                        #[serde(rename = "deleted")]
                        Deleted,
                        #[serde(rename = "takendown")]
                        Takendown,
                        #[serde(rename = "suspended")]
                        Suspended,
                        #[serde(rename = "tombstoned")]
                        Tombstoned,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Unknown => "unknown",
                                Self::Deactivated => "deactivated",
                                Self::Deleted => "deleted",
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Tombstoned => "tombstoned",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod account_hosting {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Status {
                        #[serde(rename = "takendown")]
                        Takendown,
                        #[serde(rename = "suspended")]
                        Suspended,
                        #[serde(rename = "deleted")]
                        Deleted,
                        #[serde(rename = "deactivated")]
                        Deactivated,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Takendown => "takendown",
                                Self::Suspended => "suspended",
                                Self::Deleted => "deleted",
                                Self::Deactivated => "deactivated",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod blob_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Details {
                        ImageDetails(crate::tools::ozone::moderation::defs::ImageDetails),
                        VideoDetails(crate::tools::ozone::moderation::defs::VideoDetails),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Details {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#imageDetails" => crate :: tools :: ozone :: moderation :: defs :: ImageDetails :: deserialize (map_des) . map (Self :: ImageDetails) , "tools.ozone.moderation.defs#videoDetails" => crate :: tools :: ozone :: moderation :: defs :: VideoDetails :: deserialize (map_des) . map (Self :: VideoDetails) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#imageDetails" => crate :: tools :: ozone :: moderation :: defs :: ImageDetails :: deserialize (map_des) . map (Self :: ImageDetails) , "tools.ozone.moderation.defs#videoDetails" => crate :: tools :: ozone :: moderation :: defs :: VideoDetails :: deserialize (map_des) . map (Self :: VideoDetails) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod mod_event_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Event {
                        AccountEvent(crate::tools::ozone::moderation::defs::AccountEvent),
                        IdentityEvent(crate::tools::ozone::moderation::defs::IdentityEvent),
                        ModEventAcknowledge(
                            crate::tools::ozone::moderation::defs::ModEventAcknowledge,
                        ),
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
                        RecordEvent(crate::tools::ozone::moderation::defs::RecordEvent),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Event {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventDivert" => crate :: tools :: ozone :: moderation :: defs :: ModEventDivert :: deserialize (map_des) . map (Self :: ModEventDivert) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventDivert" => crate :: tools :: ozone :: moderation :: defs :: ModEventDivert :: deserialize (map_des) . map (Self :: ModEventDivert) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        MessageRef(crate::chat::bsky::convo::defs::MessageRef),
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "chat.bsky.convo.defs#messageRef" => {
                                        crate::chat::bsky::convo::defs::MessageRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::MessageRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "chat.bsky.convo.defs#messageRef" => {
                                        crate::chat::bsky::convo::defs::MessageRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::MessageRef)
                                    }
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod mod_event_view_detail {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Event {
                        AccountEvent(crate::tools::ozone::moderation::defs::AccountEvent),
                        IdentityEvent(crate::tools::ozone::moderation::defs::IdentityEvent),
                        ModEventAcknowledge(
                            crate::tools::ozone::moderation::defs::ModEventAcknowledge,
                        ),
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
                        RecordEvent(crate::tools::ozone::moderation::defs::RecordEvent),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Event {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventDivert" => crate :: tools :: ozone :: moderation :: defs :: ModEventDivert :: deserialize (map_des) . map (Self :: ModEventDivert) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventDivert" => crate :: tools :: ozone :: moderation :: defs :: ModEventDivert :: deserialize (map_des) . map (Self :: ModEventDivert) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RecordView(crate::tools::ozone::moderation::defs::RecordView),
                        RecordViewNotFound(
                            crate::tools::ozone::moderation::defs::RecordViewNotFound,
                        ),
                        RepoView(crate::tools::ozone::moderation::defs::RepoView),
                        RepoViewNotFound(crate::tools::ozone::moderation::defs::RepoViewNotFound),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#recordView" => crate :: tools :: ozone :: moderation :: defs :: RecordView :: deserialize (map_des) . map (Self :: RecordView) , "tools.ozone.moderation.defs#recordViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RecordViewNotFound :: deserialize (map_des) . map (Self :: RecordViewNotFound) , "tools.ozone.moderation.defs#repoView" => crate :: tools :: ozone :: moderation :: defs :: RepoView :: deserialize (map_des) . map (Self :: RepoView) , "tools.ozone.moderation.defs#repoViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RepoViewNotFound :: deserialize (map_des) . map (Self :: RepoViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#recordView" => crate :: tools :: ozone :: moderation :: defs :: RecordView :: deserialize (map_des) . map (Self :: RecordView) , "tools.ozone.moderation.defs#recordViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RecordViewNotFound :: deserialize (map_des) . map (Self :: RecordViewNotFound) , "tools.ozone.moderation.defs#repoView" => crate :: tools :: ozone :: moderation :: defs :: RepoView :: deserialize (map_des) . map (Self :: RepoView) , "tools.ozone.moderation.defs#repoViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RepoViewNotFound :: deserialize (map_des) . map (Self :: RepoViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
                pub mod record_event {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Op {
                        #[serde(rename = "create")]
                        Create,
                        #[serde(rename = "update")]
                        Update,
                        #[serde(rename = "delete")]
                        Delete,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Op {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Create => "create",
                                Self::Update => "update",
                                Self::Delete => "delete",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Op {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod record_hosting {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Status {
                        #[serde(rename = "deleted")]
                        Deleted,
                        #[serde(rename = "unknown")]
                        Unknown,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Status {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Deleted => "deleted",
                                Self::Unknown => "unknown",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Status {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
                pub mod subject_status_view {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Hosting {
                        AccountHosting(crate::tools::ozone::moderation::defs::AccountHosting),
                        RecordHosting(crate::tools::ozone::moderation::defs::RecordHosting),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Hosting {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountHosting" => crate :: tools :: ozone :: moderation :: defs :: AccountHosting :: deserialize (map_des) . map (Self :: AccountHosting) , "tools.ozone.moderation.defs#recordHosting" => crate :: tools :: ozone :: moderation :: defs :: RecordHosting :: deserialize (map_des) . map (Self :: RecordHosting) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountHosting" => crate :: tools :: ozone :: moderation :: defs :: AccountHosting :: deserialize (map_des) . map (Self :: AccountHosting) , "tools.ozone.moderation.defs#recordHosting" => crate :: tools :: ozone :: moderation :: defs :: RecordHosting :: deserialize (map_des) . map (Self :: RecordHosting) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod emit_event {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    SubjectHasAction,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::SubjectHasAction => "SubjectHasAction",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(rename = "createdBy")]
                    pub created_by: atmo_core::Did,
                    pub event: crate::tools::ozone::moderation::emit_event::input::Event,
                    pub subject: crate::tools::ozone::moderation::emit_event::input::Subject,
                    #[serde(default)]
                    #[serde(rename = "subjectBlobCids")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_blob_cids: std::option::Option<std::vec::Vec<atmo_core::CidString>>,
                }
                pub mod input {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Event {
                        AccountEvent(crate::tools::ozone::moderation::defs::AccountEvent),
                        IdentityEvent(crate::tools::ozone::moderation::defs::IdentityEvent),
                        ModEventAcknowledge(
                            crate::tools::ozone::moderation::defs::ModEventAcknowledge,
                        ),
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
                        RecordEvent(crate::tools::ozone::moderation::defs::RecordEvent),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Event {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#accountEvent" => crate :: tools :: ozone :: moderation :: defs :: AccountEvent :: deserialize (map_des) . map (Self :: AccountEvent) , "tools.ozone.moderation.defs#identityEvent" => crate :: tools :: ozone :: moderation :: defs :: IdentityEvent :: deserialize (map_des) . map (Self :: IdentityEvent) , "tools.ozone.moderation.defs#modEventAcknowledge" => crate :: tools :: ozone :: moderation :: defs :: ModEventAcknowledge :: deserialize (map_des) . map (Self :: ModEventAcknowledge) , "tools.ozone.moderation.defs#modEventComment" => crate :: tools :: ozone :: moderation :: defs :: ModEventComment :: deserialize (map_des) . map (Self :: ModEventComment) , "tools.ozone.moderation.defs#modEventEmail" => crate :: tools :: ozone :: moderation :: defs :: ModEventEmail :: deserialize (map_des) . map (Self :: ModEventEmail) , "tools.ozone.moderation.defs#modEventEscalate" => crate :: tools :: ozone :: moderation :: defs :: ModEventEscalate :: deserialize (map_des) . map (Self :: ModEventEscalate) , "tools.ozone.moderation.defs#modEventLabel" => crate :: tools :: ozone :: moderation :: defs :: ModEventLabel :: deserialize (map_des) . map (Self :: ModEventLabel) , "tools.ozone.moderation.defs#modEventMute" => crate :: tools :: ozone :: moderation :: defs :: ModEventMute :: deserialize (map_des) . map (Self :: ModEventMute) , "tools.ozone.moderation.defs#modEventMuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventMuteReporter :: deserialize (map_des) . map (Self :: ModEventMuteReporter) , "tools.ozone.moderation.defs#modEventReport" => crate :: tools :: ozone :: moderation :: defs :: ModEventReport :: deserialize (map_des) . map (Self :: ModEventReport) , "tools.ozone.moderation.defs#modEventResolveAppeal" => crate :: tools :: ozone :: moderation :: defs :: ModEventResolveAppeal :: deserialize (map_des) . map (Self :: ModEventResolveAppeal) , "tools.ozone.moderation.defs#modEventReverseTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventReverseTakedown :: deserialize (map_des) . map (Self :: ModEventReverseTakedown) , "tools.ozone.moderation.defs#modEventTag" => crate :: tools :: ozone :: moderation :: defs :: ModEventTag :: deserialize (map_des) . map (Self :: ModEventTag) , "tools.ozone.moderation.defs#modEventTakedown" => crate :: tools :: ozone :: moderation :: defs :: ModEventTakedown :: deserialize (map_des) . map (Self :: ModEventTakedown) , "tools.ozone.moderation.defs#modEventUnmute" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmute :: deserialize (map_des) . map (Self :: ModEventUnmute) , "tools.ozone.moderation.defs#modEventUnmuteReporter" => crate :: tools :: ozone :: moderation :: defs :: ModEventUnmuteReporter :: deserialize (map_des) . map (Self :: ModEventUnmuteReporter) , "tools.ozone.moderation.defs#recordEvent" => crate :: tools :: ozone :: moderation :: defs :: RecordEvent :: deserialize (map_des) . map (Self :: RecordEvent) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Subject {
                        RepoRef(crate::com::atproto::admin::defs::RepoRef),
                        StrongRef(crate::com::atproto::repo::StrongRef),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Subject {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_.ty.as_ref() {
                                    "com.atproto.admin.defs#repoRef" => {
                                        crate::com::atproto::admin::defs::RepoRef::deserialize(
                                            map_des,
                                        )
                                        .map(Self::RepoRef)
                                    }
                                    "com.atproto.repo.strongRef" => {
                                        crate::com::atproto::repo::StrongRef::deserialize(map_des)
                                            .map(Self::StrongRef)
                                    }
                                    _ => atmo_core::Unknown::deserialize(map_des).map(Self::Other),
                                };
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_event {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub id: i64,
                }
            }
            pub mod get_record {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RecordNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RecordNotFound => "RecordNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cid: std::option::Option<atmo_core::CidString>,
                    pub uri: atmo_core::AtUri,
                }
            }
            pub mod get_records {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub records: std::vec::Vec<
                        crate::tools::ozone::moderation::get_records::output::Records,
                    >,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub uris: std::vec::Vec<atmo_core::AtUri>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Records {
                        RecordViewDetail(crate::tools::ozone::moderation::defs::RecordViewDetail),
                        RecordViewNotFound(
                            crate::tools::ozone::moderation::defs::RecordViewNotFound,
                        ),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Records {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#recordViewDetail" => crate :: tools :: ozone :: moderation :: defs :: RecordViewDetail :: deserialize (map_des) . map (Self :: RecordViewDetail) , "tools.ozone.moderation.defs#recordViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RecordViewNotFound :: deserialize (map_des) . map (Self :: RecordViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#recordViewDetail" => crate :: tools :: ozone :: moderation :: defs :: RecordViewDetail :: deserialize (map_des) . map (Self :: RecordViewDetail) , "tools.ozone.moderation.defs#recordViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RecordViewNotFound :: deserialize (map_des) . map (Self :: RecordViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod get_repo {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    RepoNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::RepoNotFound => "RepoNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub did: atmo_core::Did,
                }
            }
            pub mod get_repos {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub repos:
                        std::vec::Vec<crate::tools::ozone::moderation::get_repos::output::Repos>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<atmo_core::Did>,
                }
                pub mod output {
                    #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize)]
                    pub enum Repos {
                        RepoViewDetail(crate::tools::ozone::moderation::defs::RepoViewDetail),
                        RepoViewNotFound(crate::tools::ozone::moderation::defs::RepoViewNotFound),
                        #[serde(untagged)]
                        Other(atmo_core::Unknown),
                    }
                    impl<'de> serde::Deserialize<'de> for Repos {
                        fn deserialize<D>(des: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            use serde::de::Error as _;
                            if des.is_human_readable() {
                                let visitor: atmo_core::union_::UnionVisitor<serde_json::Value> =
                                    Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| (k.as_ref(), v)),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#repoViewDetail" => crate :: tools :: ozone :: moderation :: defs :: RepoViewDetail :: deserialize (map_des) . map (Self :: RepoViewDetail) , "tools.ozone.moderation.defs#repoViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RepoViewNotFound :: deserialize (map_des) . map (Self :: RepoViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            } else {
                                let visitor: atmo_core::union_::UnionVisitor<
                                    ipld_core::ipld::Ipld,
                                > = Default::default();
                                let union_ = des.deserialize_map(visitor)?;
                                let map_des = serde::de::value::MapDeserializer::new(
                                    union_.map.iter().map(|(k, v)| {
                                        (
                                            k.as_ref(),
                                            atmo_core::union_::IpldIntoDeserializer(v.clone()),
                                        )
                                    }),
                                );
                                let res = match union_ . ty . as_ref () { "tools.ozone.moderation.defs#repoViewDetail" => crate :: tools :: ozone :: moderation :: defs :: RepoViewDetail :: deserialize (map_des) . map (Self :: RepoViewDetail) , "tools.ozone.moderation.defs#repoViewNotFound" => crate :: tools :: ozone :: moderation :: defs :: RepoViewNotFound :: deserialize (map_des) . map (Self :: RepoViewNotFound) , _ => atmo_core :: Unknown :: deserialize (map_des) . map (Self :: Other) , } ;
                                res.map_err(D::Error::custom)
                            }
                        }
                    }
                }
            }
            pub mod query_events {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub events: std::vec::Vec<crate::tools::ozone::moderation::defs::ModEventView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(rename = "addedLabels")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub added_labels: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "addedTags")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub added_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub collections: std::option::Option<std::vec::Vec<atmo_core::Nsid>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "createdAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "createdBefore")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "createdBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "hasComment")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub has_comment: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "includeAllUserRecords")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_all_user_records: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "removedLabels")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub removed_labels: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "removedTags")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub removed_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "reportTypes")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub report_types: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "sortDirection")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction: std::option::Option<
                        crate::tools::ozone::moderation::query_events::params::SortDirection,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "subjectType")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_type: std::option::Option<
                        crate::tools::ozone::moderation::query_events::params::SubjectType,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub types: std::option::Option<std::vec::Vec<std::string::String>>,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SortDirection {
                        #[serde(rename = "asc")]
                        Asc,
                        #[serde(rename = "desc")]
                        Desc,
                    }
                    impl SortDirection {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Asc => "asc",
                                Self::Desc => "desc",
                            }
                        }
                    }
                    impl std::fmt::Display for SortDirection {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        #[serde(rename = "account")]
                        Account,
                        #[serde(rename = "record")]
                        Record,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl SubjectType {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Account => "account",
                                Self::Record => "record",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for SubjectType {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod query_statuses {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(rename = "subjectStatuses")]
                    pub subject_statuses:
                        std::vec::Vec<crate::tools::ozone::moderation::defs::SubjectStatusView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appealed: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub collections: std::option::Option<std::vec::Vec<atmo_core::Nsid>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub comment: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "excludeTags")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub exclude_tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "hostingDeletedAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting_deleted_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "hostingDeletedBefore")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting_deleted_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "hostingStatuses")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting_statuses: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(rename = "hostingUpdatedAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting_updated_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "hostingUpdatedBefore")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub hosting_updated_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "ignoreSubjects")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub ignore_subjects: std::option::Option<std::vec::Vec<url::Url>>,
                    #[serde(default)]
                    #[serde(rename = "includeAllUserRecords")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_all_user_records: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "includeMuted")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub include_muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "lastReviewedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_reviewed_by: std::option::Option<atmo_core::Did>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "onlyMuted")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub only_muted: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "reportedAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reported_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "reportedBefore")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reported_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "reviewState")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub review_state: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "reviewedAfter")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reviewed_after: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "reviewedBefore")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub reviewed_before: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(default)]
                    #[serde(rename = "sortDirection")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction: std::option::Option<
                        crate::tools::ozone::moderation::query_statuses::params::SortDirection,
                    >,
                    #[serde(default)]
                    #[serde(rename = "sortField")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_field: std::option::Option<
                        crate::tools::ozone::moderation::query_statuses::params::SortField,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject: std::option::Option<url::Url>,
                    #[serde(default)]
                    #[serde(rename = "subjectType")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub subject_type: std::option::Option<
                        crate::tools::ozone::moderation::query_statuses::params::SubjectType,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub tags: std::option::Option<std::vec::Vec<std::string::String>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub takendown: std::option::Option<bool>,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SortDirection {
                        #[serde(rename = "asc")]
                        Asc,
                        #[serde(rename = "desc")]
                        Desc,
                    }
                    impl SortDirection {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Asc => "asc",
                                Self::Desc => "desc",
                            }
                        }
                    }
                    impl std::fmt::Display for SortDirection {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SortField {
                        #[serde(rename = "lastReviewedAt")]
                        LastReviewedAt,
                        #[serde(rename = "lastReportedAt")]
                        LastReportedAt,
                    }
                    impl SortField {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::LastReviewedAt => "lastReviewedAt",
                                Self::LastReportedAt => "lastReportedAt",
                            }
                        }
                    }
                    impl std::fmt::Display for SortField {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SubjectType {
                        #[serde(rename = "account")]
                        Account,
                        #[serde(rename = "record")]
                        Record,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl SubjectType {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Account => "account",
                                Self::Record => "record",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for SubjectType {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod search_repos {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub repos: std::vec::Vec<crate::tools::ozone::moderation::defs::RepoView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
            #[derive(Debug)]
            pub struct GetConfig;
            impl atmo_core::xrpc::Request for GetConfig {
                type Params = atmo_core::Nothing;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::server::get_config::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub appview:
                        std::option::Option<crate::tools::ozone::server::get_config::ServiceConfig>,
                    #[serde(default)]
                    #[serde(rename = "blobDivert")]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ServiceConfig {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub url: std::option::Option<url::Url>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct ViewerConfig {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub role: std::option::Option<
                        crate::tools::ozone::server::get_config::viewer_config::Role,
                    >,
                }
                pub mod viewer_config {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Role {
                        #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                        ToolsOzoneTeamDefsRoleAdmin,
                        #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                        ToolsOzoneTeamDefsRoleModerator,
                        #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                        ToolsOzoneTeamDefsRoleTriage,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Role {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::ToolsOzoneTeamDefsRoleAdmin => {
                                    "tools.ozone.team.defs#roleAdmin"
                                }
                                Self::ToolsOzoneTeamDefsRoleModerator => {
                                    "tools.ozone.team.defs#roleModerator"
                                }
                                Self::ToolsOzoneTeamDefsRoleTriage => {
                                    "tools.ozone.team.defs#roleTriage"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Role {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
        pub mod set {
            #[derive(Debug)]
            pub struct AddValues;
            impl atmo_core::xrpc::Request for AddValues {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::set::add_values::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteSet;
            impl atmo_core::xrpc::Request for DeleteSet {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::set::delete_set::Input;
                type Output = crate::tools::ozone::set::delete_set::Output;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteValues;
            impl atmo_core::xrpc::Request for DeleteValues {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::set::delete_values::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct GetValues;
            impl atmo_core::xrpc::Request for GetValues {
                type Params = crate::tools::ozone::set::get_values::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::set::get_values::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct QuerySets;
            impl atmo_core::xrpc::Request for QuerySets {
                type Params = crate::tools::ozone::set::query_sets::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::set::query_sets::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct UpsertSet;
            impl atmo_core::xrpc::Request for UpsertSet {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::set::defs::Set;
                type Output = crate::tools::ozone::set::defs::SetView;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    pub values: std::vec::Vec<std::string::String>,
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Set {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SetView {
                    #[serde(rename = "createdAt")]
                    pub created_at: atmo_core::DateTimeString,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub name: std::string::String,
                    #[serde(rename = "setSize")]
                    pub set_size: i64,
                    #[serde(rename = "updatedAt")]
                    pub updated_at: atmo_core::DateTimeString,
                }
            }
            pub mod delete_set {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    SetNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::SetNotFound => "SetNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
            }
            pub mod delete_values {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    SetNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::SetNotFound => "SetNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub name: std::string::String,
                    pub values: std::vec::Vec<std::string::String>,
                }
            }
            pub mod get_values {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    SetNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::SetNotFound => "SetNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub set: crate::tools::ozone::set::defs::SetView,
                    pub values: std::vec::Vec<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub sets: std::vec::Vec<crate::tools::ozone::set::defs::SetView>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(rename = "namePrefix")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub name_prefix: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(rename = "sortBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_by:
                        std::option::Option<crate::tools::ozone::set::query_sets::params::SortBy>,
                    #[serde(default)]
                    #[serde(rename = "sortDirection")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub sort_direction: std::option::Option<
                        crate::tools::ozone::set::query_sets::params::SortDirection,
                    >,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SortBy {
                        #[serde(rename = "name")]
                        Name,
                        #[serde(rename = "createdAt")]
                        CreatedAt,
                        #[serde(rename = "updatedAt")]
                        UpdatedAt,
                    }
                    impl SortBy {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Name => "name",
                                Self::CreatedAt => "createdAt",
                                Self::UpdatedAt => "updatedAt",
                            }
                        }
                    }
                    impl std::fmt::Display for SortBy {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum SortDirection {
                        #[serde(rename = "asc")]
                        Asc,
                        #[serde(rename = "desc")]
                        Desc,
                    }
                    impl SortDirection {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Asc => "asc",
                                Self::Desc => "desc",
                            }
                        }
                    }
                    impl std::fmt::Display for SortDirection {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
        pub mod setting {
            #[derive(Debug)]
            pub struct ListOptions;
            impl atmo_core::xrpc::Request for ListOptions {
                type Params = crate::tools::ozone::setting::list_options::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::setting::list_options::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.setting.listOptions"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            #[derive(Debug)]
            pub struct RemoveOptions;
            impl atmo_core::xrpc::Request for RemoveOptions {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::setting::remove_options::Input;
                type Output = crate::tools::ozone::setting::remove_options::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.setting.removeOptions"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            #[derive(Debug)]
            pub struct UpsertOption;
            impl atmo_core::xrpc::Request for UpsertOption {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::setting::upsert_option::Input;
                type Output = crate::tools::ozone::setting::upsert_option::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::POST
                }
                #[inline]
                fn nsid() -> &'static str {
                    "tools.ozone.setting.upsertOption"
                }
                #[inline]
                fn output_encoding() -> &'static str {
                    "application/json"
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Option {
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    #[serde(rename = "createdBy")]
                    pub created_by: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    pub key: atmo_core::Nsid,
                    #[serde(rename = "lastUpdatedBy")]
                    pub last_updated_by: atmo_core::Did,
                    #[serde(default)]
                    #[serde(rename = "managerRole")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub manager_role: std::option::Option<
                        crate::tools::ozone::setting::defs::option::ManagerRole,
                    >,
                    pub scope: crate::tools::ozone::setting::defs::option::Scope,
                    #[serde(default)]
                    #[serde(rename = "updatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo_core::DateTimeString>,
                    pub value: atmo_core::Unknown,
                }
                pub mod option {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ManagerRole {
                        #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                        ToolsOzoneTeamDefsRoleModerator,
                        #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                        ToolsOzoneTeamDefsRoleTriage,
                        #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                        ToolsOzoneTeamDefsRoleAdmin,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ManagerRole {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::ToolsOzoneTeamDefsRoleModerator => {
                                    "tools.ozone.team.defs#roleModerator"
                                }
                                Self::ToolsOzoneTeamDefsRoleTriage => {
                                    "tools.ozone.team.defs#roleTriage"
                                }
                                Self::ToolsOzoneTeamDefsRoleAdmin => {
                                    "tools.ozone.team.defs#roleAdmin"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ManagerRole {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Scope {
                        #[serde(rename = "instance")]
                        Instance,
                        #[serde(rename = "personal")]
                        Personal,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Scope {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Instance => "instance",
                                Self::Personal => "personal",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Scope {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod list_options {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub options: std::vec::Vec<crate::tools::ozone::setting::defs::Option>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub keys: std::option::Option<std::vec::Vec<atmo_core::Nsid>>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub prefix: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub scope: std::option::Option<
                        crate::tools::ozone::setting::list_options::params::Scope,
                    >,
                }
                pub mod params {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Scope {
                        #[serde(rename = "instance")]
                        Instance,
                        #[serde(rename = "personal")]
                        Personal,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Scope {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Instance => "instance",
                                Self::Personal => "personal",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Scope {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod remove_options {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub keys: std::vec::Vec<atmo_core::Nsid>,
                    pub scope: crate::tools::ozone::setting::remove_options::input::Scope,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {}
                pub mod input {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Scope {
                        #[serde(rename = "instance")]
                        Instance,
                        #[serde(rename = "personal")]
                        Personal,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Scope {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Instance => "instance",
                                Self::Personal => "personal",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Scope {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod upsert_option {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub description: std::option::Option<std::string::String>,
                    pub key: atmo_core::Nsid,
                    #[serde(default)]
                    #[serde(rename = "managerRole")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub manager_role: std::option::Option<
                        crate::tools::ozone::setting::upsert_option::input::ManagerRole,
                    >,
                    pub scope: crate::tools::ozone::setting::upsert_option::input::Scope,
                    pub value: atmo_core::Unknown,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub option: crate::tools::ozone::setting::defs::Option,
                }
                pub mod input {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum ManagerRole {
                        #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                        ToolsOzoneTeamDefsRoleModerator,
                        #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                        ToolsOzoneTeamDefsRoleTriage,
                        #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                        ToolsOzoneTeamDefsRoleAdmin,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl ManagerRole {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::ToolsOzoneTeamDefsRoleModerator => {
                                    "tools.ozone.team.defs#roleModerator"
                                }
                                Self::ToolsOzoneTeamDefsRoleTriage => {
                                    "tools.ozone.team.defs#roleTriage"
                                }
                                Self::ToolsOzoneTeamDefsRoleAdmin => {
                                    "tools.ozone.team.defs#roleAdmin"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for ManagerRole {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Scope {
                        #[serde(rename = "instance")]
                        Instance,
                        #[serde(rename = "personal")]
                        Personal,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Scope {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::Instance => "instance",
                                Self::Personal => "personal",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Scope {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
        pub mod signature {
            #[derive(Debug)]
            pub struct FindCorrelation;
            impl atmo_core::xrpc::Request for FindCorrelation {
                type Params = crate::tools::ozone::signature::find_correlation::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::signature::find_correlation::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct FindRelatedAccounts;
            impl atmo_core::xrpc::Request for FindRelatedAccounts {
                type Params = crate::tools::ozone::signature::find_related_accounts::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::signature::find_related_accounts::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct SearchAccounts;
            impl atmo_core::xrpc::Request for SearchAccounts {
                type Params = crate::tools::ozone::signature::search_accounts::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::signature::search_accounts::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct SigDetail {
                    pub property: std::string::String,
                    pub value: std::string::String,
                }
            }
            pub mod find_correlation {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub details: std::vec::Vec<crate::tools::ozone::signature::defs::SigDetail>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    pub dids: std::vec::Vec<atmo_core::Did>,
                }
            }
            pub mod find_related_accounts {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<
                        crate::tools::ozone::signature::find_related_accounts::RelatedAccount,
                    >,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Params {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub limit: std::option::Option<i64>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    pub accounts: std::vec::Vec<crate::com::atproto::admin::defs::AccountView>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
            #[derive(Debug)]
            pub struct AddMember;
            impl atmo_core::xrpc::Request for AddMember {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::team::add_member::Input;
                type Output = crate::tools::ozone::team::defs::Member;
                type Error = String;
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
            #[derive(Debug)]
            pub struct DeleteMember;
            impl atmo_core::xrpc::Request for DeleteMember {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::team::delete_member::Input;
                type Output = atmo_core::Nothing;
                type Error = String;
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
            #[derive(Debug)]
            pub struct ListMembers;
            impl atmo_core::xrpc::Request for ListMembers {
                type Params = crate::tools::ozone::team::list_members::Params;
                type Input = atmo_core::Nothing;
                type Output = crate::tools::ozone::team::list_members::Output;
                type Error = String;
                #[inline]
                fn method() -> http::Method {
                    http::Method::GET
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
            #[derive(Debug)]
            pub struct UpdateMember;
            impl atmo_core::xrpc::Request for UpdateMember {
                type Params = atmo_core::Nothing;
                type Input = crate::tools::ozone::team::update_member::Input;
                type Output = crate::tools::ozone::team::defs::Member;
                type Error = String;
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    MemberAlreadyExists,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::MemberAlreadyExists => "MemberAlreadyExists",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    pub role: crate::tools::ozone::team::add_member::input::Role,
                }
                pub mod input {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Role {
                        #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                        ToolsOzoneTeamDefsRoleAdmin,
                        #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                        ToolsOzoneTeamDefsRoleModerator,
                        #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                        ToolsOzoneTeamDefsRoleTriage,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Role {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::ToolsOzoneTeamDefsRoleAdmin => {
                                    "tools.ozone.team.defs#roleAdmin"
                                }
                                Self::ToolsOzoneTeamDefsRoleModerator => {
                                    "tools.ozone.team.defs#roleModerator"
                                }
                                Self::ToolsOzoneTeamDefsRoleTriage => {
                                    "tools.ozone.team.defs#roleTriage"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Role {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod defs {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Member {
                    #[serde(default)]
                    #[serde(rename = "createdAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub created_at: std::option::Option<atmo_core::DateTimeString>,
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(rename = "lastUpdatedBy")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub last_updated_by: std::option::Option<std::string::String>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub profile:
                        std::option::Option<crate::app::bsky::actor::defs::ProfileViewDetailed>,
                    pub role: crate::tools::ozone::team::defs::member::Role,
                    #[serde(default)]
                    #[serde(rename = "updatedAt")]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub updated_at: std::option::Option<atmo_core::DateTimeString>,
                }
                pub mod member {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Role {
                        #[serde(rename = "#roleAdmin")]
                        RoleAdmin,
                        #[serde(rename = "#roleModerator")]
                        RoleModerator,
                        #[serde(rename = "#roleTriage")]
                        RoleTriage,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Role {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::RoleAdmin => "#roleAdmin",
                                Self::RoleModerator => "#roleModerator",
                                Self::RoleTriage => "#roleTriage",
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Role {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
            pub mod delete_member {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    MemberNotFound,
                    CannotDeleteSelf,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::MemberNotFound => "MemberNotFound",
                            Self::CannotDeleteSelf => "CannotDeleteSelf",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                }
            }
            pub mod list_members {
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Output {
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub cursor: std::option::Option<std::string::String>,
                    pub members: std::vec::Vec<crate::tools::ozone::team::defs::Member>,
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
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
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize)]
                pub enum Error {
                    MemberNotFound,
                    #[serde(untagged)]
                    Other(String),
                }
                impl Error {
                    pub fn as_str(&self) -> &str {
                        match self {
                            Self::MemberNotFound => "MemberNotFound",
                            Self::Other(s) => s.as_str(),
                        }
                    }
                }
                impl std::fmt::Display for Error {
                    #[inline]
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                        f.write_str(self.as_str())
                    }
                }
                #[derive(Clone, Debug, PartialEq, Eq, serde :: Deserialize, serde :: Serialize)]
                pub struct Input {
                    pub did: atmo_core::Did,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub disabled: std::option::Option<bool>,
                    #[serde(default)]
                    #[serde(skip_serializing_if = "std::option::Option::is_none")]
                    pub role:
                        std::option::Option<crate::tools::ozone::team::update_member::input::Role>,
                }
                pub mod input {
                    #[derive(
                        Clone, Debug, PartialEq, Eq, serde :: Serialize, serde :: Deserialize,
                    )]
                    pub enum Role {
                        #[serde(rename = "tools.ozone.team.defs#roleAdmin")]
                        ToolsOzoneTeamDefsRoleAdmin,
                        #[serde(rename = "tools.ozone.team.defs#roleModerator")]
                        ToolsOzoneTeamDefsRoleModerator,
                        #[serde(rename = "tools.ozone.team.defs#roleTriage")]
                        ToolsOzoneTeamDefsRoleTriage,
                        #[serde(untagged)]
                        Other(String),
                    }
                    impl Role {
                        pub fn as_str(&self) -> &str {
                            match self {
                                Self::ToolsOzoneTeamDefsRoleAdmin => {
                                    "tools.ozone.team.defs#roleAdmin"
                                }
                                Self::ToolsOzoneTeamDefsRoleModerator => {
                                    "tools.ozone.team.defs#roleModerator"
                                }
                                Self::ToolsOzoneTeamDefsRoleTriage => {
                                    "tools.ozone.team.defs#roleTriage"
                                }
                                Self::Other(s) => s.as_str(),
                            }
                        }
                    }
                    impl std::fmt::Display for Role {
                        #[inline]
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                            f.write_str(self.as_str())
                        }
                    }
                }
            }
        }
    }
}
