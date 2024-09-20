use crate::Alloc;
use cuda::{CurrentCtx, DevMem, Stream};

impl<'ctx> Alloc<DevMem<'ctx>> for &'ctx CurrentCtx {
    #[inline]
    fn alloc(&self, size: usize) -> DevMem<'ctx> {
        self.malloc::<u8>(size)
    }

    #[inline]
    fn free(&self, _mem: DevMem<'ctx>) {}
}

impl<'ctx> Alloc<DevMem<'ctx>> for Stream<'ctx> {
    #[inline]
    fn alloc(&self, size: usize) -> DevMem<'ctx> {
        self.malloc::<u8>(size)
    }

    #[inline]
    fn free(&self, mem: DevMem<'ctx>) {
        mem.drop_on(self)
    }
}
