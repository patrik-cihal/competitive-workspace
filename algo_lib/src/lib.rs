#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]  

pub mod io;
pub mod collections;
pub mod math;


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn segment_tree() {
        use collections::segment_tree::SegmentTree;
        let mut tree = SegmentTree::new_from_iter(vec![0, 1, 5, 7, 2, 8], 0, |a, b| *a.max(b));
        
        assert_eq!(tree.query(0..6), 8);
        assert_eq!(tree.query(0..3), 5);
        assert_eq!(tree.query(3..5), 7);

        tree.update(2, 10);
        tree.update(4, 3);

        assert_eq!(tree.query(0..6), 10);
        assert_eq!(tree.get(4), &3);
    }

    #[test]
    fn segment_tree_ru() {
        use collections::segment_tree::SegmentTreeRU;
        let mut tree = SegmentTreeRU::new_from_iter(
            vec![0, 1, 5, 7, 2, 8],
            0,
            |a, b| *a.max(b),
            |d, l, length| d+*l,
            |l1, l2| *l1+l2
        );

        tree.update(0..5, &2);

        assert_eq!(tree.query(0..3), 7);

        tree.update(0..5, &2);

        assert_eq!(tree.query(0..3), 9);
    }



    #[test]
    fn disjoint_set() {
        use collections::disjoint_set::DisjointSet;

        let mut ds = DisjointSet::new(10);

        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(4, 5);
        ds.union(1, 2);

        assert_eq!(ds.find(0), ds.find(3));
        assert_eq!(ds.find(4), ds.find(5));
    }

    #[test]
    fn graphs() {
        use collections::graph::Graph;
        let g = vec![vec![1usize, 2], vec![0, 2], vec![0, 1, 3], vec![2]];

        assert_eq!(g.neighbors(0), vec![&1, &2]);
        let first_2 = g.edges(2).take(2).collect::<Vec<_>>();
        assert_eq!(first_2, vec![&0, &1]);

        let g_weighted = vec![
            vec![(1usize, 1), (2, 2)],
            vec![(0, 1), (2, 0)],
            vec![(0, 2), (4, 3), (3, 4)],
            vec![(2, 4)],
            vec![(1, 5), (2, 6)],
        ]; 

        let bfs = g_weighted.bfs(0).into_iter();
        let first_4 = bfs.map(|(x, _)| x).take(4).collect::<Vec<usize>>();
        assert_eq!(first_4, vec![1, 2, 4, 3]);

        let dfs = g_weighted.dfs(0).into_iter();
        let first_4 = dfs.map(|(x, _)| x).take(4).collect::<Vec<usize>>();
        assert_eq!(first_4, vec![2, 3, 4, 1]);

        let shortest_path = g_weighted.djikstra(0, 3);

        assert_eq!(shortest_path, Some((5, vec![0, 1, 2, 3])));
    }
}