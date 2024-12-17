use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature;
use solana_sdk::signature::Signer;
use solana_sdk::signature::Signature;
use solana_sdk::signature::SignerError;
use std::fmt;
use std::fmt::Display;

/// A tokio compatible wrapper for `anchor_client::solana_sdk::signature::Keypair`
///
/// The standard Keypair is not Sendable and cannot be used from
/// within a Tokio runtime.
///
/// This Keypair works by storing the keypair bytes and only
/// rebuilding the original anchor_client Keypair when inside
/// the synchronous context of the Signer trait
#[derive(Clone, PartialEq, Eq)]
pub struct Keypair {
    keypair: [u8; 64],
}

pub trait SendableSigner: Send + Sync + Signer + PartialEq {
    fn to_keypair(&self) -> signature::Keypair;
}

impl SendableSigner for Keypair {
    fn to_keypair(self: &Self) -> signature::Keypair {
        signature::Keypair::from_bytes(&self.keypair).unwrap()
    }
}

impl Display for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keypair = signature::Keypair::from_bytes(&self.keypair).map_err(|_| fmt::Error)?;
        write!(f, "{}", keypair.to_base58_string())
    }
}

impl Keypair {
    pub fn new<T: AsRef<signature::Keypair>>(keypair: T) -> Self {
        Self {
            keypair: keypair.as_ref().to_bytes(),
        }
    }

    pub fn from_keypair(keypair: &signature::Keypair) -> Self {
        Self {
            keypair: keypair.to_bytes(),
        }
    }

    pub fn from_base58_string(s: &str) -> Self {
        Self {
            keypair: signature::Keypair::from_base58_string(s).to_bytes(),
        }
    }
}

impl Signer for Keypair {
    fn try_pubkey(&self) -> std::result::Result<Pubkey, SignerError> {
        // Convert the stored keypair bytes back to a Keypair
        let keypair = signature::Keypair::from_bytes(&self.keypair)
            .map_err(|e| SignerError::Custom(e.to_string()))?;

        // Return the public key of the keypair
        Ok(keypair.pubkey())
    }

    fn try_sign_message(&self, message: &[u8]) -> std::result::Result<Signature, SignerError> {
        // Convert the stored keypair bytes back to a Keypair
        let keypair = signature::Keypair::from_bytes(&self.keypair)
            .map_err(|e| SignerError::Custom(e.to_string()))?;

        // Sign the message with the keypair
        Ok(keypair.try_sign_message(message)?)
    }

    fn is_interactive(&self) -> bool {
        // This method should return true if the signer requires user interaction to sign messages.
        // In this case, we assume that it does not require user interaction, so we return false.
        false
    }
}
