use std::{collections::{HashSet, VecDeque, BinaryHeap}, cmp::Reverse, ops::Add};
use super::disjoint_set::DisjointSet;

pub trait Graph {
    type GraphEdge: Edge;
    type EdgeIterator<'a>: Iterator<Item = &'a Self::GraphEdge> where Self: 'a;
    fn edges(&self, u: usize) -> Self::EdgeIterator<'_>;
    fn all_edges(&self) -> Vec<(usize, usize, <<Self as Graph>::GraphEdge as Edge>::Weight)> {
        let mut result = vec![];
        for i in 0..self.len() {
            for e in self.edges(i) {
                result.push((i, e.to(), e.weight()));
            }
        }
        result
    }
    fn neighbors(&self, u: usize) -> Vec<&usize>;
    fn len(&self) -> usize;
    fn bfs(&self, start: usize) -> GraphBFS<Self> where Self: Sized {
        let mut q = VecDeque::new();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        for v in self.edges(start) {
            visited[v.to()] = true;
            q.push_back((v.to(), start));
        }
        GraphBFS { graph: self, q, visited }
    }
    fn dfs(&self, start: usize) -> GraphDFS<Self> where Self: Sized {
        let mut stack = Vec::new();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        for v in self.edges(start) {
            visited[v.to()] = true;
            stack.push((v.to(), start));
        }
        GraphDFS { graph: self, stack, visited }
    }
    fn djikstra(&self, start: usize, end: usize) -> Option<(<Self::GraphEdge as Edge>::Weight, Vec<usize>)> where Self: Sized {
        let zero = Default::default();

        if start == end {
            return Some((zero, vec![start]));
        }
        let mut bh = BinaryHeap::new();

        for neighbor in self.edges(start) {
            bh.push((Reverse(neighbor.weight()), neighbor.to(), start));
        }
        
        let mut path = vec![None; self.len()];
        let mut visited = vec![false; self.len()];
        visited[start] = true;

        while let Some((Reverse(dist), u, p)) = bh.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            path[u] = Some((dist, p));
            if u == end {
                break;
            }
            for neighbor in self.edges(u) {
                bh.push((Reverse(dist + neighbor.weight()), neighbor.to(), u));
            }
        }

        if let Some(ep) = path[end] {
            let mut result = vec![end];
            let mut u = end;
            let dist = ep.0;

            while let Some((_, v)) = path[u] {
                result.push(v);
                u = v;
            }

            result.reverse();
            Some((dist, result))
        } else {
            None
        }
    }
    fn spanning_tree(&self) -> Option<Vec<(usize, usize, <Self::GraphEdge as Edge>::Weight)>> where Self: Sized {
        let mut ds = DisjointSet::new(self.len());

        let mut edges = vec![];
        for i in 0..self.len() {
            edges.extend(self.edges(i).map(|e| (e.weight(), i, e.to())));
        }

        edges.sort();

        let mut result = vec![];

        for (w, u, v) in edges {
            if ds.union(u, v) {
                result.push((u, v, w));
                if result.len() == self.len() - 1 {
                    return Some(result);
                }
            }
        }

        None

    }

}

impl Graph for Vec<Vec<usize>> {
    type GraphEdge = usize;
    type EdgeIterator<'a> = std::slice::Iter<'a, usize>;
    fn edges(&'_ self, u: usize) -> Self::EdgeIterator<'_> {
        self[u].iter()
    }
    fn neighbors(&self, u: usize) -> Vec<&usize> {
        self[u].iter().collect()
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl<W: EdgeWeight> Graph for Vec<Vec<(usize, W)>> {
    type GraphEdge = (usize, W);
    type EdgeIterator<'a> = std::slice::Iter<'a, (usize, W)> where W: 'a;
    fn edges(&'_ self, u: usize) -> Self::EdgeIterator<'_> {
        self[u].iter()
    }
    fn neighbors(&self, u: usize) -> Vec<&usize> {
        self[u].iter().map(|(v, _)| v).collect()
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl Graph for Vec<HashSet<usize>> {
    type GraphEdge = usize;
    type EdgeIterator<'a> = std::collections::hash_set::Iter<'a, usize>;
    fn edges(&'_ self, u: usize) -> Self::EdgeIterator<'_> {
        self[u].iter()
    }
    fn neighbors(&self, u: usize) -> Vec<&usize> {
        self[u].iter().collect()
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl<W: EdgeWeight> Graph for Vec<HashSet<(usize, W)>> {
    type GraphEdge = (usize, W);
    type EdgeIterator<'a> = std::collections::hash_set::Iter<'a, (usize, W)> where W: 'a;
    fn edges(&'_ self, u: usize) -> Self::EdgeIterator<'_> {
        self[u].iter()
    }
    fn neighbors(&self, u: usize) -> Vec<&usize> {
        self[u].iter().map(|(v, _)| v).collect()
    }
    fn len(&self) -> usize {
        self.len()
    }
}

pub struct GraphBFS<'a, G: Graph> {
    graph: &'a G,
    q: VecDeque<(usize, usize)>,
    visited: Vec<bool>,
}

impl<'a, G: Graph> Iterator for GraphBFS<'a, G> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (u, p) = self.q.pop_front()?;

        for v in self.graph.edges(u) {
            if !self.visited[v.to()] {
                self.visited[v.to()] = true;
                self.q.push_back((v.to(), u));
            }
        }

        Some((u, p))
    }
}

pub struct GraphDFS<'a, G: Graph> {
    graph: &'a G,
    stack: Vec<(usize, usize)>,
    visited: Vec<bool>,
}

impl<'a, G: Graph> Iterator for GraphDFS<'a, G> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (u, p) = self.stack.pop()?;

        for v in self.graph.edges(u) {
            if !self.visited[v.to()] {
                self.visited[v.to()] = true;
                self.stack.push((v.to(), u));
            }
        }

        Some((u, p))
    }
}

pub trait Edge {
    type Weight: EdgeWeight;
    fn to(&self) -> usize;
    fn weight(&self) -> Self::Weight;
}

impl Edge for usize {
    type Weight = usize;
    fn to(&self) -> usize {
        *self
    }
    fn weight(&self) -> Self::Weight {
        1
    }
}

impl<W: EdgeWeight> Edge for (usize, W) {
    type Weight = W;
    fn to(&self) -> usize {
        self.0
    }
    fn weight(&self) -> Self::Weight {
        self.1
    }
}

pub trait EdgeWeight: Ord + Clone + std::fmt::Debug + Default + Copy + Add<Self, Output=Self>{}

impl<T: Ord + Clone + std::fmt::Debug + Default + Copy + Add<Self, Output=Self>> EdgeWeight for T {}
