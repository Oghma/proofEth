//! Different transaction types in Ethereum

use alloy_primitives::{Address, Bytes, ChainId, B256, U256};
use alloy_rlp::{BufMut, Encodable, RlpDecodable, RlpEncodable};

pub enum Transaction {
    Legacy(TxLegacy),
    Eip2930(Tx2930),
    Eip1559(Tx1559),
}

pub struct TxLegacy {
    pub chain_id: ChainId,
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub signature: Signature,
}

impl TxLegacy {
    fn payload_length(&self) -> usize {
        let mut len = self.nonce.length();
        len += self.gas_price.length();
        len += self.gas_limit.length();
        len += self.to.length();
        len += self.value.length();
        len += self.data.length();
        len += self.signature.v.length();
        len += self.signature.r.length();
        len += self.signature.s.length();

        len
    }

    pub fn encode(&self, out: &mut dyn BufMut) {
        let payload_length = self.payload_length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length,
        };

        header.encode(out);

        self.nonce.encode(out);
        self.gas_price.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.data.0.encode(out);
        self.signature.encode(out);
    }
}

pub struct Tx2930 {
    pub tx_type: u8,
    pub chain_id: ChainId,
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub signature: Signature,
    pub access_list: Vec<AccessListItem>,
}

impl Tx2930 {
    fn payload_length(&self) -> usize {
        let mut len = self.chain_id.length();
        len += self.nonce.length();
        len += self.gas_price.length();
        len += self.gas_limit.length();
        len += self.to.length();
        len += self.value.length();
        len += self.data.length();
        len += self.access_list.length();
        len += self.signature.length();

        len
    }

    pub fn encode(&self, out: &mut dyn BufMut) {
        let payload_length = self.payload_length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length,
        };

        out.put_u8(self.tx_type);
        header.encode(out);

        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.gas_price.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.data.0.encode(out);
        self.access_list.encode(out);
        self.signature.encode(out);
    }
}

pub struct Tx1559 {
    pub tx_type: u8,
    pub chain_id: ChainId,
    pub nonce: u64,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub signature: Signature,
    pub access_list: Vec<AccessListItem>,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
}

impl Tx1559 {
    fn payload_length(&self) -> usize {
        let mut len = self.chain_id.length();
        len += self.nonce.length();
        len += self.max_priority_fee_per_gas.length();
        len += self.max_fee_per_gas.length();
        len += self.gas_limit.length();
        len += self.to.length();
        len += self.value.length();
        len += self.data.length();
        len += self.access_list.length();
        len += self.signature.v.length();
        len += self.signature.r.length();
        len += self.signature.s.length();

        len
    }

    pub fn encode(&self, out: &mut dyn BufMut) {
        let payload_length = self.payload_length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length,
        };

        out.put_u8(self.tx_type);
        header.encode(out);

        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.max_priority_fee_per_gas.encode(out);
        self.max_fee_per_gas.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.data.0.encode(out);
        self.access_list.encode(out);
        self.signature.encode(out);
    }
}

#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct Signature {
    pub v: U256,
    pub r: U256,
    pub s: U256,
}

impl Signature {
    pub fn encode(&self, out: &mut dyn BufMut) {
        self.v.encode(out);
        self.r.encode(out);
        self.s.encode(out);
    }
}

#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct AccessListItem {
    pub address: Address,
    pub storage_key: Vec<B256>,
}
