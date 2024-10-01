use crate::{
    api::canister_management::update_last_access_time::update_last_canister_functionality_access_time,
    CANISTER_DATA,
};
use ic_cdk_macros::update;
use shared_utils::common::{
    types::{
        known_principal::KnownPrincipalType,
        utility_token::token_event::{MintEvent, TokenEvent},
    },
    utils::system_time,
};

#[update]
fn get_rewarded_for_signing_up() {
    // * access control
    let request_maker = ic_cdk::caller();
    let user_index_canister_principal_id = CANISTER_DATA.with(|canister_data_ref_cell| {
        canister_data_ref_cell
            .borrow()
            .known_principal_ids
            .get(&KnownPrincipalType::CanisterIdUserIndex)
            .cloned()
            .unwrap()
    });

    if user_index_canister_principal_id != request_maker {
        return;
    }

    update_last_canister_functionality_access_time();

    let current_time = system_time::get_current_system_time_from_ic();

    CANISTER_DATA.with(|canister_data_ref_cell| {
        let mut canister_data_ref = canister_data_ref_cell.borrow_mut();
        let my_principal_id = canister_data_ref.profile.principal_id.unwrap();
        let my_token_balance = &mut canister_data_ref.my_token_balance;

        let signup_reward_amount =
            TokenEvent::get_token_amount_for_token_event(&TokenEvent::Mint {
                amount: 0,
                details: MintEvent::NewUserSignup {
                    new_user_principal_id: my_principal_id,
                },
                timestamp: current_time,
            });

        my_token_balance.handle_token_event(TokenEvent::Mint {
            amount: signup_reward_amount,
            details: MintEvent::NewUserSignup {
                new_user_principal_id: my_principal_id,
            },
            timestamp: current_time,
        });
    });
}
