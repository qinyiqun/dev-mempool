#[deny(warnings)]

pub trait Alloc<M> {
    fn alloc(&self, size: usize) -> M;
    fn free(&self, mem: M);
}

pub trait MemPool<M>: Alloc<M> {
    fn snapshot(&self) -> Snapshot;
    fn reserve(&self, size: usize);
}

pub struct Snapshot {
    pub n_seg: usize,
    pub n_seg_free: usize,
    pub n_seg_peak: usize,
    pub n_byte: usize,
    pub n_byte_free: usize,
    pub n_byte_peak: usize,
}

#[cfg(common_cpu)]
mod common_cpu;
#[cfg(common_cpu)]
pub use common_cpu::{Blob, ThisThread};

#[cfg(nvidia_gpu)]
mod nvidia_gpu;
#[cfg(nvidia_gpu)]
pub extern crate cuda;
