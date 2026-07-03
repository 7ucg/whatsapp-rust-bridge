// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only

use crate::kem;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct KyberPreKeyId(u32);

impl KyberPreKeyId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for KyberPreKeyId {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<KyberPreKeyId> for u32 {
    fn from(v: KyberPreKeyId) -> u32 {
        v.0
    }
}

pub struct KyberPreKeyRecord {
    id: KyberPreKeyId,
    timestamp: u64,
    key_pair: kem::KeyPair,
    signature: Vec<u8>,
}

impl KyberPreKeyRecord {
    pub fn new(
        id: KyberPreKeyId,
        timestamp: u64,
        key_pair: kem::KeyPair,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            id,
            timestamp,
            key_pair,
            signature,
        }
    }

    pub fn id(&self) -> KyberPreKeyId {
        self.id
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn key_pair(&self) -> &kem::KeyPair {
        &self.key_pair
    }

    pub fn public_key(&self) -> &kem::PublicKey {
        &self.key_pair.public_key
    }

    pub fn secret_key(&self) -> &kem::SecretKey {
        &self.key_pair.secret_key
    }

    pub fn signature(&self) -> &[u8] {
        &self.signature
    }
}
