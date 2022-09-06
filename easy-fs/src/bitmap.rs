use crate::block_cache::get_block_cache;
use crate::block_dev::BlockDevice;
use crate::BLOCK_SZ;
use alloc::sync::Arc;

/// A bitmap block 位图由一个长度为64的u64数组来储存数据,正好是一个block的大小
type BitmapBlock = [u64; 64];

/// Number of bits in a block
const BLOCK_BITS: usize = BLOCK_SZ * 8;

/// A bitmap 位图对象
pub struct Bitmap {
    /// 此bitmap 0 号位置的`block_id`
    start_block_id: usize,
    blocks: usize,
}

/// Decompose bits into (block_pos, bits64_pos, inner_pos)<br/>
/// bits = block_pos * BLOCK_BITS + bits64_pos * 64 + inner_pos
fn decomposition(mut bit: usize) -> (usize, usize, usize) {
    let block_pos = bit / BLOCK_BITS;
    bit %= BLOCK_BITS;
    (block_pos, bit / 64, bit % 64)
}

impl Bitmap {
    /// A new bitmap from start block id and number of blocks
    pub fn new(start_block_id: usize, blocks: usize) -> Bitmap {
        Self {
            start_block_id,
            blocks,
        }
    }
    /// Allocate a new block from a block device<br/>
    /// 从硬盘申请一块新的block, 返回分配的bit所在的位置，等同于索引节点/数据块的编号
    pub fn alloc(&self, block_device: &Arc<dyn BlockDevice>) -> Option<usize> {
        for block_id in 0..self.blocks {
            // 遍历所有的blocks
            let pos = get_block_cache(
                block_id + self.start_block_id as usize,
                Arc::clone(block_device),
            )
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                if let Some((bits64_pos, inner_pos)) = bitmap_block
                    .iter()
                    .enumerate()
                    // 遍历`bitmap`数组,找到不为u64::MAX的值
                    .find(|(_, bits64)| **bits64 != u64::MAX)
                    // 从u64中找到最低的一个 0 并置为 1(`trailing_ones`方法实现)
                    .map(|(bits64_pos, bits64)| (bits64_pos, bits64.trailing_ones() as usize))
                {
                    // modify cache
                    bitmap_block[bits64_pos] |= 1u64 << inner_pos;
                    Some(block_id * BLOCK_BITS + bits64_pos * 64 + inner_pos as usize)
                } else {
                    None
                }
            });
            if pos.is_some() {
                return pos;
            }
        }
        None
    }
    /// Deallocate a block<br/>
    /// 从哪里来,塞回哪里去
    pub fn dealloc(&self, block_device: &Arc<dyn BlockDevice>, bit: usize) {
        let (block_pos, bits64_pos, inner_pos) = decomposition(bit);
        get_block_cache(block_pos + self.start_block_id, Arc::clone(block_device))
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                assert!(bitmap_block[bits64_pos] & (1u64 << inner_pos) > 0);
                bitmap_block[bits64_pos] -= 1u64 << inner_pos; // 找到对应的bit,置0
            });
    }
    /// Get the max number of allocatable blocks
    pub fn maximum(&self) -> usize {
        self.blocks * BLOCK_BITS
    }
}
