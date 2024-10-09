//! Ethereum specific engine API types and impls.

use reth_chainspec::ChainSpec;
use reth_engine_primitives::EngineTypes;
pub use reth_ethereum_engine_primitives::{EthBuiltPayload, EthPayloadBuilderAttributes};
use reth_payload_primitives::{
    validate_version_specific_fields, EngineApiMessageVersion, EngineObjectValidationError,
    PayloadOrAttributes, PayloadTypes,
};
pub use reth_rpc_types::{
    engine::{
        ExecutionPayloadEnvelopeV2, ExecutionPayloadEnvelopeV3, ExecutionPayloadEnvelopeV4,
        PayloadAttributes as EthPayloadAttributes,
    },
    ExecutionPayloadV1,
};

/// The types used in the default mainnet ethereum beacon consensus engine.
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CwEngineTypes;

impl PayloadTypes for CwEngineTypes {
    type BuiltPayload = EthBuiltPayload;
    type PayloadAttributes = EthPayloadAttributes;
    type PayloadBuilderAttributes = EthPayloadBuilderAttributes;
}

impl EngineTypes for CwEngineTypes {
    type ChainSpec = ChainSpec;

    type ExecutionPayloadV1 = ExecutionPayloadV1;
    type ExecutionPayloadV2 = ExecutionPayloadEnvelopeV2;
    type ExecutionPayloadV3 = ExecutionPayloadEnvelopeV3;
    type ExecutionPayloadV4 = ExecutionPayloadEnvelopeV4;

    fn validate_version_specific_fields(
        chain_spec: &ChainSpec,
        version: EngineApiMessageVersion,
        payload_or_attrs: PayloadOrAttributes<'_, EthPayloadAttributes>,
    ) -> Result<(), EngineObjectValidationError> {
        validate_version_specific_fields(chain_spec, version, payload_or_attrs)
    }
}
