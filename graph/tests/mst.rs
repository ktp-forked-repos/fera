extern crate fera_graph;
extern crate fera_fun;

#[macro_use]
extern crate quickcheck;

use fera_fun::vec;
use fera_graph::prelude::*;
use fera_graph::algs::{Kruskal, Prim, Trees};
use fera_graph::arbitrary::GnConnectedWithEdgeProp;

quickcheck! {
    fn mst(x: GnConnectedWithEdgeProp<StaticGraph, u32>) -> bool {
        let GnConnectedWithEdgeProp(g, w) = x;
        if g.num_vertices() == 0 || g.num_edges() == 0 {
            return true;
        }
        let prim = vec(g.prim(&w));
        let w_prim: u32 = sum_prop(&w, &prim);
        let kruskal = vec(g.kruskal_mst(&w));
        let w_kruskal: u32 = sum_prop(&w, &kruskal);
        assert!(g.spanning_subgraph(&prim).is_tree());
        assert!(g.spanning_subgraph(&kruskal).is_tree());
        assert_eq!(w_prim, w_kruskal);
        true
    }
}