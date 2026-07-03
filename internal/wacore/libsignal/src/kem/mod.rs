// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only

//! Keys and protocol functions for standard key encapsulation mechanisms (KEMs).

mod kyber1024;

use std::fmt;
use std::marker::PhantomData;

use rand::{CryptoRng, Rng};
use subtle::ConstantTimeEq;

use crate::protocol::error::SignalProtocolError;

type Result<T> = std::result::Result<T, SignalProtocolError>;

pub(crate) type RawCiphertext = Box<[u8]>;
pub type SerializedCiphertext = Box<[u8]>;

trait Parameters {
    const KEY_TYPE: KeyType;
    const PUBLIC_KEY_LENGTH: usize;
    const SECRET_KEY_LENGTH: usize;
    const CIPHERTEXT_LENGTH: usize;
    const SHARED_SECRET_LENGTH: usize;
    fn generate<R: CryptoRng + ?Sized>(
        csprng: &mut R,
    ) -> (KeyMaterial<Public>, KeyMaterial<Secret>);
    fn encapsulate<R: CryptoRng + ?Sized>(
        pub_key: &KeyMaterial<Public>,
        csprng: &mut R,
    ) -> std::result::Result<(Box<[u8]>, Box<[u8]>), BadKEMKeyLength>;
    fn decapsulate(
        secret_key: &KeyMaterial<Secret>,
        ciphertext: &[u8],
    ) -> std::result::Result<Box<[u8]>, DecapsulateError>;
}

trait DynParameters {
    fn public_key_length(&self) -> usize;
    fn secret_key_length(&self) -> usize;
    fn ciphertext_length(&self) -> usize;
    fn shared_secret_length(&self) -> usize;
    fn generate(&self, rng: &mut dyn CryptoRng) -> (KeyMaterial<Public>, KeyMaterial<Secret>);
    fn encapsulate(
        &self,
        pub_key: &KeyMaterial<Public>,
        csprng: &mut dyn CryptoRng,
    ) -> Result<(Box<[u8]>, Box<[u8]>)>;
    fn decapsulate(&self, secret_key: &KeyMaterial<Secret>, ciphertext: &[u8])
        -> Result<Box<[u8]>>;
}

impl<T: Parameters> DynParameters for T {
    fn public_key_length(&self) -> usize {
        Self::PUBLIC_KEY_LENGTH
    }
    fn secret_key_length(&self) -> usize {
        Self::SECRET_KEY_LENGTH
    }
    fn ciphertext_length(&self) -> usize {
        Self::CIPHERTEXT_LENGTH
    }
    fn shared_secret_length(&self) -> usize {
        Self::SHARED_SECRET_LENGTH
    }

    fn generate(&self, csprng: &mut dyn CryptoRng) -> (KeyMaterial<Public>, KeyMaterial<Secret>) {
        Self::generate(csprng)
    }

    fn encapsulate(
        &self,
        pub_key: &KeyMaterial<Public>,
        csprng: &mut dyn CryptoRng,
    ) -> Result<(Box<[u8]>, Box<[u8]>)> {
        Self::encapsulate(pub_key, csprng).map_err(|BadKEMKeyLength| {
            SignalProtocolError::InvalidArgument(format!(
                "bad KEM public key length: {}",
                pub_key.len()
            ))
        })
    }

    fn decapsulate(
        &self,
        secret_key: &KeyMaterial<Secret>,
        ciphertext: &[u8],
    ) -> Result<Box<[u8]>> {
        Self::decapsulate(secret_key, ciphertext).map_err(|e| match e {
            DecapsulateError::BadKeyLength => SignalProtocolError::InvalidArgument(format!(
                "bad KEM secret key length: {}",
                secret_key.len()
            )),
            DecapsulateError::BadCiphertext => SignalProtocolError::InvalidArgument(format!(
                "bad KEM ciphertext length: {}",
                ciphertext.len()
            )),
        })
    }
}

/// Helper trait for extracting the size of `libcrux_ml_kem` generic types.
pub(crate) trait ConstantLength {
    const LENGTH: usize;
}

