// Copyright 2026 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only

//! PQXDH key agreement protocol.
//!
//! Implements the Post-Quantum Extended Diffie-Hellman handshake:
//! 4 EC DH agreements + 1 ML-KEM (Kyber1024) encapsulation/decapsulation.

use rand::{CryptoRng, Rng};

use crate::kem;
use crate::protocol::ratchet::keys::{ChainKey, RootKey};
use crate::protocol::{IdentityKey, IdentityKeyPair, KeyPair, PublicKey, Result};

pub type InitialPQRKey = [u8; 32];

/// Keys derived from a PQXDH handshake, ready for ratchet initialization.
pub struct HandshakeKeys {
    pub root_key: RootKey,
    pub chain_key: ChainKey,
    pub pqr_key: InitialPQRKey,
}

impl HandshakeKeys {
    fn derive(secret_input: &[u8]) -> Self {
        Self::derive_with_label(
            b"WhisperText_X25519_SHA-256_CRYSTALS-KYBER-1024",
            secret_input,
        )
    }

    fn derive_with_label(label: &[u8], secret_input: &[u8]) -> Self {
        let mut derived = [0u8; 96];
        hkdf::Hkdf::<sha2::Sha256>::new(None, secret_input)
            .expand(label, &mut derived)
            .expect("valid length");
        let root_key_bytes: [u8; 32] = derived[0..32].try_into().expect("correct length");
        let chain_key_bytes: [u8; 32] = derived[32..64].try_into().expect("correct length");
        let pqr_bytes: [u8; 32] = derived[64..96].try_into().expect("correct length");

        Self {
            root_key: RootKey::new(root_key_bytes),
            chain_key: ChainKey::new(chain_key_bytes, 0),
            pqr_key: pqr_bytes,
        }
    }
}

/// Output of the initiator side of PQXDH.
pub struct InitiatorAgreement {
    pub keys: HandshakeKeys,
    /// The KEM ciphertext the recipient needs to complete the agreement.
    pub kyber_ciphertext: kem::SerializedCiphertext,
}

// ── Initiator ────────────────────────────────────────────────────────

/// Parameters for the initiator side of a PQXDH key agreement.
pub struct InitiatorParameters {
    our_identity_key_pair: IdentityKeyPair,
    our_ephemeral_key_pair: KeyPair,

    their_identity_key: IdentityKey,
    their_signed_pre_key: PublicKey,
    their_one_time_pre_key: Option<PublicKey>,
    their_ratchet_key: PublicKey,
    their_kyber_pre_key: kem::PublicKey,
}

impl InitiatorParameters {
    pub fn new(
        our_identity_key_pair: IdentityKeyPair,
        our_ephemeral_key_pair: KeyPair,
        their_identity_key: IdentityKey,
        their_signed_pre_key: PublicKey,
        their_ratchet_key: PublicKey,
        their_kyber_pre_key: kem::PublicKey,
    ) -> Self {
        Self {
            our_identity_key_pair,
            our_ephemeral_key_pair,
            their_identity_key,
            their_one_time_pre_key: None,
            their_signed_pre_key,
            their_ratchet_key,
            their_kyber_pre_key,
        }
    }

    pub fn set_their_one_time_pre_key(&mut self, ec_public: PublicKey) {
        self.their_one_time_pre_key = Some(ec_public);
    }
}

/// Perform the initiator side of the PQXDH key agreement.
pub fn pqxdh_initiate<R: Rng + CryptoRng>(
    parameters: &InitiatorParameters,
    csprng: &mut R,
) -> Result<InitiatorAgreement> {
    let mut secrets = Vec::with_capacity(32 * 6);

    // discontinuity bytes
    secrets.extend_from_slice(&[0xFFu8; 32]);

    secrets.extend_from_slice(
        &parameters
            .our_identity_key_pair
            .private_key()
            .calculate_agreement(&parameters.their_signed_pre_key)?,
    );

    let our_ephemeral_private_key = parameters.our_ephemeral_key_pair.private_key.clone();

    secrets.extend_from_slice(
        &our_ephemeral_private_key
            .calculate_agreement(parameters.their_identity_key.public_key())?,
    );

    secrets.extend_from_slice(
        &our_ephemeral_private_key.calculate_agreement(&parameters.their_signed_pre_key)?,
    );

    if let Some(their_one_time_prekey) = &parameters.their_one_time_pre_key {
        secrets.extend_from_slice(
            &our_ephemeral_private_key.calculate_agreement(their_one_time_prekey)?,
        );
    }

    let kyber_ciphertext = {
        let (ss, ct) = parameters.their_kyber_pre_key.encapsulate(csprng)?;
        secrets.extend_from_slice(ss.as_ref());
        ct
    };

    Ok(InitiatorAgreement {
        keys: HandshakeKeys::derive(&secrets),
        kyber_ciphertext,
    })
}

// ── Recipient ────────────────────────────────────────────────────────

/// Parameters for the recipient side of a PQXDH key agreement.
pub struct RecipientParameters {
    our_identity_key_pair: IdentityKeyPair,
    our_signed_pre_key_pair: KeyPair,
    our_one_time_pre_key_pair: Option<KeyPair>,
    our_kyber_pre_key_pair: kem::KeyPair,

    their_identity_key: IdentityKey,
    their_ephemeral_key: PublicKey,
    their_kyber_ciphertext: Box<[u8]>,
}

impl RecipientParameters {
    pub fn new(
        our_identity_key_pair: IdentityKeyPair,
        our_signed_pre_key_pair: KeyPair,
        our_one_time_pre_key_pair: Option<KeyPair>,
        our_kyber_pre_key_pair: kem::KeyPair,
        their_identity_key: IdentityKey,
        their_ephemeral_key: PublicKey,
        their_kyber_ciphertext: Box<[u8]>,
    ) -> Self {
        Self {
            our_identity_key_pair,
            our_signed_pre_key_pair,
            our_one_time_pre_key_pair,
            our_kyber_pre_key_pair,
            their_identity_key,
            their_ephemeral_key,
            their_kyber_ciphertext,
        }
    }
}

/// Perform the recipient side of the PQXDH key agreement.
pub fn pqxdh_accept(parameters: &RecipientParameters) -> Result<HandshakeKeys> {
    let mut secrets = Vec::with_capacity(32 * 6);

    // discontinuity bytes
    secrets.extend_from_slice(&[0xFFu8; 32]);

    secrets.extend_from_slice(
        &parameters
            .our_signed_pre_key_pair
            .private_key
            .calculate_agreement(parameters.their_identity_key.public_key())?,
    );

    secrets.extend_from_slice(
        &parameters
            .our_identity_key_pair
            .private_key()
            .calculate_agreement(&parameters.their_ephemeral_key)?,
    );

    secrets.extend_from_slice(
        &parameters
            .our_signed_pre_key_pair
            .private_key
            .calculate_agreement(&parameters.their_ephemeral_key)?,
    );

    if let Some(our_one_time_pre_key_pair) = &parameters.our_one_time_pre_key_pair {
        secrets.extend_from_slice(
            &our_one_time_pre_key_pair
                .private_key
                .calculate_agreement(&parameters.their_ephemeral_key)?,
        );
    }

    secrets.extend_from_slice(
        &parameters
            .our_kyber_pre_key_pair
            .secret_key
            .decapsulate(&parameters.their_kyber_ciphertext)?,
    );

    Ok(HandshakeKeys::derive(&secrets))
}
