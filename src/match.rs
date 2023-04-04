use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Match(pub usize, pub usize);

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Hash for Match {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // XOR the two hashes together - the same hash is written regardless of order
        let h = calculate_hash(&self.0) ^ calculate_hash(&self.1);
        state.write_u64(h);
    }
}
impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        calculate_hash(self) == calculate_hash(other)
    }
}
impl Eq for Match {}