impl<const N: usize> ConstantLength for libcrux_ml_kem::MlKemPrivateKey<N> {
    const LENGTH: usize = N;
}
impl<const N: usize> ConstantLength for libcrux_ml_kem::MlKemPublicKey<N> {
    const LENGTH: usize = N;
}
impl<const N: usize> ConstantLength for libcrux_ml_kem::MlKemCiphertext<N> {
    const LENGTH: usize = N;
}

pub(crate) struct BadKEMKeyLength;

pub(crate) enum DecapsulateError {
    BadKeyLength,
    BadCiphertext,
}

/// Designates a supported KEM protocol.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyType {
    Kyber1024,
}

impl KeyType {
    pub fn value(&self) -> u8 {
        match self {
            KeyType::Kyber1024 => 0x08,
        }
    }

    const fn parameters(&self) -> &'static dyn DynParameters {
        match self {
            KeyType::Kyber1024 => &kyber1024::Parameters,
        }
    }
}

impl TryFrom<u8> for KeyType {
    type Error = SignalProtocolError;

    fn try_from(x: u8) -> Result<Self> {
        match x {
            0x08 => Ok(KeyType::Kyber1024),
            t => Err(SignalProtocolError::InvalidArgument(format!(
                "unknown KEM key type: {t:#04x}"
            ))),
        }
    }
}

pub trait KeyKind {
    fn key_length(key_type: KeyType) -> usize;
}

pub enum Public {}

impl KeyKind for Public {
    fn key_length(key_type: KeyType) -> usize {
        key_type.parameters().public_key_length()
    }
}

pub enum Secret {}

impl KeyKind for Secret {
    fn key_length(key_type: KeyType) -> usize {
        key_type.parameters().secret_key_length()
    }
}

pub(crate) struct KeyMaterial<T: KeyKind> {
    data: Box<[u8]>,
    kind: PhantomData<T>,
}

impl<T: KeyKind> KeyMaterial<T> {
    fn new(data: Box<[u8]>) -> Self {
        KeyMaterial {
            data,
            kind: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: KeyKind> Clone for KeyMaterial<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            kind: PhantomData,
        }
    }
}

impl<T: KeyKind> std::ops::Deref for KeyMaterial<T> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl<const N: usize> From<libcrux_ml_kem::MlKemPublicKey<N>> for KeyMaterial<Public> {
    fn from(value: libcrux_ml_kem::MlKemPublicKey<N>) -> Self {
        KeyMaterial::new(value.as_ref().into())
    }
}

impl<const N: usize> From<libcrux_ml_kem::MlKemPrivateKey<N>> for KeyMaterial<Secret> {
    fn from(value: libcrux_ml_kem::MlKemPrivateKey<N>) -> Self {
        KeyMaterial::new(value.as_ref().into())
    }
}

pub struct Key<T: KeyKind> {
    key_type: KeyType,
    key_data: KeyMaterial<T>,
}

impl<T: KeyKind> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self {
            key_type: self.key_type,
            key_data: self.key_data.clone(),
        }
    }
}

impl<T: KeyKind> fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Key")
            .field("key_type", &self.key_type)
            .field("bytes_len", &self.key_data.len())
            .finish()
    }
}

impl<T: KeyKind> Key<T> {
    /// Deserialize from bytes produced by `Key<Kind>::serialize`.
    pub fn deserialize(value: &[u8]) -> Result<Self> {
        if value.is_empty() {
            return Err(SignalProtocolError::NoKeyTypeIdentifier);
        }
        let key_type = KeyType::try_from(value[0])?;
        let expected_len = T::key_length(key_type) + 1;
        if value.len() != expected_len {
            return Err(SignalProtocolError::InvalidArgument(format!(
                "bad KEM key length: expected {expected_len}, got {}",
                value.len()
            )));
        }
        Ok(Key {
            key_type,
            key_data: KeyMaterial::new(value[1..].into()),
        })
    }

    /// Serialize the key with a one-byte key-type prefix.
    pub fn serialize(&self) -> Box<[u8]> {
        let mut result = Vec::with_capacity(1 + self.key_data.len());
        result.push(self.key_type.value());
        result.extend_from_slice(&self.key_data);
        result.into_boxed_slice()
    }

    pub fn key_type(&self) -> KeyType {
        self.key_type
    }
}

