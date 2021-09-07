//! Quickly generate a collection of N hashed values whose computed DHT locations
//! are evenly distributed across the space of u32 values.

use crate::*;
use arbitrary::{Arbitrary, Unstructured};
use kitsune_p2p_dht_arc::DhtLocation;
use serde::{Deserialize, Serialize};

/// Container for fixture data generated by this module
#[derive(Debug, Serialize, Deserialize)]
pub struct HashedFixtures<C: HashableContent> {
    /// The total number of items
    pub num: usize,
    /// The generated items
    pub items: Vec<HoloHashed<C>>,
}

impl<'a, T, C> HashedFixtures<C>
where
    C: Clone + HashableContent<HashType = T> + Arbitrary<'a>,
    T: HashTypeSync,
{
    /// Quickly generate a collection of `num` hashed values whose computed
    /// DHT locations are evenly distributed across the space of u32 values.
    /// Specifically, there is only one hash per interval of size `2^32 / num`
    pub fn generate<F: Fn(&HoloHashed<C>) -> DhtLocation>(
        num: usize,
        u: &mut Unstructured<'a>,
        relevant_location: F,
    ) -> Self {
        let mut tot = 0;
        let mut items = vec![None; num];
        while tot < num {
            let content = C::arbitrary(u).unwrap();
            let item = HoloHashed::from_content_sync(content);
            let loc = relevant_location(&item).to_u32();
            let idx = loc / (u32::MAX / num as u32);

            match &mut items[idx as usize] {
                Some(_) => (),
                h @ None => {
                    *h = Some(item);
                    tot += 1;
                }
            }
        }
        assert!(items.iter().all(|h| h.is_some()));
        let items = items.into_iter().flatten().collect(); //into_iter().map(|i| i.unwrap()).collect();
        Self { num, items }
    }

    /// Get the item at the specified "bucket".
    /// There are `self.num` buckets, and the index can be a negative number,
    /// which will be counted backwards from `num`.
    pub fn get(&self, i: i32) -> &HoloHashed<C> {
        &self.items[self.rectify_index(i)]
    }

    /// Get the endpoints for the bucket at the specified index
    pub fn bucket(&self, i: i32) -> (DhtLocation, DhtLocation) {
        let bucket_size = u32::MAX / self.num as u32;
        let start = self.rectify_index(i) as u32 * bucket_size;
        (
            DhtLocation::new(start),
            DhtLocation::new(start + bucket_size),
        )
    }

    fn rectify_index(&self, i: i32) -> usize {
        rectify_index(self.num, i)
    }
}

/// Map a signed index into an unsigned index
pub fn rectify_index(num: usize, i: i32) -> usize {
    let num = num as i32;
    if i >= num || i <= -num {
        panic!(
            "attempted to rectify an out-of-bounds index: |{}| >= {}",
            i, num
        );
    }
    if i < 0 {
        (num + i) as usize
    } else {
        i as usize
    }
}