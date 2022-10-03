# Reference: https://github.com/CosmWasm/cosmwasm/blob/main/contracts/ibc-reflect/src/contract.rs
<!-- msg.rs -->
#[cw_serde]
pub enum PacketMsg {
    Dispatch { msgs: Vec<CosmosMsg> },
    WhoAmI {},
    Balances {},
    UpdatePrice { denom: String, price: Uint128 }
}
      
<!-- contract.rs (producer) -->
pub fn update_price_from_producer (deps: DepsMut) {
    <!--    Receive price data    -->
    let packet = to_binary(PacketMsg::UpdatePrice { denom, price } )
    let ibc_msg = SubMessage::new(IbcMsg::SendPacket {
        channel_id: msg.channel,
        data: to_binary(&packet)?,
        timeout: timeout.into(),
    })
    Response::new()
        .add_submessage(ibc_message)
}
      
<!-- contract.rs (consumer) -->
#[entry_point]
/// we look for a the proper reflect contract to relay to and send the message
/// We cannot return any meaningful response value as we do not know the response value
/// of execution. We just return ok if we dispatched, error if we failed to dispatch
pub fn ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    // put this in a closure so we can convert all error responses into acknowledgements
    (|| {
        let packet = msg.packet;
        // which local channel did this packet come on
        let caller = packet.dest.channel_id;
        let msg: PacketMsg = from_slice(&packet.data)?;
        match msg {
            PacketMsg::Dispatch { msgs } => receive_dispatch(deps, caller, msgs),
            PacketMsg::WhoAmI {} => receive_who_am_i(deps, caller),
            PacketMsg::Balances {} => receive_balances(deps, caller),
            PacketMsg::UpdatePrice { denom, price } => execute_update_price(deps, caller, denom, price)
        }
    })()
    .or_else(|e| {
        // we try to capture all app-level errors and convert them into
        // acknowledgement packets that contain an error code.
        let acknowledgement = encode_ibc_error(format!("invalid packet: {}", e));
        Ok(IbcReceiveResponse::new()
            .set_ack(acknowledgement)
            .add_event(Event::new("ibc").add_attribute("packet", "receive")))
    })
}

pub fn execute_update_price(deps: DepsMut, caller: String, denom:String, price: Uint128) -> Result<Response, ContractError> {
    PRICES.save(deps.storage, &denom, price)?;
}
      
<!-- state.rs (consumer) -->
const PRICES: Map<&str, Uint128> = Map::new("prices");