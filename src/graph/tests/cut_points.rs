use crate::graph::con_comps::connected_components;
use crate::graph::cut_points::cut_points;
use crate::graph::simple::{NodeIndex, SimpleGraph};
use crate::plat::classic::includes::{IntoVecExt, SliceSortedExt};
use crate::utils::rand::Random;

use super::utils::undirected_graph;

#[test]
fn cut_points_basic() {
    check_cut_points(undirected_graph(1, &[]), vec![]);
    check_cut_points(undirected_graph(2, &[(0, 1)]), vec![]);
    check_cut_points(undirected_graph(3, &[(0, 1), (1, 2)]), vec![1]);
}

#[test]
fn cut_points_random_small() {
    test_random(5, 300);
}

#[test]
fn cut_points_random_big() {
    test_random(10, 100);
}

#[track_caller]
fn test_random(n: usize, iters: usize) {
    let mut rand = Random::new(42);
    let mut all_edges = (0..n).flat_map(|v| (0..v).map(move |u| (v, u))).into_vec();
    for _ in 0..iters {
        rand.shuffle(&mut all_edges);
        let m = rand.gen_range(0..=all_edges.len());
        let edges = &all_edges[0..m];
        check_cut_points(undirected_graph(n, edges), cut_points_naive(n, edges));
    }
}

fn cut_points_naive(n: usize, edges: &[(NodeIndex, NodeIndex)]) -> Vec<NodeIndex> {
    let mut ret = Vec::new();
    let cc = connected_components(&undirected_graph(n, edges)).len();
    for v in 0..n {
        let edges = edges
            .iter()
            .filter(|e| e.0 != v && e.1 != v)
            .cloned()
            .into_vec();
        let new_cc = connected_components(&undirected_graph(n, &edges)).len() - 1;
        if new_cc > cc {
            ret.push(v);
        }
    }
    ret
}

#[track_caller]
fn check_cut_points(g: SimpleGraph, expected: Vec<NodeIndex>) {
    let actual = cut_points(&g);
    assert_eq!(actual.sorted(), expected.sorted(), "graph: {g:?}");
}
