/*
5.4.3 GWINFO
Length MsgType GwId GwAdd*
(octet 0) (1) (2) (3:n)
(*) only present if message is sent by a client
Table 8: GWINFO Message
The GWINFO message is sent as response to a SEARCHGW message using the broadcast service of the
underlying layer, with the radius as indicated in the SEARCHGW message. If sent by a GW, it contains only the
id of the sending GW; otherwise, if sent by a client, it also includes the address of the GW, see Table 8:
• Length and MsgType: see Section 5.2.
• GwId: the id of a GW.
• GwAdd: address of the indicated GW; optional, only included if message is sent by a client.
Like the SEARCHGW message the broadcast radius for this message is also indicated to the underlying
network layer when MQTT-SN gives this message for transmission.
*/
use crate::{
    eformat, function, BrokerLib::MqttSnClient, MSG_LEN_GW_INFO_HEADER,
    MSG_TYPE_GW_INFO,
};
use bytes::{BufMut, BytesMut};
use custom_debug::Debug;
use getset::{CopyGetters, Getters, MutGetters};
use log::*;
use std::str; // NOTE: needed for MutGetters

#[derive(
    // NOTE: must include std::str for MutGetters
    Debug,
    Clone,
    Getters,
    CopyGetters,
    MutGetters,
    Default,
    PartialEq,
    Hash,
    Eq,
)]
#[getset(get, set)]
pub struct GwInfo {
    pub len: u8,
    #[debug(format = "0x{:x}")]
    pub msg_type: u8,
    pub gw_id: u8,
    pub gw_addr: String,
}
impl GwInfo {
    pub fn send(
        gw_id: u8,
        gw_addr: String,
        client: &MqttSnClient,
    ) -> Result<(), String> {
        let len = MSG_LEN_GW_INFO_HEADER as usize + gw_addr.len() as usize;
        if len > 255 {
            return Err(format!("gw_addr too long: {}", len));
        }
        let mut bytes = BytesMut::with_capacity(len);
        let buf: &[u8] = &[len as u8, MSG_TYPE_GW_INFO, gw_id];
        bytes.put(buf);
        bytes.put(gw_addr.as_bytes());
        dbg!(&bytes);
        match client
            .transmit_tx
            .try_send((client.remote_addr, bytes.to_owned()))
        {
            Ok(()) => Ok(()),
            Err(err) => return Err(eformat!(client.remote_addr, err)),
        }
    }
    pub fn recv(
        buf: &[u8],
        size: usize,
        client: &MqttSnClient,
    ) -> Result<(), String> {
        let (gw_info, read_fixed_len) = GwInfo::try_read(buf, size).unwrap();
        info!(
            "{}: {} with {}",
            client.remote_addr, gw_info.gw_id, gw_info.gw_addr
        );
        Ok(())
    }
}
