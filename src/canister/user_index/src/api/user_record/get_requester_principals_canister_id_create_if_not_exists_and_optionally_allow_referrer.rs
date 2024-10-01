use crate::{
    util::canister_management::{
        check_and_request_cycles_from_platform_orchestrator, create_empty_user_canister,
        install_canister_wasm,
    },
    CANISTER_DATA,
};
use candid::Principal;
use ic_cdk::api::call;
use ic_cdk_macros::update;
use shared_utils::{
    canister_specific::individual_user_template::types::session::SessionType,
    common::{
        types::{
            known_principal::KnownPrincipalType,
            wasm::{CanisterWasm, WasmType},
        },
        utils::task::run_task_concurrently,
    },
    constant::{
        get_backup_individual_user_canister_batch_size,
        get_backup_individual_user_canister_threshold,
        get_individual_user_canister_subnet_batch_size,
        get_individual_user_canister_subnet_threshold,
        INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY,
    },
};

#[update]
async fn get_requester_principals_canister_id_create_if_not_exists() -> Result<Principal, String> {
    let api_caller = ic_cdk::caller();

    if api_caller == Principal::anonymous() {
        panic!("Anonymous principal is not allowed to call this method");
    }

    let canister_id_for_this_caller = CANISTER_DATA.with(|canister_data_ref_cell| {
        canister_data_ref_cell
            .borrow()
            .user_principal_id_to_canister_id_map
            .get(&api_caller)
            .cloned()
    });

    match canister_id_for_this_caller {
        // * canister already exists
        Some(canister_id) => Ok(canister_id),
        None => {
            // * create new canister
            let created_canister_id = new_user_signup(api_caller).await?;
            // * reward user for signing up
            call::notify(created_canister_id, "get_rewarded_for_signing_up", ()).ok();

            Ok(created_canister_id)
        }
    }
}

#[deprecated(note = "use get_requester_principals_canister_id_create_if_not_exists instead")]
#[update]
async fn get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(
) -> Principal {
    let api_caller = ic_cdk::caller();

    if api_caller == Principal::anonymous() {
        panic!("Anonymous principal is not allowed to call this method");
    }

    let canister_id_for_this_caller = CANISTER_DATA.with(|canister_data_ref_cell| {
        canister_data_ref_cell
            .borrow()
            .user_principal_id_to_canister_id_map
            .get(&api_caller)
            .cloned()
    });

    match canister_id_for_this_caller {
        // * canister already exists
        Some(canister_id) => canister_id,
        None => {
            // * create new canister
            let created_canister_id = new_user_signup(api_caller).await.unwrap();

            // * reward user for signing up
            call::notify(created_canister_id, "get_rewarded_for_signing_up", ()).ok();

            created_canister_id
        }
    }
}

async fn new_user_signup(user_id: Principal) -> Result<Principal, String> {
    //check if sigups are enabled on this subnet
    let is_signup_enabled = CANISTER_DATA
        .with_borrow(|canister_data| canister_data.configuration.signups_open_on_this_subnet);

    if !is_signup_enabled {
        return Err("Signups closed on this subnet".into());
    }

    let user_canister_id = CANISTER_DATA.with_borrow(|canister_data| {
        canister_data
            .user_principal_id_to_canister_id_map
            .get(&user_id)
            .cloned()
    });

    if user_canister_id.is_some() {
        return Ok(user_canister_id.unwrap());
    }

    let canister_id_res = CANISTER_DATA
        .with_borrow_mut(|canister_data| {
            let mut available_canisters = canister_data.available_canisters.iter().cloned();
            let canister_id = available_canisters.next();
            canister_data.available_canisters = available_canisters.collect();
            canister_id
        })
        .ok_or("Not Available".into());

    let response = match canister_id_res {
        Ok(canister_id) => {
            //Set owner of canister as this principal
            call::call(canister_id, "update_profile_owner", (user_id,))
                .await
                .map_err(|e| e.1)?;
            CANISTER_DATA.with_borrow_mut(|canister_data| {
                canister_data
                    .user_principal_id_to_canister_id_map
                    .insert(user_id, canister_id)
            });

            //update session type for the user
            call::call(
                canister_id,
                "update_session_type",
                (SessionType::AnonymousSession,),
            )
            .await
            .map_err(|e| e.1)?;

            Ok(canister_id)
        }
        Err(e) => Err(e),
    };

    let individual_user_canisters_cnt = CANISTER_DATA.with_borrow(|canister_data| {
        canister_data.user_principal_id_to_canister_id_map.len() as u64
    });
    let available_individual_user_canisters_cnt =
        CANISTER_DATA.with_borrow(|canister_data| canister_data.available_canisters.len() as u64);
    let backup_individual_user_canister_cnt =
        CANISTER_DATA.with_borrow(|canister_data| canister_data.backup_canister_pool.len() as u64);
    let total_canister_provisioned_on_subnet = individual_user_canisters_cnt
        + available_individual_user_canisters_cnt
        + backup_individual_user_canister_cnt;

    // notify platform_orchestrator that this subnet has reached maximum capacity.
    if response.is_err()
        && individual_user_canisters_cnt > INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY
    {
        let platform_orchestrator_canister_id = CANISTER_DATA.with_borrow(|canister_data| {
            *canister_data
                .configuration
                .known_principal_ids
                .get(&KnownPrincipalType::CanisterIdPlatformOrchestrator)
                .unwrap_or(&Principal::anonymous())
        });
        ic_cdk::notify(
            platform_orchestrator_canister_id,
            "subnet_orchestrator_maxed_out",
            (),
        )
        .unwrap_or_default();
    }

    let individual_user_template_canister_wasm = CANISTER_DATA.with_borrow(|canister_data| {
        canister_data
            .wasms
            .get(&WasmType::IndividualUserWasm)
            .unwrap()
            .clone()
    });
    let individual_user_canister_subnet_threshold = get_individual_user_canister_subnet_threshold();
    if available_individual_user_canisters_cnt < individual_user_canister_subnet_threshold {
        ic_cdk::spawn(provision_new_available_canisters(
            individual_user_template_canister_wasm,
        ));
    }

    let backup_individual_user_canister_batch_size =
        get_backup_individual_user_canister_batch_size();
    let backup_individual_user_canister_threshold = get_backup_individual_user_canister_threshold();
    if total_canister_provisioned_on_subnet < INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY
        && backup_individual_user_canister_cnt < backup_individual_user_canister_threshold
    {
        let new_canister_cnt = backup_individual_user_canister_batch_size.min(
            INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY - total_canister_provisioned_on_subnet,
        );
        ic_cdk::spawn(provision_new_backup_canisters(new_canister_cnt));
    }

    response
}

