use imbl::{HashMap, HashSet};
use std::hash::Hash;

// Somewhat based on this dep_graph crate: https://docs.rs/dep-graph/latest/dep_graph/struct.DepGraph.html

type DependencyMap<I> = HashMap<I, HashSet<I>>;

pub trait Node<I> {
    fn get_id(&self) -> I;
    fn get_deps<'a>(&'a self) -> &'a Vec<I>;
}

pub struct DepGraph<I: Clone + Eq + PartialEq + Hash> {
    ready_nodes: HashSet<I>,
    deps: DependencyMap<I>,
    rdeps: DependencyMap<I>,
}

impl<I> DepGraph<I>
where
    I: Clone + Eq + PartialEq + Hash,
{
    pub fn new<N: Node<I>, Collection: IntoIterator<Item = N>>(nodes: Collection) -> Self {
        let mut deps = DependencyMap::<I>::new();
        let mut rdeps = DependencyMap::<I>::new();
        let mut ready_nodes = HashSet::<I>::new();

        for node in nodes {
            let id = node.get_id();
            let node_deps = node.get_deps();

            deps.insert(id.clone(), node_deps.into());

            if node_deps.is_empty() {
                ready_nodes.insert(id.clone());
            }

            for node_dep in node_deps {
                if !rdeps.contains_key(node_dep) {
                    let mut dep_rdeps = HashSet::new();
                    dep_rdeps.insert(id.clone());
                    rdeps.insert(node_dep.clone(), dep_rdeps);
                } else {
                    let dep_rdeps = rdeps.get_mut(node_dep).unwrap();
                    dep_rdeps.insert(id.clone());
                }
            }
        }

        // TODO: Check for cycles?

        Self {
            ready_nodes,
            deps,
            rdeps,
        }
    }

    pub fn get_direct_dependents(&self, id: I) -> Box<dyn Iterator<Item = I> + '_> {
        match self.rdeps.get(&id) {
            Some(node_rdeps) => Box::new(node_rdeps.iter().map(|id| id.clone())),
            None => Box::new(std::iter::empty()),
        }
    }

    // pub fn update_node<N: Node<I>>(&self, node: N) -> Self {}
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNode {
        id: i32,
        deps: Vec<i32>,
    }

    impl TestNode {
        fn new(id: i32, deps: Vec<i32>) -> Self {
            Self { id, deps }
        }
    }

    impl Node<i32> for TestNode {
        fn get_id(&self) -> i32 {
            self.id
        }

        fn get_deps<'a>(&'a self) -> &'a Vec<i32> {
            &self.deps
        }
    }

    #[test]
    fn test_basic() {
        let nodes = vec![
            TestNode::new(1, vec![]),
            TestNode::new(2, vec![1, 3]),
            TestNode::new(3, vec![1]),
            TestNode::new(4, vec![3]),
        ];

        let graph = DepGraph::new(nodes);
        assert_eq!(
            graph.get_direct_dependents(1).collect::<Vec<_>>(),
            vec![2, 3]
        );
    }
}
