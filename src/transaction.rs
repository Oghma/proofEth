//! Different transaction types in Ethereum

use alloy_primitives::{Address, Bytes, ChainId, B256, U256, U64};
use alloy_rlp::{BufMut, Encodable, RlpDecodable, RlpEncodable};
use ethers::types::{TransactionReceipt, U64 as EU64};

use crate::receipt::VerifiedReceipt;

#[derive(Debug)]
pub enum VerifiedTransaction {
    Legacy(TxLegacy),
    Eip2930(Tx2930),
    Eip1559(Tx1559),
}

impl VerifiedTransaction {
    pub fn new(transaction: &ethers::types::Transaction, receipt: &TransactionReceipt) -> Self {
        match transaction.transaction_type {
            Some(EU64([0])) => {
                let txn = TxLegacy {
                    nonce: transaction.nonce.as_u64(),
                    gas_price: transaction.gas_price.unwrap().as_u128(),
                    gas_limit: transaction.gas.as_u64(),
                    to: Address::from(transaction.to.unwrap().0),
                    value: transaction.value.into(),
                    data: Bytes::from(transaction.input.0.clone()),
                    signature: Signature {
                        v: U256::from(U64::from_limbs(transaction.v.0)),
                        r: transaction.r.into(),
                        s: transaction.s.into(),
                    },
                    receipt: VerifiedReceipt::from(receipt),
                };
                VerifiedTransaction::Legacy(txn)
            }
            Some(EU64([1])) => {
                let access_list: Option<Vec<AccessListItem>> =
                    transaction.access_list.clone().map(|list| {
                        list.0
                            .iter()
                            .map(|item| AccessListItem {
                                address: Address::from(item.address.0),
                                storage_key: item
                                    .storage_keys
                                    .iter()
                                    .map(|key| key.0.into())
                                    .collect(),
                            })
                            .collect()
                    });

                let txn = Tx2930 {
                    tx_type: 1,
                    chain_id: transaction.chain_id.unwrap().as_u64(),
                    nonce: transaction.nonce.as_u64(),
                    gas_price: transaction.gas_price.unwrap().as_u128(),
                    gas_limit: transaction.gas.as_u64(),
                    to: Address::from(transaction.to.unwrap().0),
                    value: transaction.value.into(),
                    data: Bytes::from(transaction.input.0.clone()),
                    access_list: access_list.unwrap(),
                    signature: Signature {
                        v: U256::from(U64::from_limbs(transaction.v.0)),
                        r: transaction.r.into(),
                        s: transaction.s.into(),
                    },
                    receipt: VerifiedReceipt::from(receipt),
                };
                VerifiedTransaction::Eip2930(txn)
            }
            Some(EU64([2])) => {
                let access_list: Option<Vec<AccessListItem>> =
                    transaction.access_list.clone().map(|list| {
                        list.0
                            .iter()
                            .map(|item| AccessListItem {
                                address: Address::from(item.address.0),
                                storage_key: item
                                    .storage_keys
                                    .iter()
                                    .map(|key| key.0.into())
                                    .collect(),
                            })
                            .collect()
                    });

                let txn = Tx1559 {
                    tx_type: 2,
                    chain_id: transaction.chain_id.unwrap().as_u64(),
                    nonce: transaction.nonce.as_u64(),
                    gas_limit: transaction.gas.as_u64(),
                    to: Address::from(transaction.to.unwrap().0),
                    value: transaction.value.into(),
                    data: Bytes::from(transaction.input.0.clone()),
                    access_list: access_list.unwrap(),
                    max_fee_per_gas: transaction.max_fee_per_gas.unwrap().as_u128(),
                    max_priority_fee_per_gas: transaction
                        .max_priority_fee_per_gas
                        .unwrap()
                        .as_u128(),
                    signature: Signature {
                        v: U256::from(U64::from_limbs(transaction.v.0)),
                        r: transaction.r.into(),
                        s: transaction.s.into(),
                    },
                    receipt: VerifiedReceipt::from(receipt),
                };
                VerifiedTransaction::Eip1559(txn)
            }
            _ => panic!("Unknown transaction type"),
        }
    }

    pub fn encode(&self, out: &mut dyn BufMut) {
        match self {
            Self::Legacy(txn) => txn.encode(out),
            Self::Eip1559(txn) => txn.encode(out),
            Self::Eip2930(txn) => txn.encode(out),
        }
    }

    pub fn receipt(&self) -> &VerifiedReceipt {
        match self {
            Self::Legacy(txn) => &txn.receipt,
            Self::Eip1559(txn) => &txn.receipt,
            Self::Eip2930(txn) => &txn.receipt,
        }
    }
}

#[derive(Debug)]
pub struct TxLegacy {
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub signature: Signature,
    pub receipt: VerifiedReceipt,
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

#[derive(Debug)]
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
    pub receipt: VerifiedReceipt,
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

#[derive(Debug)]
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
    pub receipt: VerifiedReceipt,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_primitives::{address, keccak256, uint, BlockHash};

