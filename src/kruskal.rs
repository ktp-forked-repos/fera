use graph::*;
use iter::IteratorExt;
use unionfind::DisjointSet;

#[derive(PartialEq, Eq)]
pub enum Accept {
    Yes,
    No,
}

pub trait Visitor<G>
    where G: Graph
{
    fn visit(&mut self, e: Edge<G>) -> Accept;
}

impl<F, G> Visitor<G> for F
    where G: Graph,
          F: FnMut(Edge<G>) -> Accept
{
    fn visit(&mut self, e: Edge<G>) -> Accept {
        self(e)
    }
}

pub trait Kruskal: Graph {
    fn kruskal_edges<'a, I, V>(&'a self, edges: I, visitor: &mut V)
        where &'a Self: Types<Self>,
              I: Iterator<Item=Edge<Self>>,
              V: Visitor<Self>
    {
        let mut ds = DisjointSet::new(self);
        let mut num_sets = self.num_vertices();
        for e in edges {
            let (u, v) = self.endvertices(e);
            if !ds.in_same_set(u, v) && visitor.visit(e) == Accept::Yes {
                ds.union(u, v);
                num_sets -= 1;
                if num_sets == 1 {
                    return;
                }
            }
        }
    }

    fn kruskal<'a, T, V>(&'a self, weight: &'a PropEdge<Self, T>, visitor: &mut V)
        where &'a Self: Types<Self>,
              Self: WithProps<T>,
              T: 'a + Ord + Clone,
              V: Visitor<Self>,
    {
        let mut edges = self.edges().into_vec();
        edges.sort_by(|&a, &b| weight[a].cmp(&weight[b]));
        self.kruskal_edges(edges.iter().cloned(), visitor);
    }

    fn kruskal_mst<'a, T>(&'a self, weight: &'a PropEdge<Self, T>) -> VecEdge<Self>
        where &'a Self: Types<Self>,
              Self: WithProps<T>,
              T: 'a + Ord + Clone,
    {
        let mut edges = vec![];
        self.kruskal::<T, _>(weight, &mut |e| {
            edges.push(e);
            Accept::Yes
        });
        edges
    }
}

impl<G> Kruskal for G
    where G: Graph { }


#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use iter::*;
    use kruskal::*;

    #[test]
    fn kruskal_mst() {
        let g = StaticGraph::new_with_edges(
            5,
            &[(0, 4), (2, 3), (0, 1), (1, 4), (1, 2), (2, 4), (3, 4)]);
        // expected tree
        //      0       1       2               3
        let mut weight = g.edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight));
    }
}
