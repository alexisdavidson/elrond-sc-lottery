use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("lottery");

    blockchain.register_contract_builder("file:output/lottery.wasm", ping_pong::ContractBuilder);
    blockchain
}

#[test]
fn ping_pong_call_ping_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-ping.scen.json", world());
}

#[test]
fn ping_pong_call_ping_second_user_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-ping-second-user.scen.json", world());
}

#[test]
fn ping_pong_call_ping_twice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-ping-twice.scen.json", world());
}

#[test]
fn ping_pong_call_ping_wrong_amount_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-ping-wrong-amount.scen.json", world());
}

#[test]
fn ping_pong_call_pong_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-pong.scen.json", world());
}

#[test]
fn ping_pong_call_pong_before_deadline_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/lottery-call-pong-before-deadline.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_pong_twice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-pong-twice.scen.json", world());
}

#[test]
fn ping_pong_call_pong_without_ping_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-call-pong-without-ping.scen.json", world());
}

#[test]
fn ping_pong_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/lottery-init.scen.json", world());
}
