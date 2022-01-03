use crate::op::OpHash;

pub type Hash32 = [u8; 32];

pub fn fake_hash() -> Hash32 {
    use rand::distributions::*;

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(u8::MIN..=u8::MAX);
    let bytes: Vec<u8> = uni.sample_iter(&mut rng).take(32).collect();
    let bytes: [u8; 32] = bytes.try_into().unwrap();
    bytes
}

pub fn array_xor<const N: usize>(a: &mut [u8; N], b: &[u8; N]) {
    for i in 0..N {
        a[i] ^= b[i];
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegionHash([u8; 32]);

impl RegionHash {
    /// Any null node hashes just get ignored.
    pub fn xor(&mut self, other: &Self) {
        array_xor(&mut self.0, &other.0);
    }
}

impl std::ops::Add for RegionHash {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        Self::xor(&mut self, &rhs);
        self
    }
}

impl num_traits::Zero for RegionHash {
    fn zero() -> Self {
        Self([0; 32])
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl From<OpHash> for RegionHash {
    fn from(h: OpHash) -> Self {
        Self(h.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RegionData {
    pub hash: RegionHash,
    pub size: u32,
    pub count: u32,
}

impl RegionData {
    pub const MASS: usize = std::mem::size_of::<RegionData>();
}

impl num_traits::Zero for RegionData {
    fn zero() -> Self {
        Self {
            hash: RegionHash::zero(),
            size: 0,
            count: 0,
        }
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl std::ops::AddAssign for RegionData {
    fn add_assign(&mut self, other: Self) {
        self.hash.xor(&other.hash);
        self.size += other.size;
        self.count += other.count;
    }
}

impl std::ops::Add for RegionData {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl std::ops::SubAssign for RegionData {
    fn sub_assign(&mut self, other: Self) {
        // XOR works as both addition and subtraction
        self.hash.xor(&other.hash);
        self.size -= other.size;
        self.count -= other.count;
    }
}

impl std::ops::Sub for RegionData {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}