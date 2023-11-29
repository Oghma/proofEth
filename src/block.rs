//! A block representing an Ethereum block
use alloy_primitives::{keccak256, Address, BlockHash, Bloom, Bytes, B256, B64, U256, U64};
use alloy_rlp::{Encodable, RlpDecodable, RlpEncodable};

/// Ethereum block hader
#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct BlockHeader {
    pub parent: BlockHash,
    pub uncles_hash: BlockHash,
    pub miner: Address,
    pub state_root: B256,
    pub transaction_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: U64,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub mix_hash: B256,
    pub nonce: B64,
    pub base_fee_per_gas: U256,
    pub withdrawals_root: B256,
}

#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct Block {
    pub hash: BlockHash,
    pub header: BlockHeader,
}

impl Block {
    pub fn new(header: BlockHeader) -> Self {
        let mut buffer = Vec::<u8>::new();
        header.encode(&mut buffer);
        let hash = keccak256(buffer);

        Self { header, hash }
    }

    /// Check if the block hash is correct
    pub fn verify_block_hash(&self, hash: &BlockHash) -> bool {
        &self.hash == hash
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, uint};

    use super::*;

    #[test]
    fn should_hash_correct() {
        let header= BlockHeader {
            parent: "0x9e8dd74d00937fddbbf465cb828acbdb9af2514a6e9d633589f5e4a047dfec5b".parse().unwrap(),
            uncles_hash: "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".parse().unwrap(),
            miner: address!("95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5"),
            state_root: "0xf7f5ceaac85a1ecd7e0c74f6af0cc2d2a88aca9ab9e356c12d1670322ec7fbdd".parse().unwrap(),
            transaction_root: "0x541e0fa363e67d568a0c99bf0b9c0f5cf6a268137072d33a4bca36d784542007".parse().unwrap(),
            receipts_root:"0xda79a01eae58b7437540a7647a6e8c1d26109bc5d985e6ba315cf7637c785d41".parse().unwrap(),
            logs_bloom: "0xa8b0050247c27195101a00008040cea31c210a20908a52153201006004103c730c04a509281890a083690d621000c1884319910a893124c987400e4886328c22bc9281d8202c0a092c954029e4546aa990230815045e2804a0101470cba8144050100100b16cc2998c98438800263cc424182a7280031700172414f6c18a800c4812048003000144005b046ad900800e4b055205116480c82401404276151569120c0849013820616a1020c018821cb880c1540200618d0200380e9a041830181ea4340a029018406000a00000f110e0040c13278503009058406f0e800122462454a62029c1806ca24410700800088480819820009600430008809108052005".parse().unwrap(),
            difficulty:uint!(0_U256),
            number:uint!(18677559_U64),
            gas_limit:uint!(30000000_U256),
            gas_used:uint!(11754067_U256),
            timestamp:uint!(1701264383_U256),
            extra_data:"0x6265617665726275696c642e6f7267".parse().unwrap(),
            mix_hash:"0xf380df736ba8959509e0214cdf0862db0f45731d950789a2780a821faabc15a8".parse().unwrap(),
            nonce: "0x0000000000000000".parse().unwrap(),
            base_fee_per_gas: uint!(41014545799_U256),
            withdrawals_root: "0x89b1b0500a08b49ec6f538aedb39aab1c384874bff882edc4560e76c76ef3f05".parse().unwrap()
        };
        let block = Block::new(header);

        assert!(block.verify_block_hash(
            "0x8c07fbc176e8cd1b0ea49dc56132e6e571d0c94ef0b88907658c7d197c4a9dfc"
                .parse()
                .as_ref()
                .unwrap()
        ))
    }
}
