use graph::*;
use ds::{IteratorExt, VecExt};
use builder::{Builder, WithBuilder};
use choose::Choose;

use std::iter::{Cloned, Map};
use std::ops::{Deref, Index, IndexMut, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::Debug;

use rand::Rng;

pub type StaticGraph = StaticGraphGeneric<u32, usize>;

pub trait Num: 'static + Eq + Copy + Clone + Debug + Hash +
               traits::OptionItem<StaticVertex<Self>> +
               traits::OptionItem<StaticEdge<Self>> {
    type Range: Iterator<Item = Self>;
    fn range(a: usize, b: usize) -> Self::Range;
    fn to_usize(self) -> usize;
    fn from_usize(v: usize) -> Self;
    fn is_valid(v: usize) -> bool;
    fn max() -> Self;
}

macro_rules! impl_num {
    ($t: ident) => (
        impl Num for $t {
            type Range = Range<$t>;

            #[inline(always)]
            fn range(a: usize, b: usize) -> Self::Range {
                Self::from_usize(a) .. Self::from_usize(b)
            }

            #[inline(always)]
            fn to_usize(self) -> usize {
                self as usize
            }

            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                v as Self
            }

            #[inline(always)]
            fn is_valid(v: usize) -> bool {
                (v as u64) < (Self::max() as u64)
            }

            #[inline(always)]
            fn max() -> Self {
                use std;
                std::$t::MAX
            }
        }

        impl traits::OptionItem<StaticVertex<$t>> for $t {
            #[inline(always)]
            fn to_option(&self) -> Option<Self> {
                if <$t as traits::OptionItem<StaticVertex<$t>>>::is_none(self) {
                    None
                } else {
                    Some(*self)
                }
            }

            #[inline(always)]
            fn is_none(&self) -> bool {
                *self == Self::max()
            }

            #[inline(always)]
            fn is_some(&self) -> bool {
                *self != Self::max()
            }

            #[inline(always)]
            fn eq_some(&self, other: $t) -> bool {
                *self == other
            }
        }

        impl traits::OptionItem<StaticEdge<$t>> for $t {
            #[inline(always)]
            fn to_option(&self) -> Option<StaticEdge<$t>> {
                if <$t as traits::OptionItem<StaticEdge<$t>>>::is_none(self) {
                    None
                } else {
                    Some(StaticEdge(*self))
                }
            }

            #[inline(always)]
            fn is_none(&self) -> bool {
                *self == Self::max()
            }

            #[inline(always)]
            fn is_some(&self) -> bool {
                *self != Self::max()
            }

            #[inline(always)]
            fn eq_some(&self, other: StaticEdge<$t>) -> bool {
                StaticEdge(*self) == other
            }
        }
    )
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(usize);


// StaticEdge

#[derive(Copy, Clone, Debug, Eq)]
pub struct StaticEdge<N: Num>(N);

// TODO: Document the representation of StaticEdge
impl<N: Num> StaticEdge<N> {
    #[inline(always)]
    fn new(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e + 1))
    }

    #[inline(always)]
    fn new_reverse(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e))
    }

    #[inline(always)]
    fn to_index(self) -> usize {
        Num::to_usize(self.0) / 2
    }

    #[inline(always)]
    fn reverse(self) -> Self {
        StaticEdge(Num::from_usize(Num::to_usize(self.0) ^ 1))
    }
}

impl<N: Num> traits::Item for StaticEdge<N> {
    type Option = N;

    #[inline(always)]
    fn new_none() -> Self::Option {
        N::max()
    }

    #[inline(always)]
    fn to_some(&self) -> Self::Option {
        self.0
    }
}

impl<N: Num> PartialEq for StaticEdge<N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_index() == other.to_index()
    }
}

impl<N: Num> PartialOrd for StaticEdge<N> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_index().partial_cmp(&other.to_index())
    }
}

impl<N: Num> Ord for StaticEdge<N> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl<N: Num> Hash for StaticEdge<N> {
    #[inline(always)]
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}

#[derive(Clone, Debug)]
pub struct PropStaticEdge<T>(Vec<T>);

impl<T> Deref for PropStaticEdge<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, N: Num> Index<StaticEdge<N>> for PropStaticEdge<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: StaticEdge<N>) -> &Self::Output {
        self.0.index(index.to_index())
    }
}

impl<T, N: Num> IndexMut<StaticEdge<N>> for PropStaticEdge<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: StaticEdge<N>) -> &mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}


// Vertex

pub type StaticVertex<N> = N;

#[derive(Clone, Debug)]
pub struct PropStaticVertex<T>(Vec<T>);

impl<T> Deref for PropStaticVertex<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, N: Num> Index<StaticVertex<N>> for PropStaticVertex<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: StaticVertex<N>) -> &Self::Output {
        self.0.index(Num::to_usize(index))
    }
}

impl<T, N: Num> IndexMut<StaticVertex<N>> for PropStaticVertex<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: StaticVertex<N>) -> &mut Self::Output {
        self.0.index_mut(Num::to_usize(index))
    }
}

impl<N: Num> traits::Item for StaticVertex<N> {
    type Option = StaticVertex<N>;

    #[inline(always)]
    fn new_none() -> Self::Option {
        N::max()
    }

    #[inline(always)]
    fn to_some(&self) -> Self::Option {
        *self
    }
}

// TODO: Define a feature to disable property bounds check for vertex and edge property.

// StaticGraphGeneric

#[derive(Clone)]
pub struct StaticGraphGeneric<V: Num, E: Num> {
    num_vertices: usize,
    endvertices: Vec<StaticVertex<V>>,
    inc: Vec<Vec<StaticEdge<E>>>,
}

