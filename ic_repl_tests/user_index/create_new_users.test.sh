

import user_index_canister = "be2us-64aaa-aaaaa-qaabq-cai";


identity alice;

let my_canister = call user_index_canister.get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(null);

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});




identity bob;

let my_canister = call user_index_canister.get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(null);

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});




identity charlie;
let my_canister = call user_index_canister.get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(null);

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});

call my_canister.add_post_v2(record { hashtags = vec { "a"; "b"; "c" }; description = "This is post from integration test"; video_uid = "#1234567890"; creator_consent_for_inclusion_in_hot_or_not = false; is_nsfw = false});

