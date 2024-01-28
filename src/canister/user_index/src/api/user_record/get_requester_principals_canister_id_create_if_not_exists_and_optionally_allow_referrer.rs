use crate::{util::canister_management::create_users_canister, CANISTER_DATA};
use candid::Principal;
use ic_cdk::api::call;
use shared_utils::{common::{types::wasm::WasmType, utils::task::run_task_concurrently}, constant::{INDIVIDUAL_USER_CANISTER_SUBNET_BATCH_SIZE, INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY, INDIVIDUAL_USER_CANISTER_SUBNET_THREESHOLD}};

#[ic_cdk::update]
#[candid::candid_method(update)]
async fn get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(
    referrer: Option<Principal>,
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

            // * reward referrer for referring
            if let Some(referrer_principal_id) = referrer {
                let referrer_canister_id = CANISTER_DATA.with(|canister_data_ref_cell| {
                    canister_data_ref_cell
                        .borrow()
                        .user_principal_id_to_canister_id_map
                        .get(&referrer_principal_id)
                        .cloned()
                });
                if let Some(referrer_canister_id) = referrer_canister_id {
                    call::notify(
                        referrer_canister_id,
                        "get_rewarded_for_referral",
                        (referrer_principal_id, api_caller),
                    )
                    .ok();
                    call::notify(
                        created_canister_id,
                        "get_rewarded_for_referral",
                        (referrer_principal_id, api_caller),
                    )
                    .ok();
                }
            }

            created_canister_id
        }
    }
}

async fn new_user_signup(user_id: Principal) -> Result<Principal, String> {
    //check if sigups are enabled on this subnet
    let is_signup_enabled = CANISTER_DATA.with_borrow(|canister_data| canister_data.configuration.signups_open_on_this_subnet);

    if !is_signup_enabled {
        return Err("Signups closed on this subnet".into());
    }

    let user_canister_id = CANISTER_DATA.with_borrow(|canister_data| canister_data.user_principal_id_to_canister_id_map.get(&user_id).cloned());

    if user_canister_id.is_some() {
        return Ok(user_canister_id.unwrap());
    }

    let canister_id = CANISTER_DATA.with_borrow_mut(|canister_data| {
        let mut available_canisters =  canister_data.available_canisters.iter().cloned();
        let canister_id = available_canisters.next();
        canister_data.available_canisters = available_canisters.collect();
        canister_id
    })
    .ok_or("Not Available")?;


    
    // provision new canisters on the subnet if required.
    let individual_user_canisters_cnt = CANISTER_DATA.with_borrow(|canister_data| canister_data.user_principal_id_to_canister_id_map.len() as u64);
    let available_individual_user_canisters_cnt = CANISTER_DATA.with_borrow(|canister_data| canister_data.available_canisters.len() as u64);
    if individual_user_canisters_cnt + available_individual_user_canisters_cnt < INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY && available_individual_user_canisters_cnt < INDIVIDUAL_USER_CANISTER_SUBNET_THREESHOLD {
        let provision_canister_cnt = u64::min(INDIVIDUAL_USER_CANISTER_SUBNET_BATCH_SIZE, INDIVIDUAL_USER_CANISTER_SUBNET_MAX_CAPACITY - available_individual_user_canisters_cnt - individual_user_canisters_cnt);
        ic_cdk::spawn(provision_new_canister(provision_canister_cnt));
    }

    //Set owner of canister as this principal
    let _res: () = call::call(
        canister_id,
        "update_profile_owner",
        (user_id,)
    )
    .await
    .map_err(|e| e.1)?;
    CANISTER_DATA.with_borrow_mut(|canister_data|
        canister_data.user_principal_id_to_canister_id_map.insert(user_id, canister_id)
    );

    Ok(canister_id)
}

async fn provision_new_canister(provision_canister_cnt: u64) {
    let mut create_canister_futures = vec![];
    for _ in 0..provision_canister_cnt {
        create_canister_futures.push(
            create_users_canister(
                None,
                CANISTER_DATA.with_borrow(|canister_data| canister_data.last_run_upgrade_status.version.clone()),
                CANISTER_DATA.with_borrow(|canister_data| canister_data.wasms.get(&WasmType::IndividualUserWasm).unwrap().as_ref().to_vec())
            )
        )
    }
    run_task_concurrently(create_canister_futures.into_iter(), 10, |_| {}, || false).await;
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
