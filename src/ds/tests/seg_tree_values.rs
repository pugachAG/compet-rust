use crate::ds::seg_tree_values::{
    seg_tree_value_max, seg_tree_value_min, seg_tree_value_sum, simple_seg_tree_value,
};

#[test]
fn seg_tree_value_macros() {
    simple_seg_tree_value!(ElementXor, u64, l, r, l ^ r, 0);
    seg_tree_value_sum!(ElementSumInt, u64);
    seg_tree_value_sum!(ElementSumFloat, f64, 0.0);
    seg_tree_value_min!(ElementMinInt, u64);
    seg_tree_value_min!(ElementMinTuple, (i32, i32), (i32::MAX, i32::MAX));
    seg_tree_value_max!(ElementMaxInt, u64);
    seg_tree_value_max!(ElementMaxTuple, (u64, u64), (0, 0));
}
