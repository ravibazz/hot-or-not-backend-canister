use ic_cdk::{api::is_controller, caller};
use ic_cdk_macros::update;
use shared_utils::common::types::wasm::{CanisterWasm, WasmType};

use crate::{guard::is_caller::is_caller_global_admin_or_controller, CANISTER_DATA};

 
#[update(guard = "is_caller_global_admin_or_controller")]
pub fn upload_wasms(wasm_type: WasmType, wasm: Vec<u8>) -> Result<String, String> {
    CANISTER_DATA.with_borrow_mut(|canister_data| {
        let canister_wasm  = CanisterWasm {
            wasm_blob: wasm,
            version: "1.0.0".into(),
        };
        canister_data.wasms.insert(wasm_type, canister_wasm);
        ic_cdk::println!("{} version ",canister_data.version_detail.version);
        canister_data.subnet_canister_upgrade_log.get(0);
    });
    Ok("Success".into())
}