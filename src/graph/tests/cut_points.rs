use crate::graph::con_comps::connected_components;
use crate::graph::cut_points::{cut_connected_components, CutConComps};
use crate::graph::simple::{NodeIndex, SimpleGraph};
use crate::plat::classic::includes::IntoVecExt;
use crate::utils::rand::Random;

use super::utils::undirected_graph;

#[test]
fn cut_vertices_basic() {
    check_cut_vertices(undirected_graph(1, &[]), vec![]);
    check_cut_vertices(undirected_graph(2, &[(0, 1)]), vec![]);
    check_cut_vertices(undirected_graph(3, &[(0, 1), (1, 2)]), vec![1]);
}

#[test]
fn cut_connected_components_random_small() {
    test_random(5, 300);
}

#[test]
fn cut_connected_components_random_big() {
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
        check_cut_con_comps(undirected_graph(n, edges), cut_points_naive(n, edges));
    }
}

fn cut_points_naive(n: usize, edges: &[(NodeIndex, NodeIndex)]) -> CutConComps {
    let cc = connected_components(&undirected_graph(n, edges)).len();
    let mut cut_cc = Vec::with_capacity(n);
    for v in 0..n {
        let edges = edges
            .iter()
            .filter(|e| e.0 != v && e.1 != v)
            .cloned()
            .into_vec();
        let new_cc = connected_components(&undirected_graph(n, &edges)).len() - 1;
        cut_cc.push(new_cc);
    }
    CutConComps { cc, cut_cc }
}

#[track_caller]
fn check_cut_vertices(g: SimpleGraph, expected: Vec<NodeIndex>) {
    let actual = cut_connected_components(&g).cut_vertices().into_vec();
    assert_eq!(actual, expected, "graph: {g:?}");
}

#[track_caller]
fn check_cut_con_comps(g: SimpleGraph, expected: CutConComps) {
    let actual = cut_connected_components(&g);
    assert_eq!(actual, expected, "graph: {g:?}");
}