impl<V: Num, E: Num> StaticGraphGeneric<V, E> {
    pub fn new_with_edges(num_vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut builder = StaticGraphGeneric::builder(num_vertices, edges.len());
        for &(u, v) in edges {
            builder.add_edge(u, v)
        }
        builder.finalize()
    }

    pub fn new_empty() -> Self {
        StaticGraphGeneric::new_with_edges(0, &[])
    }

    fn add_edge(&mut self, u: Vertex<Self>, v: Vertex<Self>) {
        self.endvertices.push(u);
        self.endvertices.push(v);
        let e = (self.endvertices.len() - 2) / 2;
        self.inc[Num::to_usize(u)].push(StaticEdge::new(e));
        self.inc[Num::to_usize(v)].push(StaticEdge::new_reverse(e));
    }

    fn inc(&self, v: Vertex<Self>) -> &Vec<StaticEdge<E>> {
        self.inc.index(Num::to_usize(v))
    }
}

impl<V: Num, E: Num> WithBuilder for StaticGraphGeneric<V, E> {
    type Builder = StaticGraphGenericBuilder<V, E>;

    fn builder(num_vertices: usize, num_edges: usize) -> Self::Builder {
        // TODO: test this assert
        assert!(V::is_valid(num_vertices));
        StaticGraphGenericBuilder {
            g: StaticGraphGeneric {
                num_vertices: num_vertices,
                endvertices: Vec::with_capacity(2 * num_edges),
                inc: vec![vec![]; num_vertices],
            },
        }
    }
}

pub struct StaticGraphGenericBuilder<V: Num, E: Num> {
    g: StaticGraphGeneric<V, E>,
}

impl<V: Num, E: Num> Builder for StaticGraphGenericBuilder<V, E> {
    type Graph = StaticGraphGeneric<V, E>;

    fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(Num::from_usize(u), Num::from_usize(v));
    }

    fn finalize(self) -> Self::Graph {
        // TODO: test this assert
        assert!(E::is_valid(self.g.endvertices.len()));
        self.g
    }

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>) {
        // TODO: test this assert
        assert!(E::is_valid(self.g.endvertices.len()));
        let v = self.g.vertices().into_vec();
        let e = self.g.edges().into_vec();
        (self.g, v, e)
    }
}


impl<'a, V: Num, E: Num> IterTypes<'a, StaticGraphGeneric<V, E>> for StaticGraphGeneric<V, E> {
    type Vertex = V::Range;
    type Edge = Map<Range<usize>, fn(usize) -> StaticEdge<E>>;
    type Inc = Cloned<Iter<'a, StaticEdge<E>>>;
}

impl<V: Num, E: Num> Basic for StaticGraphGeneric<V, E> {
    type Vertex = StaticVertex<V>;
    type Edge = StaticEdge<E>;

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices(&self) -> IterVertex<Self> {
        V::range(0, self.num_vertices)
    }

    #[inline(always)]
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[Num::to_usize(e.0) ^ 1]
    }

    #[inline(always)]
    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[Num::to_usize(e.0)]
    }

    fn num_edges(&self) -> usize {
        self.endvertices.len() / 2
    }

    fn edges(&self) -> IterEdge<Self> {
        // TODO: iterate over 1, 3, 5, ...
        (0..self.num_edges()).map(StaticEdge::new)
    }

    #[inline(always)]
    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        e.reverse()
    }

    // Inc

    #[inline(always)]
    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[Num::to_usize(v)].len()
    }

    fn inc_edges(&self, v: Vertex<Self>) -> IterInc<Self> {
        self.inc(v).iter().cloned()
    }
}

impl<T: 'static + Clone, V: Num, E: Num> WithProps<T> for StaticGraphGeneric<V, E> {
    type Vertex = PropStaticVertex<T>;
    type Edge = PropStaticEdge<T>;

    fn vertex_prop(&self, value: T) -> PropVertex<Self, T> {
        PropStaticVertex(Vec::with_value(value, self.num_vertices()))
    }

    fn edge_prop(&self, value: T) -> PropEdge<Self, T> {
        PropStaticEdge(Vec::with_value(value, self.num_edges()))
    }
}


impl<V: Num, E: Num> Choose for StaticGraphGeneric<V, E>  {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        Num::from_usize(rng.gen_range(0, self.num_vertices()))
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        StaticEdge::new(rng.gen_range(0, self.num_edges()))
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc(v)[rng.gen_range(0, self.degree(v))]
    }
}

// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use builder::*;
    use tests::*;

    #[test]
    fn builder() {
        let mut builder = StaticGraph::builder(3, 1);

        builder.add_edge(0, 1);
        builder.add_edge(1, 2);

        let g = builder.finalize();
        assert_eq!(3, g.num_vertices);
        assert_eq!(vec![0, 1, 1, 2], g.endvertices);
        assert_eq!(vec![vec![StaticEdge::new(0)],
                        vec![StaticEdge::new_reverse(0), StaticEdge::new(1)],
                        vec![StaticEdge::new_reverse(1)]],
                   g.inc);
    }


    impl StaticGraph {
        fn new(num_vertices: usize,
               edges: &[(usize, usize)])
               -> (Self, VecVertex<Self>, VecEdge<Self>) {
            let g = StaticGraph::new_with_edges(num_vertices, edges);
            let vertices = g.vertices().into_vec();
            let edges = g.edges().into_vec();
            (g, vertices, edges)
        }
    }

    test_basic!{ StaticGraph }
    test_degree!{ StaticGraph }
    test_inc!{ StaticGraph }
    test_adj!{ StaticGraph }
    test_vertex_prop!{ StaticGraph }
    test_edge_prop!{ StaticGraph }
}
