use crate::BlockDevice;
use crate::BLOCK_SZ;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;

/// Cached block inside memory
pub struct BlockCache {
    /// 缓存的数据,4096 bytes
    cache: [u8; BLOCK_SZ],
    /// underlying block id
    block_id: usize,
    /// 块设备指针,操作缓存数据往硬盘的读写
    block_device: Arc<dyn BlockDevice>,
    /// 数据是否被修改
    modified: bool,
}

impl BlockCache {
    /// 从硬盘加载一块`block_id`数据到缓存
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SZ];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }
    /// 根据`offset`获取对应的地址
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }
    /// 获取对应`offset`地址的引用
    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        // T 必须是一个编译时已知大小的类型
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }
    /// 获取对应`offset`地址的可变引用
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }
    /// 读取对应`offset`的数据,并传入`f`执行获取对应的`V`
    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    /// 读取对应`offset`的可变数据,并传入`f`执行获取对应的`V`
    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }

    /// 把缓存区里的东西写回去.
    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}

/// RAII机制,实现`drop`,从manager中移除后,自动把缓存数据写回硬盘
impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync();
    }
}

/// Use a block cache of 16 blocks
const BLOCK_CACHE_SIZE: usize = 16;

pub struct BlockCacheManager {
    /// 缓存双端队列,item位二元组<block_id, block_cache>
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> BlockCacheManager {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            // 找到id相同的block
            Arc::clone(&pair.1)
        } else {
            // 找不到处理替换情况
            if self.queue.len() == BLOCK_CACHE_SIZE {
                // 如果缓存已经满了,找到强引用为1的
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    // 删了对应idx的block
                    self.queue.drain(idx..=idx);
                } else {
                    // 没有强引用为1的块,报错
                    panic!("Run out of BlockCache!");
                }
            }
            // 从硬盘中加载数据,并压入dequeue中.
            let block_cache = Arc::new(Mutex::new(BlockCache::new(
                block_id,
                Arc::clone(&block_device),
            )));
            self.queue.push_back((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}

/// Get the block cache corresponding to the given block id and block device
/// 根据`block_id`和`block_device`从缓存中找到对应的`block_cache`.
pub fn get_block_cache(
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER
        .lock()
        .get_block_cache(block_id, block_device)
}
/// 把所有的block都写入硬盘
pub fn block_cache_sync_all() {
    let manager = BLOCK_CACHE_MANAGER.lock();
    for (_, cache) in manager.queue.iter() {
        cache.lock().sync();
    }
}
