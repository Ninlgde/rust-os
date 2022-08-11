//! 地址转换相关操作

use core::fmt;
use core::fmt::{Debug, Formatter};
use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use crate::mm::page_table::PageTableEntry;
pub use crate::util::range::{SimpleRange, StepByOne};

/// 物理地址实际宽度
const PA_WIDTH_SV39: usize = 56;
/// 虚拟地址实际宽度
const VA_WIDTH_SV39: usize = 39;

/// 物理页编号的宽度 44 = 56-12
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
/// 虚拟页编号的宽度 27 = 39-12
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

/// 物理地址
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

/// 虚拟地址
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

/// 物理页号 frame
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

/// 虚拟页号
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

/// Debugging
impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

/// 类型转换方法 usize -> PA
impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self { Self(v & ((1 << PA_WIDTH_SV39) - 1)) }
}

/// 类型转换方法 usize -> PPN
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self { Self(v & ((1 << PPN_WIDTH_SV39) - 1)) }
}

/// 类型转换方法 PA -> usize
impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self { v.0 }
}

/// 类型转换方法 PPN -> usize
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self { v.0 }
}

impl PhysAddr {
    /// 获取物理地址实际偏移量
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    /// 通过PA计算PPN下界
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    /// 通过PA计算PPN 上界
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
    /// 是否对齐
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
    ///Get mutable reference to `PhysAddr` value
    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.0 as *mut T).as_mut().unwrap() }
    }
}

/// 类型转换方法 usize -> VA
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self { Self(v & ((1 << VA_WIDTH_SV39) - 1)) }
}

/// 类型转换方法 usize -> VPN
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self { Self(v & ((1 << VPN_WIDTH_SV39) - 1)) }
}

/// 类型转换方法 VA -> usize
impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        if v.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            v.0 | (!((1 << VA_WIDTH_SV39) - 1))
        } else {
            v.0
        }
    }
}

/// 类型转换方法 VPN -> usize
impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self { v.0 }
}

impl VirtAddr {
    /// 获取虚拟地址实际偏移量
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    /// 通过VA计算VPN下界
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    /// 通过VA计算VPN 上界
    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
    /// 是否对齐
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

/// 类型转换方法 PA -> PPN
impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

/// 类型转换方法 PPN -> PA
impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

/// 类型转换方法 VA -> VPN
impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysPageNum {
    /// 获取物理页可变字节数组
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096)
        }
    }
    /// 将ppn转换为可变pte列表,并获取
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }
    /// 获取可变ppn as T
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = (*self).into();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl VirtPageNum {
    /// 获取VPN0,1,3 每个9bits
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

/// a simple range structure for virtual page number
pub type VPNRange = SimpleRange<VirtPageNum>;