//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

pub mod keys;
mod params;

use rand::{CryptoRng, Rng};

pub use self::keys::{ChainKey, MessageKeyGenerator, RootKey};
pub use self::params::{AliceSignalProtocolParameters, BobSignalProtocolParameters, UsePQRatchet};
use super::pqxdh;
use crate::protocol::state::SessionState;
use crate::protocol::{KeyPair, Result, SessionRecord};

type InitialPQRKey = [u8; 32];

pub fn derive_keys(secret_input: &[u8]) -> (RootKey, ChainKey, InitialPQRKey) {
    derive_keys_with_label(b"WhisperText".as_slice(), secret_input)
}

fn message_version() -> u8 {
    3
}

fn derive_keys_with_label(label: &[u8], secret_input: &[u8]) -> (RootKey, ChainKey, InitialPQRKey) {
    let mut secrets = [0; 96];
    hkdf::Hkdf::<sha2::Sha256>::new(None, secret_input)
        .expand(label, &mut secrets)
        .expect("valid length");
    let (root_key_bytes, chain_key_bytes, pqr_bytes) =
        (&secrets[0..32], &secrets[32..64], &secrets[64..96]);

    let root_key = RootKey::new(root_key_bytes.try_into().expect("correct length"));
    let chain_key = ChainKey::new(chain_key_bytes.try_into().expect("correct length"), 0);
    let pqr_key: InitialPQRKey = pqr_bytes.try_into().expect("correct length");

    (root_key, chain_key, pqr_key)
}

