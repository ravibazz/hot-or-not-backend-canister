use candid::Principal;
use shared_utils::common::types::known_principal::KnownPrincipalType;

use crate::CANISTER_DATA;

pub struct SubnetOrchestrator {
    canister_id: Principal,
}

impl SubnetOrchestrator {
    pub fn new() -> Result<Self, String> {
        let subnet_orchestrator = CANISTER_DATA.with_borrow(|canister_data| {
            let canister_id = canister_data
                .known_principal_ids
                .get(&KnownPrincipalType::CanisterIdUserIndex)
                .copied();

            canister_id.map(|canister_id| Self { canister_id })
        });

        subnet_orchestrator.ok_or("Subnet Orchestrator canister not found".into())
    }

    pub async fn get_empty_canister(&self) -> Result<Principal, String> {
        let (result,) = ic_cdk::call(self.canister_id, "allot_empty_canister", ())
            .await
            .map_err(|e| e.1)?;

        result
    }
}
