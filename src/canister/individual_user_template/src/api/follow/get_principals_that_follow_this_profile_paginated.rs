use std::ops::Bound::Included;

use crate::{
    api::canister_management::update_last_access_time::update_last_canister_functionality_access_time,
    data_model::CanisterData, CANISTER_DATA,
};

use ic_cdk_macros::query;
use shared_utils::canister_specific::individual_user_template::types::follow::{
    FollowEntryDetail, FollowEntryId,
};

pub const MAX_FOLLOW_ENTRIES_PER_PAGE: usize = 10;

#[query]
pub fn get_principals_that_follow_this_profile_paginated(
    last_index_received: Option<u64>,
) -> Vec<(FollowEntryId, FollowEntryDetail)> {
    update_last_canister_functionality_access_time();
    CANISTER_DATA.with(|canister_data_ref_cell| {
        let canister_data = canister_data_ref_cell.borrow();
        get_principals_that_follow_this_profile_paginated_impl(&canister_data, last_index_received)
    })
}

fn get_principals_that_follow_this_profile_paginated_impl(
    canister_data: &CanisterData,
    last_index_received: Option<u64>,
) -> Vec<(FollowEntryId, FollowEntryDetail)> {
    let follower = &canister_data.follow_data.follower;
    let last_key: u64 = follower
        .sorted_index
        .last_key_value()
        .map_or(0, |(k, _)| *k);

    follower
        .sorted_index
        .range((
            Included(0),
            Included(last_index_received.unwrap_or(last_key)),
        ))
        .rev()
        .take(MAX_FOLLOW_ENTRIES_PER_PAGE)
        .map(|(id, entry)| (*id, entry.clone()))
        .collect::<Vec<(u64, FollowEntryDetail)>>()
}

#[cfg(test)]
mod test {
    use candid::Principal;
    use test_utils::setup::test_constants::get_mock_user_alice_principal_id;

    use super::*;

    #[test]
    fn test_get_principals_that_follow_this_profile_paginated_impl() {
        let mut canister_data = CanisterData::default();
        let mut last_index_received: Option<u64> = None;

        canister_data.profile.principal_id = Some(get_mock_user_alice_principal_id());

        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 0);

        (0..25).for_each(|id: u64| {
            let follow_entry_detail = FollowEntryDetail {
                principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                canister_id: Principal::self_authenticating(id.to_ne_bytes()),
            };
            canister_data.follow_data.follower.add(follow_entry_detail);
        });

        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            (15..=24)
                .rev()
                .map(|id: u64| (
                    id,
                    FollowEntryDetail {
                        principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                        canister_id: Principal::self_authenticating(id.to_ne_bytes()),
                    }
                ))
                .collect::<Vec<(u64, FollowEntryDetail)>>()
        );

        last_index_received = Some(15);
        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            (6..=15)
                .rev()
                .map(|id: u64| (
                    id,
                    FollowEntryDetail {
                        principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                        canister_id: Principal::self_authenticating(id.to_ne_bytes()),
                    }
                ))
                .collect::<Vec<(u64, FollowEntryDetail)>>()
        );

        last_index_received = Some(5);

        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 6);
        assert_eq!(
            result,
            (0..=5)
                .rev()
                .map(|id: u64| (
                    id,
                    FollowEntryDetail {
                        principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                        canister_id: Principal::self_authenticating(id.to_ne_bytes()),
                    }
                ))
                .collect::<Vec<(u64, FollowEntryDetail)>>()
        );

        last_index_received = Some(0);

        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(
            result,
            (0..1)
                .rev()
                .map(|id: u64| (
                    id,
                    FollowEntryDetail {
                        principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                        canister_id: Principal::self_authenticating(id.to_ne_bytes()),
                    }
                ))
                .collect::<Vec<(u64, FollowEntryDetail)>>()
        );

        last_index_received = Some(100);

        let result = get_principals_that_follow_this_profile_paginated_impl(
            &canister_data,
            last_index_received,
        );

        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            (15..=24)
                .rev()
                .map(|id: u64| (
                    id,
                    FollowEntryDetail {
                        principal_id: Principal::self_authenticating(id.to_ne_bytes()),
                        canister_id: Principal::self_authenticating(id.to_ne_bytes()),
                    }
                ))
                .collect::<Vec<(u64, FollowEntryDetail)>>()
        );
    }
}
