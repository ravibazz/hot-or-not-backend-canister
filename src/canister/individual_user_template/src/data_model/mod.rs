use std::collections::{BTreeMap, BTreeSet};

use candid::{Deserialize, Principal};
use ic_cdk::api::management_canister::provisional::CanisterId;
use serde::Serialize;
use shared_utils::{
    canister_specific::individual_user_template::types::{
        configuration::IndividualUserConfiguration, follow::FollowData,
        hot_or_not::PlacedBetDetail, post::Post, profile::UserProfile, token::TokenBalance,
    },
    common::types::{
        app_primitive_type::PostId, known_principal::KnownPrincipalMap,
        top_posts::post_score_index::PostScoreIndex,
    },
};

use self::version_details::VersionDetails;

pub mod version_details;
pub mod memory;

#[derive(Default, Deserialize, Serialize)]
pub struct CanisterData {
    // Key is Post ID
    pub all_created_posts: BTreeMap<u64, Post>,
    pub all_hot_or_not_bets_placed: BTreeMap<(CanisterId, PostId), PlacedBetDetail>,
    pub configuration: IndividualUserConfiguration,
    pub follow_data: FollowData,
    pub known_principal_ids: KnownPrincipalMap,
    pub my_token_balance: TokenBalance,
    pub posts_index_sorted_by_home_feed_score: PostScoreIndex,
    pub posts_index_sorted_by_hot_or_not_feed_score: PostScoreIndex,
    pub principals_i_follow: BTreeSet<Principal>,
    pub principals_that_follow_me: BTreeSet<Principal>,
    pub profile: UserProfile,
    pub version_details: VersionDetails,
}