pub fn initialize_alice_session<R: Rng + CryptoRng>(
    parameters: &AliceSignalProtocolParameters,
    mut csprng: &mut R,
) -> Result<(SessionState, Option<Box<[u8]>>)> {
    let local_identity = parameters.our_identity_key_pair().identity_key();
    let sending_ratchet_key = KeyPair::generate(&mut csprng);

    // PQXDH path: when PQ ratchet is enabled and the bundle included a Kyber key.
    if matches!(parameters.use_pq_ratchet(), UsePQRatchet::Yes) {
        if let Some(their_kyber_pre_key) = parameters.their_kyber_pre_key() {
            let mut pq_params = pqxdh::InitiatorParameters::new(
                parameters.our_identity_key_pair().clone(),
                parameters.our_base_key_pair().clone(),
                *parameters.their_identity_key(),
                *parameters.their_signed_pre_key(),
                *parameters.their_ratchet_key(),
                their_kyber_pre_key.clone(),
            );
            if let Some(otp) = parameters.their_one_time_pre_key() {
                pq_params.set_their_one_time_pre_key(*otp);
            }

            let agreement = pqxdh::pqxdh_initiate(&pq_params, &mut csprng)?;
            let root_key = agreement.keys.root_key;
            let chain_key = agreement.keys.chain_key;
            let kyber_ciphertext = agreement.kyber_ciphertext;

            let (sending_chain_root_key, sending_chain_chain_key) = root_key.create_chain(
                parameters.their_ratchet_key(),
                &sending_ratchet_key.private_key,
            )?;

            let session = SessionState::new(
                message_version(),
                local_identity,
                parameters.their_identity_key(),
                &sending_chain_root_key,
                &parameters.our_base_key_pair().public_key,
            )
            .with_receiver_chain(parameters.their_ratchet_key(), &chain_key)
            .with_sender_chain(&sending_ratchet_key, &sending_chain_chain_key);

            return Ok((session, Some(kyber_ciphertext)));
        }
    }

    // Classic X3DH path (no Kyber).
    let mut secrets = [0u8; 160];
    let mut secrets_len = 0usize;

    secrets[..32].copy_from_slice(&[0xFFu8; 32]);
    secrets_len += 32;

    let our_base_private_key = parameters.our_base_key_pair().private_key.clone();

    let agreement = parameters
        .our_identity_key_pair()
        .private_key()
        .calculate_agreement(parameters.their_signed_pre_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    let agreement =
        our_base_private_key.calculate_agreement(parameters.their_identity_key().public_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    let agreement = our_base_private_key.calculate_agreement(parameters.their_signed_pre_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    if let Some(their_one_time_prekey) = parameters.their_one_time_pre_key() {
        let agreement = our_base_private_key.calculate_agreement(their_one_time_prekey)?;
        secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
        secrets_len += 32;
    }

    let (root_key, chain_key, _) = derive_keys(&secrets[..secrets_len]);

    let (sending_chain_root_key, sending_chain_chain_key) = root_key.create_chain(
        parameters.their_ratchet_key(),
        &sending_ratchet_key.private_key,
    )?;

    let session = SessionState::new(
        message_version(),
        local_identity,
        parameters.their_identity_key(),
        &sending_chain_root_key,
        &parameters.our_base_key_pair().public_key,
    )
    .with_receiver_chain(parameters.their_ratchet_key(), &chain_key)
    .with_sender_chain(&sending_ratchet_key, &sending_chain_chain_key);

    Ok((session, None))
}

pub fn initialize_bob_session(parameters: &BobSignalProtocolParameters) -> Result<SessionState> {
    let local_identity = parameters.our_identity_key_pair().identity_key();

    // PQXDH path: when PQ ratchet is enabled and the message included a Kyber ciphertext.
    if matches!(parameters.use_pq_ratchet(), UsePQRatchet::Yes) {
        if let (Some(our_kyber_pair), Some(their_ct)) = (
            parameters.our_kyber_pre_key_pair(),
            parameters.their_kyber_ciphertext(),
        ) {
            let pq_params = pqxdh::RecipientParameters::new(
                parameters.our_identity_key_pair().clone(),
                parameters.our_signed_pre_key_pair().clone(),
                parameters.our_one_time_pre_key_pair().cloned(),
                our_kyber_pair.clone(),
                *parameters.their_identity_key(),
                *parameters.their_base_key(),
                their_ct.clone(),
            );

            let keys = pqxdh::pqxdh_accept(&pq_params)?;

            let session = SessionState::new(
                message_version(),
                local_identity,
                parameters.their_identity_key(),
                &keys.root_key,
                parameters.their_base_key(),
            )
            .with_sender_chain(parameters.our_ratchet_key_pair(), &keys.chain_key);

            return Ok(session);
        }
    }

    // Classic X3DH path (no Kyber).
    let mut secrets = [0u8; 160];
    let mut secrets_len = 0usize;

    secrets[..32].copy_from_slice(&[0xFFu8; 32]);
    secrets_len += 32;

    let agreement = parameters
        .our_signed_pre_key_pair()
        .private_key
        .calculate_agreement(parameters.their_identity_key().public_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    let agreement = parameters
        .our_identity_key_pair()
        .private_key()
        .calculate_agreement(parameters.their_base_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    let agreement = parameters
        .our_signed_pre_key_pair()
        .private_key
        .calculate_agreement(parameters.their_base_key())?;
    secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
    secrets_len += 32;

    if let Some(our_one_time_pre_key_pair) = parameters.our_one_time_pre_key_pair() {
        let agreement = our_one_time_pre_key_pair
            .private_key
            .calculate_agreement(parameters.their_base_key())?;
        secrets[secrets_len..secrets_len + 32].copy_from_slice(&agreement);
        secrets_len += 32;
    }

    let (root_key, chain_key, _) = derive_keys(&secrets[..secrets_len]);

    let session = SessionState::new(
        message_version(),
        local_identity,
        parameters.their_identity_key(),
        &root_key,
        parameters.their_base_key(),
    )
    .with_sender_chain(parameters.our_ratchet_key_pair(), &chain_key);

    Ok(session)
}

pub fn initialize_alice_session_record<R: Rng + CryptoRng>(
    parameters: &AliceSignalProtocolParameters,
    csprng: &mut R,
) -> Result<SessionRecord> {
    let (session, _kyber_ciphertext) = initialize_alice_session(parameters, csprng)?;
    Ok(SessionRecord::new(session))
}

pub fn initialize_bob_session_record(
    parameters: &BobSignalProtocolParameters,
) -> Result<SessionRecord> {
    Ok(SessionRecord::new(initialize_bob_session(parameters)?))
}
