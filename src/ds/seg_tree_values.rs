#![allow(unused_imports)]
#![allow(unused_macros)]
/// Usage example: `seg_tree_value_macros` test

macro_rules! simple_seg_tree_value {
    ($name:ident, $type:ty, $l:ident, $r:ident, $op:expr, $e:expr) => {
        #[derive(Copy, Clone)]
        struct $name($type);

        impl crate::ds::seg_tree::SegTreeValue for $name {
            fn op(l: Self, r: Self) -> Self {
                let $l = l.0;
                let $r = r.0;
                Self($op)
            }

            fn e() -> Self {
                Self($e)
            }
        }
    };
}

pub(crate) use simple_seg_tree_value;

macro_rules! seg_tree_value_sum {
    ($name:ident, $type:ty) => {
        $crate::ds::seg_tree_values::seg_tree_value_sum!($name, $type, 0);
    };
    ($name:ident, $type:ty, $e:expr) => {
        $crate::ds::seg_tree_values::simple_seg_tree_value!($name, $type, l, r, l + r, $e);
    };
}

pub(crate) use seg_tree_value_sum;

macro_rules! seg_tree_value_min {
    ($name:ident, $type:ident) => {
        $crate::ds::seg_tree_values::seg_tree_value_min!($name, $type, $type::MAX);
    };
    ($name:ident, $type:ty, $e:expr) => {
        $crate::ds::seg_tree_values::simple_seg_tree_value!(
            $name,
            $type,
            l,
            r,
            std::cmp::min(l, r),
            $e
        );
    };
}

pub(crate) use seg_tree_value_min;

macro_rules! seg_tree_value_max {
    ($name:ident, $type:ident) => {
        $crate::ds::seg_tree_values::seg_tree_value_max!($name, $type, $type::MIN);
    };
    ($name:ident, $type:ty, $e:expr) => {
        $crate::ds::seg_tree_values::simple_seg_tree_value!(
            $name,
            $type,
            l,
            r,
            std::cmp::max(l, r),
            $e
        );
    };
}

pub(crate) use seg_tree_value_max;
