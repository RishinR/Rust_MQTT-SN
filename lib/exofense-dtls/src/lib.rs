#![warn(rust_2018_idioms)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
use anyhow::Result;
use async_trait::async_trait;
#[async_trait]
pub trait KeyingMaterialExporter {
    async fn export_keying_material(
        &self,
        label: &str,
        context: &[u8],
        length: usize,
    ) -> Result<Vec<u8>>;
}

pub mod fixed_big_int;
pub mod udp_conn;
pub mod buffer;
pub mod replay_detector;
pub mod alert;
pub mod application_data;
pub mod change_cipher_spec;
pub mod cipher_suite;
pub mod client_certificate_type;
pub mod compression_methods;
pub mod config;
pub mod conn;
pub mod content;
pub mod crypto;
pub mod curve;
pub mod error;
pub mod extension;
pub mod flight;
pub mod fragment_buffer;
pub mod handshake;
pub mod handshaker;
pub mod listener;
pub mod prf;
pub mod record_layer;
pub mod signature_hash_algorithm;
pub mod state;
pub mod connection_db;
pub mod mio_udp_socket;

use cipher_suite::*;
use extension::extension_use_srtp::SrtpProtectionProfile;

pub(crate) fn find_matching_srtp_profile(
    a: &[SrtpProtectionProfile],
    b: &[SrtpProtectionProfile],
) -> Result<SrtpProtectionProfile, ()> {
    for a_profile in a {
        for b_profile in b {
            if a_profile == b_profile {
                return Ok(*a_profile);
            }
        }
    }
    Err(())
}

pub(crate) fn find_matching_cipher_suite(
    a: &[CipherSuiteId],
    b: &[CipherSuiteId],
) -> Result<CipherSuiteId, ()> {
    for a_suite in a {
        for b_suite in b {
            if a_suite == b_suite {
                return Ok(*a_suite);
            }
        }
    }
    Err(())
}