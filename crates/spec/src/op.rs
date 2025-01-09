/// An Optimism hardfork.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
)]
pub enum OpHardfork {
    /// Bedrock: <https://blog.oplabs.co/introducing-optimism-bedrock>.
    Bedrock,
    /// Regolith: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#regolith>.
    Regolith,
    /// <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#canyon>.
    Canyon,
    /// Ecotone: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#ecotone>.
    Ecotone,
    /// Fjord: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#fjord>
    Fjord,
    /// Granite: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#granite>
    Granite,
    /// Holocene: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#holocene>
    Holocene,
    /// Isthmus: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/isthmus/overview.md>
    Isthmus,
}
