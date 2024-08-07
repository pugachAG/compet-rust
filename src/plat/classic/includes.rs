#![allow(unused_imports)]
pub use super::io::Io;
pub use crate::types::str::Str;
pub use crate::utils::collections::{
    def_vec, def_vec2, IntoMapExt, IntoSetExt, IntoVecExt, SliceMinMaxExt, SliceReversedExt,
    SliceSortedByKeyExt, SliceSortedExt, SliceSumExt, Vec2,
};
pub use crate::{exec_mode, input, output};
pub use std::cmp::{max, min, Reverse};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
pub use std::iter::FromIterator;