impl Key<Public> {
    /// Encapsulate: returns `(shared_secret, serialized_ciphertext)`.
    pub fn encapsulate<R: CryptoRng>(
        &self,
        csprng: &mut R,
    ) -> Result<(Box<[u8]>, SerializedCiphertext)> {
        let (ss, ct) = self
            .key_type
            .parameters()
            .encapsulate(&self.key_data, csprng)?;
        Ok((
            ss,
            Ciphertext {
                key_type: self.key_type,
                data: &ct,
            }
            .serialize(),
        ))
    }
}

impl Key<Secret> {
    /// Decapsulate: returns the shared secret.
    pub fn decapsulate(&self, ct_bytes: &SerializedCiphertext) -> Result<Box<[u8]>> {
        let ct = Ciphertext::deserialize(ct_bytes)?;
        if ct.key_type != self.key_type {
            return Err(SignalProtocolError::InvalidArgument(format!(
                "KEM key type mismatch: ciphertext {:#04x}, key {:#04x}",
                ct.key_type.value(),
                self.key_type.value()
            )));
        }
        self.key_type
            .parameters()
            .decapsulate(&self.key_data, ct.data)
    }
}

impl TryFrom<&[u8]> for Key<Public> {
    type Error = SignalProtocolError;
    fn try_from(value: &[u8]) -> Result<Self> {
        Self::deserialize(value)
    }
}

impl TryFrom<&[u8]> for Key<Secret> {
    type Error = SignalProtocolError;
    fn try_from(value: &[u8]) -> Result<Self> {
        Self::deserialize(value)
    }
}

impl subtle::ConstantTimeEq for Key<Public> {
    fn ct_eq(&self, other: &Self) -> subtle::Choice {
        if self.key_type != other.key_type {
            return 0.ct_eq(&1);
        }
        self.key_data.ct_eq(&other.key_data)
    }
}

impl PartialEq for Key<Public> {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}

impl Eq for Key<Public> {}

/// A KEM public key with the ability to encapsulate a shared secret.
pub type PublicKey = Key<Public>;

/// A KEM secret key with the ability to decapsulate a shared secret.
pub type SecretKey = Key<Secret>;

/// A public/secret key pair for a KEM protocol.
#[derive(Clone)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl KeyPair {
    pub fn generate<R: Rng + CryptoRng>(key_type: KeyType, csprng: &mut R) -> Self {
        let (pk, sk) = key_type.parameters().generate(csprng);
        Self {
            secret_key: SecretKey {
                key_type,
                key_data: sk,
            },
            public_key: PublicKey {
                key_type,
                key_data: pk,
            },
        }
    }

    pub fn new(public_key: PublicKey, secret_key: SecretKey) -> Self {
        assert_eq!(public_key.key_type, secret_key.key_type);
        Self {
            public_key,
            secret_key,
        }
    }

    pub fn from_public_and_private(public_key: &[u8], secret_key: &[u8]) -> Result<Self> {
        let public_key = PublicKey::try_from(public_key)?;
        let secret_key = SecretKey::try_from(secret_key)?;
        if public_key.key_type != secret_key.key_type {
            return Err(SignalProtocolError::InvalidArgument(
                "KEM public/secret key type mismatch".to_string(),
            ));
        }
        Ok(Self {
            public_key,
            secret_key,
        })
    }
}

struct Ciphertext<'a> {
    key_type: KeyType,
    data: &'a [u8],
}

impl<'a> Ciphertext<'a> {
    pub fn deserialize(value: &'a [u8]) -> Result<Self> {
        if value.is_empty() {
            return Err(SignalProtocolError::NoKeyTypeIdentifier);
        }
        let key_type = KeyType::try_from(value[0])?;
        let expected_len = key_type.parameters().ciphertext_length() + 1;
        if value.len() != expected_len {
            return Err(SignalProtocolError::InvalidArgument(format!(
                "bad KEM ciphertext length: expected {expected_len}, got {}",
                value.len()
            )));
        }
        Ok(Ciphertext {
            key_type,
            data: &value[1..],
        })
    }

    pub fn serialize(&self) -> SerializedCiphertext {
        let mut result = Vec::with_capacity(1 + self.data.len());
        result.push(self.key_type.value());
        result.extend_from_slice(self.data);
        result.into_boxed_slice()
    }
}
