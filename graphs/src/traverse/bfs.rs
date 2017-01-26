use prelude::*;
use props::Color;
use traverse::*;
use params::*;

use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::iter;

trait_alias!(BfsWithRoot = Incidence + WithVertexProp<Color>);
trait_alias!(BfsDefault = Incidence + VertexList + WithVertexProp<Color>);

pub trait Bfs: WithEdge {
    fn bfs<V>(&self, vis: V) -> Control
        where Self: BfsDefault,
              V: Visitor<Self>
    {
        self.bfs_()
            .visitor(vis)
            .run()
    }

    fn bfs_with_root<V>(&self, root: Vertex<Self>, vis: V) -> Control
        where Self: BfsWithRoot,
              V: Visitor<Self>
    {
        self.bfs_()
            .visitor(vis)
            .root(root)
            .run()
    }

    fn bfs_(&self) -> BfsAlg<&Self, EmptyVisitor, AllVertices, NewVertexProp<Color>, NewBfsQueue> {
        BfsAlg(self,
               EmptyVisitor,
               AllVertices,
               NewVertexProp(Color::White),
               NewBfsQueue)
    }
}

impl<G: WithEdge> Bfs for G {}


generic_struct!(BfsAlg(graph, visitor, roots, color, queue));

impl<'a, G, V, R, C, Q> BfsAlg<&'a G, V, R, C, Q> {
    pub fn run(self) -> Control
        where G: Incidence,
              V: Visitor<G>,
              R: ParamIterator<'a, G, Item = Vertex<G>>,
              C: ParamVertexProp<G, Color>,
              Q: Param<'a, G, BfsQueue<G>>
    {
        let BfsAlg(g, mut vis, roots, color, queue) = self;
        return_unless!(vis.start(g));
        let mut color = color.build(g);
        let color = color.borrow_mut();
        let mut queue = queue.build(g);
        let queue = queue.borrow_mut();
        for v in roots.build(g) {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                queue.push_back((G::edge_none(), v));
                break_unless!(vis.discover_root_vertex(g, v));
                break_unless!(vis.discover_vertex(g, v));
                break_unless!(bfs_visit(g, color, queue, &mut vis));
                break_unless!(vis.finish_root_vertex(g, v));
            }
        }
        vis.finish(g)
    }

    pub fn root(self, root: Vertex<G>) -> BfsAlg<&'a G, V, iter::Once<Vertex<G>>, C, Q>
        where G: WithVertex
    {
        self.roots(iter::once(root))
    }
}

pub fn bfs_visit<G, C, V>(g: &G, color: &mut C, queue: &mut BfsQueue<G>, vis: &mut V) -> Control
    where G: Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    while let Some((from, u)) = queue.pop_front() {
        for e in g.out_edges(u) {
            let v = g.target(e);
            if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
                continue;
            }
            return_unless!(vis.discover_edge(g, e));
            match color[v] {
                Color::White => {
                    color[v] = Color::Gray;
                    queue.push_back((e.into(), v));
                    return_unless!(vis.discover_tree_edge(g, e));
                    return_unless!(vis.discover_vertex(g, v));
                    continue;
                }
                Color::Gray => {
                    return_unless!(vis.discover_back_edge(g, e));
                }
                Color::Black => {
                    return_unless!(vis.discover_cross_or_forward_edge(g, e));
                }
            }
            return_unless!(vis.finish_edge(g, e));
        }
        color[u] = Color::Black;
        return_unless!(vis.finish_vertex(g, u));
        if let Some(from) = from.into_option() {
            return_unless!(vis.finish_tree_edge(g, from));
            return_unless!(vis.finish_edge(g, from));
        }
    }
    Control::Continue
}


pub type BfsQueue<G> = VecDeque<(OptionEdge<G>, Vertex<G>)>;

#[derive(Default)]
pub struct NewBfsQueue;

impl<'a, G: 'a + WithEdge> Param<'a, G, BfsQueue<G>> for NewBfsQueue {
    type Output = BfsQueue<G>;

    fn build(self, _g: &'a G) -> Self::Output {
        BfsQueue::<G>::new()
    }
}


// Tests

#[cfg(test)]
mod tests {
    use prelude::*;
    use traverse::*;
    use fera_fun::vec;

    fn new() -> StaticGraph {
        //    1
        //  / | \         4
        // 0  |  3      /   \
        //  \ | /      5 --- 6
        //    2
        graph!(StaticGraph,
               7,
               (0, 1),
               (0, 2),
               (1, 2),
               (1, 3),
               (2, 3),

               (4, 5),
               (4, 6),
               (5, 6))
    }

    fn edge_by_ends(g: &StaticGraph,
                    u: Vertex<StaticGraph>,
                    v: Vertex<StaticGraph>)
                    -> Edge<StaticGraph> {
        for e in g.edges() {
            let (x, y) = g.ends(e);
            if u == x && v == y {
                return e;
            } else if u == y && v == x {
                return g.reverse(e);
            }
        }
        panic!()
    }

    #[test]
    fn events() {
        use traverse::TraverseEvent::*;
        let g = new();
        let v = vec(g.vertices());
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
        let expected = vec![
            Start,

            DiscoverRootVertex(0),
            DiscoverVertex(0),
            DiscoverEdge(e(0, 1)),
            DiscoverTreeEdge(e(0, 1)),
            DiscoverVertex(1),
            DiscoverEdge(e(0, 2)),
            DiscoverTreeEdge(e(0, 2)),
            DiscoverVertex(2),
            FinishVertex(0),
            DiscoverEdge(e(1, 2)),
            DiscoverBackEdge(e(1, 2)),
            FinishEdge(e(1, 2)),
            DiscoverEdge(e(1, 3)),
            DiscoverTreeEdge(e(1, 3)),
            DiscoverVertex(3),
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
            DiscoverEdge(e(2, 3)),
            DiscoverBackEdge(e(2, 3)),
            FinishEdge(e(2, 3)),
            FinishVertex(2),
            FinishTreeEdge(e(0, 2)),
            FinishEdge(e(0, 2)),
            FinishVertex(3),
            FinishTreeEdge(e(1, 3)),
            FinishEdge(e(1, 3)),
            FinishRootVertex(0),

            DiscoverRootVertex(4),
            DiscoverVertex(4),
            DiscoverEdge(e(4, 5)),
            DiscoverTreeEdge(e(4, 5)),
            DiscoverVertex(5),
            DiscoverEdge(e(4, 6)),
            DiscoverTreeEdge(e(4, 6)),
            DiscoverVertex(6),
            FinishVertex(4),
            DiscoverEdge(e(5, 6)),
            DiscoverBackEdge(e(5, 6)),
            FinishEdge(e(5, 6)),
            FinishVertex(5),
            FinishTreeEdge(e(4, 5)),
            FinishEdge(e(4, 5)),
            FinishVertex(6),
            FinishTreeEdge(e(4, 6)),
            FinishEdge(e(4, 6)),
            FinishRootVertex(4),

            Finish,
        ];

        let mut v = vec![];
        g.bfs(OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);
    }
}