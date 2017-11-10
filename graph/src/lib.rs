// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url="https://docs.rs/fera-graph/0.1.0/")]
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

//! Graph data structures and algorithms.

#[cfg(test)]
extern crate itertools;

#[cfg(feature = "quickcheck")]
extern crate quickcheck;

extern crate fera_fun;
extern crate fera_optional;
extern crate fera_unionfind;
extern crate num_traits;
extern crate rand;

#[cfg(test)]
#[macro_use]
pub mod tests;

// basic
#[macro_use]
pub mod builder;

#[macro_use]
pub mod params;

pub mod algs;
pub mod graphs;
pub mod props;
pub mod traverse;

// others
#[cfg(feature = "quickcheck")]
pub mod arbitrary;
pub mod choose;
pub mod ext;
pub mod sets;
pub mod unionfind;

mod fun;
pub use fun::*;

/// The fera graph prelude.
pub mod prelude {
    pub use graphs::adjset::{AdjSetGraph, AdjSetDigraph};
    pub use graphs::complete::{CompleteGraph, CompleteDigraph};
    pub use graphs::static_::{StaticGraph, StaticDigraph};
    pub use graphs::adaptors::{Subgraph, SpanningSubgraph, WithSubgraph};
    pub use graphs::{
        Adjacency,
        AdjacencyDigraph,
        AdjacencyGraph,
        DefaultEdgePropMut,
        DefaultVertexPropMut,
        Digraph,
        Directed,
        Edge,
        EdgeIndexProp,
        EdgeIter,
        EdgeKind,
        EdgeList,
        EdgeTypes,
        Graph,
        GraphItem,
        Incidence,
        IncidenceDigraph,
        IncidenceGraph,
        Mixed,
        OptionEdge,
        OptionVertex,
        Orientation,
        OutEdgeIter,
        OutNeighborIter,
        Undirected,
        UniformEdgeKind,
        Vertex,
        VertexIndexProp,
        VertexIter,
        VertexList,
        VertexTypes,
        WithEdge,
        WithVertex,
    };
    pub use props::{
        BasicEdgeProps,
        BasicProps,
        BasicVertexProps,
        EdgeProp,
        EdgePropGet,
        EdgePropMut,
        EdgePropMutNew,
        PropGet,
        PropIndexMut,
        VertexProp,
        VertexPropGet,
        VertexPropMut,
        VertexPropMutNew,
        WithEdgeIndexProp,
        WithEdgeProp,
        WithVertexIndexProp,
        WithVertexProp,
    };
    pub use builder::{Builder, WithBuilder};
    pub use ext::{GraphsSliceExt, GraphsVecExt};
    pub use fera_optional::Optional;
}
