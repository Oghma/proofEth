//! Utilities functions

/// First encode index from 1..127 and then 0.
///
/// 0 is encoded as the Nibble("0800") while numbers from 1 to 127 are
/// Nibble("0001")..Nibble("070f"). When adding a leaf, the trie checks that
/// the key we being added is greater than the current one.
pub const fn index_for_rlp(i: usize, len: usize) -> usize {
    if i > 0x7f {
        i
    } else if i == 0x7f || i + 1 == len {
        0
    } else {
        i + 1
    }
}
