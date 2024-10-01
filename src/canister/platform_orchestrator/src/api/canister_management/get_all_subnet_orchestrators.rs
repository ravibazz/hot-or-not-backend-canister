use candid::Principal;
use ic_cdk_macros::query;

use crate::CANISTER_DATA;



#[query]
fn get_all_subnet_orchestrators() -> Vec<Principal> {
    CANISTER_DATA.with_borrow(|canister_data| {
      let canisters = canister_data.all_subnet_orchestrator_canisters_list.iter().map(|canister_id| {*canister_id}).collect::<Vec<Principal>>();
      canisters
    })
}