use crate::math::modulo::Modulo;

/*
Usage:
type Mod = super::definitions::Modulo998244353;
type Mod = super::definitions::Modulo10Pow9Plus7;
*/

pub const NUM_10_POW_9_PLUS_7: u64 = 10u64.pow(9) + 7;
pub type Modulo10Pow9Plus7 = Modulo<NUM_10_POW_9_PLUS_7>;
pub const NUM_998244353: u64 = 998244353;
pub type Modulo998244353 = Modulo<NUM_998244353>;
