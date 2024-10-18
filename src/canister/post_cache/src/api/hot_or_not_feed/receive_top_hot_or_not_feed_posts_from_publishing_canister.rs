use ic_cdk_macros::update;
use shared_utils::common::types::top_posts::post_score_index_item::{
    PostScoreIndexItem, PostScoreIndexItemV1,
};

use crate::{
    api::feed::trigger_update_indexes::{
        self, trigger_reconcile_scores, trigger_update_hot_or_not_index,
    },
    data_model::CanisterData,
    CANISTER_DATA,
};

#[update]
fn receive_top_hot_or_not_feed_posts_from_publishing_canister(
    top_posts_from_publishing_canister: Vec<PostScoreIndexItemV1>,
) {
    let caller = ic_cdk::caller();
    for post_score_index_item in top_posts_from_publishing_canister.clone() {
        if post_score_index_item.publisher_canister_id != caller {
            return;
        }
    }

    CANISTER_DATA.with(|canister_data| {
        let mut canister_data = canister_data.borrow_mut();

        receive_top_hot_or_not_feed_posts_from_publishing_canister_impl(
            top_posts_from_publishing_canister,
            &mut canister_data,
        );
    });

    trigger_update_hot_or_not_index();
    trigger_reconcile_scores();
}

fn receive_top_hot_or_not_feed_posts_from_publishing_canister_impl(
    top_posts_from_publishing_canister: Vec<PostScoreIndexItemV1>,
    canister_data: &mut CanisterData,
) {
    let posts_index_sorted_by_hot_or_not_feed_score =
        &mut canister_data.posts_index_sorted_by_hot_or_not_feed_score_v1;

    for post_score_index_item in top_posts_from_publishing_canister.clone() {
        posts_index_sorted_by_hot_or_not_feed_score.replace(&post_score_index_item);
    }

    if posts_index_sorted_by_hot_or_not_feed_score.iter().count() > 1500 {
        *posts_index_sorted_by_hot_or_not_feed_score = posts_index_sorted_by_hot_or_not_feed_score
            .into_iter()
            .take(1000)
            .cloned()
            .collect();
    }
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use candid::Principal;
    use shared_utils::common::types::top_posts::post_score_index_item::PostStatus;

    use super::*;

    #[test]
    fn test_receive_top_hot_or_not_feed_posts_from_publishing_canister_impl() {
        let mut canister_data = CanisterData::default();
        let created_at_now = SystemTime::now();

        let top_posts_from_publishing_canister = vec![
            PostScoreIndexItemV1 {
                post_id: 1,
                score: 1,
                publisher_canister_id: Principal::from_text("aaaaa-aa").unwrap(),
                is_nsfw: false,
                status: PostStatus::ReadyToView,
                created_at: Some(created_at_now),
            },
            PostScoreIndexItemV1 {
                post_id: 2,
                score: 2,
                publisher_canister_id: Principal::from_text("aaaaa-aa").unwrap(),
                is_nsfw: false,
                status: PostStatus::ReadyToView,
                created_at: Some(created_at_now),
            },
            PostScoreIndexItemV1 {
                post_id: 3,
                score: 3,
                publisher_canister_id: Principal::from_text("aaaaa-aa").unwrap(),
                is_nsfw: false,
                status: PostStatus::ReadyToView,
                created_at: Some(created_at_now),
            },
        ];

        receive_top_hot_or_not_feed_posts_from_publishing_canister_impl(
            top_posts_from_publishing_canister,
            &mut canister_data,
        );

        let posts_index_sorted_by_hot_or_not_feed_score =
            &canister_data.posts_index_sorted_by_hot_or_not_feed_score_v1;

        assert_eq!(
            posts_index_sorted_by_hot_or_not_feed_score.iter().count(),
            3
        );
    }
}
