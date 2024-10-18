use candid::{CandidType, Principal};
use ic_ledger_types::{BlockIndex, Tokens};
use pocket_ic::WasmResult as PocketICWasmResult;
use shared_utils::{
    canister_specific::individual_user_template::types::{
        error::GetPostsOfUserProfileError,
        migration::MigrationErrors,
        post::{Post, PostDetailsForFrontend, PostDetailsFromFrontend},
        session::SessionType,
    },
    common::types::known_principal::KnownPrincipalType,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::Duration,
};
use test_utils::setup::{
    env::pocket_ic_env::get_new_pocket_ic_env,
    test_constants::{
        get_mock_user_alice_principal_id, get_mock_user_bob_principal_id,
        get_mock_user_charlie_principal_id, get_mock_user_dan_principal_id,
    },
};

#[test]
#[ignore]
fn test_transfer_token_can_heppen_only_once_from_hot_or_not_canister_to_yral_canister_triggered_by_profile_owner(
) {
    let (pocket_ic, known_principal) = get_new_pocket_ic_env();
    let platform_canister_id = known_principal
        .get(&KnownPrincipalType::CanisterIdPlatformOrchestrator)
        .cloned()
        .unwrap();

    let super_admin = known_principal
        .get(&KnownPrincipalType::UserIdGlobalSuperAdmin)
        .cloned()
        .unwrap();

    let application_subnets = pocket_ic.topology().get_app_subnets();

    let hot_or_not_subnet_orchestrator_canister_id: Principal = pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "provision_subnet_orchestrator_canister",
            candid::encode_one(application_subnets[0]).unwrap(),
        )
        .map(|res| {
            let canister_id_result: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id_result.unwrap()
        })
        .unwrap();

    let yral_subnet_orchestrator_canister_id: Principal = pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "provision_subnet_orchestrator_canister",
            candid::encode_one(application_subnets[1]).unwrap(),
        )
        .map(|res| {
            let canister_id_result: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id_result.unwrap()
        })
        .unwrap();

    for _ in 0..30 {
        pocket_ic.tick();
    }

    let post_cache_canister_id = Principal::anonymous();

    pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "update_global_known_principal",
            candid::encode_args((
                KnownPrincipalType::CanisterIdHotOrNotSubnetOrchestrator,
                hot_or_not_subnet_orchestrator_canister_id,
            ))
            .unwrap(),
        )
        .unwrap();

    for _ in 0..30 {
        pocket_ic.tick();
    }

    //Alice hot or not and yral canister
    let alice_hot_or_not_principal_id = get_mock_user_alice_principal_id();
    let alice_hot_or_not_canister_id: Principal = pocket_ic
        .update_call(
            hot_or_not_subnet_orchestrator_canister_id,
            alice_hot_or_not_principal_id,
            "get_requester_principals_canister_id_create_if_not_exists",
            candid::encode_one(()).unwrap(),
        )
        .map(|res| {
            let canister_id: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id
        })
        .unwrap()
        .unwrap();

    for _ in 0..52 {
        let _alice_post_id = pocket_ic
            .update_call(
                alice_hot_or_not_canister_id,
                alice_hot_or_not_principal_id,
                "add_post_v2",
                candid::encode_args((PostDetailsFromFrontend {
                    is_nsfw: false,
                    description: "This is a fun video to watch".to_string(),
                    hashtags: vec!["fun".to_string(), "video".to_string()],
                    video_uid: "abcd#1234".to_string(),
                    creator_consent_for_inclusion_in_hot_or_not: true,
                },))
                .unwrap(),
            )
            .map(|reply_payload| {
                let newly_created_post_id_result: Result<u64, String> = match reply_payload {
                    PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                    _ => panic!("\n🛑 add_post failed\n"),
                };
                newly_created_post_id_result.unwrap()
            })
            .unwrap();
    }

    pocket_ic
        .update_call(
            alice_hot_or_not_canister_id,
            hot_or_not_subnet_orchestrator_canister_id,
            "update_session_type",
            candid::encode_one(SessionType::RegisteredSession).unwrap(),
        )
        .map(|reply_payload| {
            let res: Result<String, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            res
        })
        .unwrap()
        .unwrap();

    let alice_yral_principal_id = get_mock_user_bob_principal_id();
    let alice_yral_canister_id: Principal = pocket_ic
        .update_call(
            yral_subnet_orchestrator_canister_id,
            alice_yral_principal_id,
            "get_requester_principals_canister_id_create_if_not_exists",
            candid::encode_one(()).unwrap(),
        )
        .map(|res| {
            let canister_id: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id
        })
        .unwrap()
        .unwrap();

    pocket_ic
        .update_call(
            alice_yral_canister_id,
            yral_subnet_orchestrator_canister_id,
            "update_session_type",
            candid::encode_one(SessionType::RegisteredSession).unwrap(),
        )
        .map(|reply_payload| {
            let res: Result<String, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            res
        })
        .unwrap()
        .unwrap();

    pocket_ic
        .update_call(
            alice_yral_canister_id,
            alice_yral_principal_id,
            "add_post_v2",
            candid::encode_args((PostDetailsFromFrontend {
                is_nsfw: false,
                description: "This is a yral fun video to watch".to_string(),
                hashtags: vec!["fun".to_string(), "video".to_string()],
                video_uid: "abcd#1234".to_string(),
                creator_consent_for_inclusion_in_hot_or_not: true,
            },))
            .unwrap(),
        )
        .map(|reply_payload| {
            let newly_created_post_id_result: Result<u64, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            newly_created_post_id_result.unwrap()
        })
        .unwrap();

    //charlie hot or not and yral canister
    let charlie_hot_or_not_principal_id = get_mock_user_charlie_principal_id();
    let charlie_hot_or_not_canister_id: Principal = pocket_ic
        .update_call(
            hot_or_not_subnet_orchestrator_canister_id,
            charlie_hot_or_not_principal_id,
            "get_requester_principals_canister_id_create_if_not_exists",
            candid::encode_one(()).unwrap(),
        )
        .map(|res| {
            let canister_id: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id
        })
        .unwrap()
        .unwrap();

    let charlie_yral_principal_id = get_mock_user_dan_principal_id();
    let charlie_yral_canister_id = pocket_ic
        .update_call(
            yral_subnet_orchestrator_canister_id,
            charlie_yral_principal_id,
            "get_requester_principals_canister_id_create_if_not_exists",
            candid::encode_one(()).unwrap(),
        )
        .map(|res| {
            let canister_id: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id
        })
        .unwrap()
        .unwrap();

    //update subnet known principal
    pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "update_subnet_known_principal",
            candid::encode_args((
                hot_or_not_subnet_orchestrator_canister_id,
                KnownPrincipalType::CanisterIdPostCache,
                post_cache_canister_id,
            ))
            .unwrap(),
        )
        .map(|res| {
            let update_res: Result<String, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("update subnet_known_principal"),
            };
            update_res
        })
        .unwrap()
        .unwrap();

    for _ in 0..30 {
        pocket_ic.tick()
    }

    //charlie tries to transfer alice canister
    let success = pocket_ic
        .update_call(
            alice_hot_or_not_canister_id,
            charlie_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((charlie_yral_principal_id, charlie_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();

    assert_eq!(success, Err(MigrationErrors::Unauthorized));

    //Transfer token from yral to yral account
    let success = pocket_ic
        .update_call(
            alice_yral_canister_id,
            alice_yral_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((charlie_yral_principal_id, charlie_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();

    assert_eq!(success, Err(MigrationErrors::InvalidFromCanister));

    //Transfer token from hotornot to hotornot account
    let success = pocket_ic
        .update_call(
            alice_hot_or_not_canister_id,
            alice_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((
                charlie_hot_or_not_principal_id,
                charlie_hot_or_not_canister_id,
            ))
            .unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();

    assert_eq!(success, Err(MigrationErrors::InvalidToCanister));

    // transfer token
    let success = pocket_ic
        .update_call(
            alice_hot_or_not_canister_id,
            alice_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((alice_yral_principal_id, alice_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();

    assert_eq!(success, Ok(()));

    pocket_ic.advance_time(Duration::from_secs(1000));

    for _ in 0..25 {
        pocket_ic.tick();
    }

    let alice_yral_token_balance = pocket_ic
        .query_call(
            alice_yral_canister_id,
            Principal::anonymous(),
            "get_utility_token_balance",
            candid::encode_one(()).unwrap(),
        )
        .map(|reply_payload| {
            let balance: u64 = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            balance
        })
        .unwrap();

    let alice_hot_or_not_utility_balance = pocket_ic
        .query_call(
            alice_hot_or_not_canister_id,
            Principal::anonymous(),
            "get_utility_token_balance",
            candid::encode_one(()).unwrap(),
        )
        .map(|reply_payload| {
            let balance: u64 = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            balance
        })
        .unwrap();

    assert_eq!(alice_yral_token_balance, 2000);
    assert_eq!(alice_hot_or_not_utility_balance, 0);

    let posts_response = pocket_ic
        .query_call(
            alice_yral_canister_id,
            Principal::anonymous(),
            "get_posts_of_this_user_profile_with_pagination_cursor",
            candid::encode_args((0_u64, 100_u64)).unwrap(),
        )
        .map(|reply_payload| {
            let posts_response: Result<Vec<PostDetailsForFrontend>, GetPostsOfUserProfileError> =
                match reply_payload {
                    PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                    _ => panic!("\n🛑 get_posts_of_this_user_profile_with_pagination failed\n"),
                };
            posts_response
        })
        .unwrap()
        .unwrap();

    assert_eq!(posts_response.len(), 53);

    //mark charile hot or not as registered
    pocket_ic
        .update_call(
            charlie_hot_or_not_canister_id,
            hot_or_not_subnet_orchestrator_canister_id,
            "update_session_type",
            candid::encode_one(SessionType::RegisteredSession).unwrap(),
        )
        .map(|reply_payload| {
            let res: Result<String, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            res
        })
        .unwrap()
        .unwrap();

    // attempt to transfer token from new hot or not account to already used for migration yral account
    let success = pocket_ic
        .update_call(
            charlie_hot_or_not_canister_id,
            charlie_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((alice_yral_principal_id, alice_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();
    assert_eq!(success, Err(MigrationErrors::AlreadyUsedForMigration));

    // attempt to transfer token from already migrated hot or not account to new yral account.
    let success = pocket_ic
        .update_call(
            alice_hot_or_not_canister_id,
            alice_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((charlie_yral_principal_id, charlie_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();
    assert_eq!(success, Err(MigrationErrors::AlreadyMigrated));

    //attempt to transfer token from hot or not account to not registered yral account
    let success = pocket_ic
        .update_call(
            charlie_hot_or_not_canister_id,
            charlie_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((charlie_yral_principal_id, charlie_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();
    assert_eq!(success, Err(MigrationErrors::UserNotRegistered));

    //mark charlie yral account as registered
    pocket_ic
        .update_call(
            charlie_yral_canister_id,
            yral_subnet_orchestrator_canister_id,
            "update_session_type",
            candid::encode_one(SessionType::RegisteredSession).unwrap(),
        )
        .map(|reply_payload| {
            let res: Result<String, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            res
        })
        .unwrap()
        .unwrap();

    //attempt to transfer token from new hot or not account to new yral account
    let success = pocket_ic
        .update_call(
            charlie_hot_or_not_canister_id,
            charlie_hot_or_not_principal_id,
            "transfer_tokens_and_posts",
            candid::encode_args((charlie_yral_principal_id, charlie_yral_canister_id)).unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();
    assert_eq!(success, Ok(()));
}

#[test]
fn test_when_user_tries_to_misuse_to_recieve_tokens_and_posts() {
    let (pocket_ic, known_principal) = get_new_pocket_ic_env();
    let platform_canister_id = known_principal
        .get(&KnownPrincipalType::CanisterIdPlatformOrchestrator)
        .cloned()
        .unwrap();

    let super_admin = known_principal
        .get(&KnownPrincipalType::UserIdGlobalSuperAdmin)
        .cloned()
        .unwrap();

    let application_subnets = pocket_ic.topology().get_app_subnets();

    let hot_or_not_subnet_orchestrator_canister_id: Principal = pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "provision_subnet_orchestrator_canister",
            candid::encode_one(application_subnets[0]).unwrap(),
        )
        .map(|res| {
            let canister_id_result: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id_result.unwrap()
        })
        .unwrap();

    let yral_subnet_orchestrator_canister_id: Principal = pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "provision_subnet_orchestrator_canister",
            candid::encode_one(application_subnets[1]).unwrap(),
        )
        .map(|res| {
            let canister_id_result: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id_result.unwrap()
        })
        .unwrap();

    for _ in 0..30 {
        pocket_ic.tick();
    }

    let post_cache_canister_id = Principal::anonymous();

    pocket_ic
        .update_call(
            platform_canister_id,
            super_admin,
            "update_global_known_principal",
            candid::encode_args((
                KnownPrincipalType::CanisterIdHotOrNotSubnetOrchestrator,
                hot_or_not_subnet_orchestrator_canister_id,
            ))
            .unwrap(),
        )
        .unwrap();

    for _ in 0..10 {
        pocket_ic.tick();
    }

    let alice_yral_principal_id = get_mock_user_alice_principal_id();
    let alice_yral_canister_id = pocket_ic
        .update_call(
            yral_subnet_orchestrator_canister_id,
            alice_yral_principal_id,
            "get_requester_principals_canister_id_create_if_not_exists",
            candid::encode_one(()).unwrap(),
        )
        .map(|res| {
            let canister_id: Result<Principal, String> = match res {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("Canister call failed"),
            };
            canister_id
        })
        .unwrap()
        .unwrap();

    //mark alice yral canister as registered
    pocket_ic
        .update_call(
            alice_yral_canister_id,
            yral_subnet_orchestrator_canister_id,
            "update_session_type",
            candid::encode_one(SessionType::RegisteredSession).unwrap(),
        )
        .map(|reply_payload| {
            let res: Result<String, String> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 add_post failed\n"),
            };
            res
        })
        .unwrap()
        .unwrap();

    let alice_dummy_canister = pocket_ic.create_canister();
    //Transfer token from yral to yral account
    let res = pocket_ic
        .update_call(
            alice_yral_canister_id,
            alice_dummy_canister,
            "receive_data_from_hotornot",
            candid::encode_args((
                alice_yral_principal_id,
                10000_u64,
                BTreeMap::<u64, Post>::new(),
            ))
            .unwrap(),
        )
        .map(|reply_payload| {
            let success: Result<(), MigrationErrors> = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            success
        })
        .unwrap();

    let alice_yral_token_balance = pocket_ic
        .query_call(
            alice_yral_canister_id,
            Principal::anonymous(),
            "get_utility_token_balance",
            candid::encode_one(()).unwrap(),
        )
        .map(|reply_payload| {
            let balance: u64 = match reply_payload {
                PocketICWasmResult::Reply(payload) => candid::decode_one(&payload).unwrap(),
                _ => panic!("\n🛑 transfer_tokens_and_posts failed\n"),
            };
            balance
        })
        .unwrap();

    assert_eq!(alice_yral_token_balance, 1000);

    assert_eq!(res, Err(MigrationErrors::Unauthorized));
}
pub type CanisterId = Principal;

#[derive(CandidType)]
struct NnsLedgerCanisterInitPayload {
    minting_account: String,
    initial_values: HashMap<String, Tokens>,
    send_whitelist: HashSet<CanisterId>,
    transfer_fee: Option<Tokens>,
}

#[derive(CandidType)]
struct CyclesMintingCanisterInitPayload {
    ledger_canister_id: CanisterId,
    governance_canister_id: CanisterId,
    minting_account_id: Option<String>,
    last_purged_notification: Option<BlockIndex>,
}

#[derive(CandidType)]
struct AuthorizedSubnetWorks {
    who: Option<Principal>,
    subnets: Vec<Principal>,
}
