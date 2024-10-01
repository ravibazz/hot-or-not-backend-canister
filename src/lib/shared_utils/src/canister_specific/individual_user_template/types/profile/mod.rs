use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use super::migration::MigrationInfo;

#[derive(Default, Clone, CandidType, Deserialize, Debug, Serialize)]
pub struct UserProfile {
    pub display_name: Option<String>,
    pub unique_user_name: Option<String>,
    pub principal_id: Option<Principal>,
    pub profile_picture_url: Option<String>,
    pub profile_stats: UserProfileGlobalStats,
    #[serde(default)]
    pub referrer_details: Option<UserCanisterDetails>,
}

#[derive(Clone, CandidType, Deserialize, Debug, Serialize, PartialEq, Eq)]
pub struct UserCanisterDetails {
    pub profile_owner: Principal,
    pub user_canister_id: Principal,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct UserProfileDetailsForFrontend {
    pub display_name: Option<String>,
    pub followers_count: u64,
    pub following_count: u64,
    pub principal_id: Principal,
    pub profile_picture_url: Option<String>,
    pub profile_stats: UserProfileGlobalStats,
    pub lifetime_earnings: u64,
    pub unique_user_name: Option<String>,
    pub referrer_details: Option<UserCanisterDetails>
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]
pub struct UserProfileDetailsForFrontendV2 {
    pub display_name: Option<String>,
    pub followers_count: u64,
    pub following_count: u64,
    pub principal_id: Principal,
    pub profile_picture_url: Option<String>,
    pub profile_stats: UserProfileGlobalStats,
    pub lifetime_earnings: u64,
    pub unique_user_name: Option<String>,
    pub referrer_details: Option<UserCanisterDetails>,
    pub migration_info: MigrationInfo
}

#[derive(CandidType, Deserialize, Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
pub struct UserProfileGlobalStats {
    pub hot_bets_received: u64,
    pub not_bets_received: u64,
}

#[derive(Deserialize, CandidType)]
pub struct UserProfileUpdateDetailsFromFrontend {
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
}
