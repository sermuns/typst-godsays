use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn godsays(amount_le_bytes: &[u8]) -> Vec<u8> {
    let amount = usize::from_le_bytes(amount_le_bytes.try_into().expect("array of size 4"));
    let god = godsays::God::init(amount);
    god.speak().into()
}
