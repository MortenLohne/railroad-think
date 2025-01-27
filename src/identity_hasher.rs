#[derive(Default)]
pub struct IdentityHasher(u64);
use std::hash::Hasher;

/// Identity hasher
impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("IdentityHasher only supports u32 keys")
    }

    fn write_u32(&mut self, i: u32) {
        self.0 = u64::from(i);
    }
}

#[derive(Clone, Default)]
pub struct BuildHasher;
impl std::hash::BuildHasher for BuildHasher {
    type Hasher = IdentityHasher;
    fn build_hasher(&self) -> IdentityHasher {
        IdentityHasher::default()
    }
}