async fn provision_new_available_canisters(individual_user_template_canister_wasm: CanisterWasm) {
    let backup_pool_canister_count =
        CANISTER_DATA.with_borrow(|canister_data| canister_data.backup_canister_pool.len() as u64);
    let individual_canister_batch_size = get_individual_user_canister_subnet_batch_size();
    let canister_count = individual_canister_batch_size.min(backup_pool_canister_count);
    let available_canister_count =
        CANISTER_DATA.with_borrow(|canister_data| canister_data.available_canisters.len() as u64);
    let max_canister_count = available_canister_count + canister_count;

    let install_canister_wasm_futures = CANISTER_DATA.with_borrow(|canister_data| {
        let mut backup_pool_canister = canister_data.backup_canister_pool.clone().into_iter();
        (0..canister_count).map(move |_| {
            let individual_user_template_canister_wasm_version =
                individual_user_template_canister_wasm.version.clone();

            let individual_user_template_canister_wasm =
                individual_user_template_canister_wasm.wasm_blob.clone();

            let canister_id = backup_pool_canister.next().unwrap();

            async move {
                let _ = check_and_request_cycles_from_platform_orchestrator().await;
                let future = install_canister_wasm(
                    canister_id,
                    None,
                    individual_user_template_canister_wasm_version,
                    individual_user_template_canister_wasm,
                );
                future.await
            }
        })
    });

    let result_callback = |canister_id: Principal| {
        CANISTER_DATA.with_borrow_mut(|canister_data| {
            canister_data.available_canisters.insert(canister_id);
            canister_data.backup_canister_pool.remove(&canister_id);
        })
    };

    let breaking_condition = || {
        CANISTER_DATA.with_borrow(|canister_data| {
            canister_data.available_canisters.len() as u64 >= max_canister_count
        })
    };

    run_task_concurrently(
        install_canister_wasm_futures,
        10,
        result_callback,
        breaking_condition,
    )
    .await;
}

async fn provision_new_backup_canisters(canister_count: u64) {
    let create_canister_futures = (0..canister_count).map(|_| {
        let future = create_empty_user_canister();
        future
    });

    let result_callback = |canister_id: Principal| {
        CANISTER_DATA.with_borrow_mut(|canister_data| {
            canister_data.backup_canister_pool.insert(canister_id)
        });
    };

    let breaking_condition = || {
        CANISTER_DATA.with_borrow(|canister_data| {
            canister_data.backup_canister_pool.len() as u64
                > get_backup_individual_user_canister_batch_size()
        })
    };

    run_task_concurrently(
        create_canister_futures.into_iter(),
        10,
        result_callback,
        breaking_condition,
    )
    .await;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ingress_principal_id_equality_from_different_sources() {
        assert_eq!("2vxsx-fae".to_string(), Principal::anonymous().to_text());
        assert_eq!(
            Principal::from_text("2vxsx-fae").unwrap(),
            Principal::anonymous()
        );
    }
}
