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

pub mod serialize {
    use super::BuildHasher;
    use serde::ser::{Serialize, SerializeMap, Serializer};
    use std::collections::HashMap;

    pub fn serialize<K, V, S>(
        hash_map: &HashMap<K, V, BuildHasher>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Serialize + std::fmt::Debug,
        V: Serialize,
    {
        let mut map = serializer.serialize_map(Some(hash_map.len()))?;
        for (k, v) in hash_map {
            map.serialize_entry(&format!("{k:?}"), v)?;
        }
        map.end()
    }
}
