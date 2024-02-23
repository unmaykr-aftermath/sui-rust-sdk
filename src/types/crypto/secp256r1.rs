//! Implementation of secp256r1 public-key cryptogrophy.

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Secp256r1PrivateKey(
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array32>>")
    )]
    [u8; Self::LENGTH],
);

impl Secp256r1PrivateKey {
    /// The length of an secp256r1 private key in bytes.
    pub const LENGTH: usize = 32;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Secp256r1PublicKey(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array33, [::serde_with::Same; 33]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl Secp256r1PublicKey {
    /// The length of an secp256r1 public key in bytes.
    pub const LENGTH: usize = 33;
}

impl std::fmt::Display for Secp256r1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display33(&self.0), f)
    }
}

impl std::fmt::Debug for Secp256r1PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256r1PublicKey")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Secp256r1Signature(
    #[cfg_attr(
        feature = "serde",
        serde(
            with = "::serde_with::As::<::serde_with::IfIsHumanReadable<super::Base64Array64, [::serde_with::Same; 64]>>"
        )
    )]
    [u8; Self::LENGTH],
);

impl Secp256r1Signature {
    /// The length of an secp256r1 signature key in bytes.
    pub const LENGTH: usize = 64;
}

impl std::fmt::Display for Secp256r1Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&super::Base64Display64(&self.0), f)
    }
}

impl std::fmt::Debug for Secp256r1Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secp256r1Signature")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}
