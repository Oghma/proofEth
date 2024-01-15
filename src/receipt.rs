//! Define a receipt contained in a [Transaction](super::transaction::Transaction)

use alloy_primitives::{Bloom, Log, U256};
use alloy_rlp::{BufMut, Encodable};

/// Receipt of an executed transaction. It contains teh details of it execution.
#[derive(Debug, Default)]
pub struct VerifiedReceipt {
    pub transaction_type: Option<u8>,
    pub status: bool,
    pub cumulative_gas_used: U256,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
}

impl VerifiedReceipt {
    fn payload_length(&self) -> usize {
        let mut len = self.status.length();
        len += self.cumulative_gas_used.length();
        len += self.logs_bloom.length();
        len += self.logs.len();

        len
    }

    pub fn encode(&self, out: &mut dyn BufMut) {
        let payload_length = self.payload_length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length,
        };

        if let Some(tx_type) = self.transaction_type {
            out.put_u8(tx_type);
        }

        header.encode(out);

        self.status.encode(out);
        self.cumulative_gas_used.encode(out);
        self.logs_bloom.encode(out);
        self.logs.encode(out);
    }
}

impl From<&ethers::prelude::TransactionReceipt> for VerifiedReceipt {
    fn from(value: &ethers::prelude::TransactionReceipt) -> Self {
        let logs = value
            .logs
            .iter()
            .map(|log| {
                Log::new(
                    log.address.0.into(),
                    log.topics.iter().map(|topic| topic.0.into()).collect(),
                    log.data.0.clone().into(),
                )
                .unwrap()
            })
            .collect();

        Self {
            transaction_type: value
                .transaction_type
                .map(|tx_type| u8::try_from(tx_type.as_u64()).unwrap()),
            status: value.status.map_or(false, |status| status.0[0] == 1),
            cumulative_gas_used: value.cumulative_gas_used.into(),
            logs,
            logs_bloom: value.logs_bloom.0.into(),
        }
    }
}