    use super::*;

    #[test]
    fn should_legacy_hash_correctly() {
        let txn = TxLegacy {
            nonce: 1752,
            gas_price: 300000000000,
            gas_limit: 90277,
            to: address!("1643E812aE58766192Cf7D2Cf9567dF2C37e9B7F"),
            value: uint!(3000000000000000000_U256),
            data: "0xa1903eab0000000000000000000000000000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            signature: Signature {
                v: uint!(45_U256),
                r: "0xb1df344bc5f8d4508b03bc24e73b8a411e6662152fc083bc044e59826cae3421"
                    .parse()
                    .unwrap(),
                s: "0x08d15757b321670c81ad46e61eaa7c58279559af972d048648cfc40ba8ff4133"
                    .parse()
                    .unwrap(),
            },
            receipt: VerifiedReceipt::default(),
        };

        let mut buffer = Vec::<u8>::new();
        txn.encode(&mut buffer);

        assert_eq!(
            keccak256(buffer),
            BlockHash::from_str(
                "0x2dd5d1a058f69df4c374081e0d6be639c65f8b39967d4ea8dc62ec77b4cca1d5"
            )
            .unwrap()
        );
    }

    #[test]
    fn should_type1_hash_correctly() {
        let txn = Tx2930 {
            tx_type: 1,
            chain_id: 1,
            nonce: 160466,
            gas_limit: 230684,
            gas_price: 41014545799,
            to: address!("A69babEF1cA67A37Ffaf7a485DfFF3382056e78C"),
            value: uint!(11846912_U256),
            data: "0x78e111f60000000000000000000000002d876e69e7017421b77822b1bb4c8da1307a19700000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000014470aa0dfe000000000000000000000000e45b4a84e0ad24b8617a489d743c52b84b7acebe0000000000000000000000005b7533812759b45c2b44c19e320ba2cd2681b542000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000002c6b50bca00000000000000000000000000000000000000000000000000006c72001c8d6e00000000000000000000000000000000000000000000000001a5ce878dc1dc50000000000000000000000000000000000000000000013633fa3aece210000000000000000000000000000000000000000000000000013633fa3aece2100000000000000000000000000000000000000000000000000000000000000065673bffff0000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            signature: Signature {
                v: uint!(1_U256),
                r: "0xbed4a3918a4478c26dc5cec677fe21dc3599f2597e2b8ff0320c141a1d5213c8"
                    .parse()
                    .unwrap(),
                s: "0x55a57e4e2904c8268c698357e1c77789af9d484122ace41bb69add4f3bc697c0"
                    .parse()
                    .unwrap(),
            },
            access_list: Vec::new(),
            receipt: VerifiedReceipt::default()
        };

        let mut buffer = Vec::<u8>::new();
        txn.encode(&mut buffer);

        assert_eq!(
            keccak256(buffer),
            BlockHash::from_str(
                "0x6fa053fe85c3bbda94b727f7a085196222bd80429325b49481b518865ff0fe9f"
            )
            .unwrap()
        );
    }

    #[test]
    fn should_type2_hash_correctly() {
        let txn = Tx1559 {
            tx_type: 2,
            chain_id: 1,
            nonce: 160466,
            gas_limit: 230684,
            to: address!("A69babEF1cA67A37Ffaf7a485DfFF3382056e78C"),
            value: uint!(11846912_U256),
            data: "0x78e111f60000000000000000000000002d876e69e7017421b77822b1bb4c8da1307a19700000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000014470aa0dfe000000000000000000000000e45b4a84e0ad24b8617a489d743c52b84b7acebe0000000000000000000000005b7533812759b45c2b44c19e320ba2cd2681b542000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000002c6b50bca00000000000000000000000000000000000000000000000000006c72001c8d6e00000000000000000000000000000000000000000000000001a5ce878dc1dc50000000000000000000000000000000000000000000013633fa3aece210000000000000000000000000000000000000000000000000013633fa3aece2100000000000000000000000000000000000000000000000000000000000000065673bffff0000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            signature: Signature {
                v: uint!(1_U256),
                r: "0xbed4a3918a4478c26dc5cec677fe21dc3599f2597e2b8ff0320c141a1d5213c8"
                    .parse()
                    .unwrap(),
                s: "0x55a57e4e2904c8268c698357e1c77789af9d484122ace41bb69add4f3bc697c0"
                    .parse()
                    .unwrap(),
            },
            access_list: Vec::new(),
            max_fee_per_gas: 61521818698,
            max_priority_fee_per_gas: 0,
            receipt: VerifiedReceipt::default()
        };

        let mut buffer = Vec::<u8>::new();
        txn.encode(&mut buffer);

        assert_eq!(
            keccak256(buffer),
            BlockHash::from_str(
                "0xd6792b3f289f49876449d68af4706cc8cbfa5a9b480fdb7e5e0fa1fd79374348"
            )
            .unwrap()
        );
    }
}
