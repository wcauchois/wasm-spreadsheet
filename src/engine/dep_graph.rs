use imbl::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

// Somewhat based on this dep_graph crate: https://docs.rs/dep-graph/latest/dep_graph/struct.DepGraph.html

type DependencyMap<I> = HashMap<I, HashSet<I>>;

pub trait Node<I> {
    fn get_id(&self) -> I;
    fn get_deps<'a>(&'a self) -> &'a Vec<I>;
}

impl<I: fmt::Debug> fmt::Debug for Node<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("id", &self.get_id()).finish()
    }
}

#[derive(Debug)]
pub struct DepGraph<I: Clone + Eq + PartialEq + Hash + fmt::Debug> {
    ready_nodes: HashSet<I>,
    deps: DependencyMap<I>,
    rdeps: DependencyMap<I>,
}

impl<I> DepGraph<I>
where
    I: Clone + Eq + PartialEq + Hash + fmt::Debug,
{
    fn add_dependency_to_map(map: &mut DependencyMap<I>, from_id: &I, to_id: &I) {
        if !map.contains_key(from_id) {
            let mut set = HashSet::new();
            set.insert(to_id.clone());
            map.insert(from_id.clone(), set);
        } else {
            let set = map.get_mut(from_id).unwrap();
            set.insert(to_id.clone());
        }
    }

    fn remove_dependency_from_map(map: &mut DependencyMap<I>, from_id: &I, to_id: &I) {
        if let Some(set) = map.get_mut(from_id) {
            set.remove(to_id);
        }
    }

    pub fn with_nodes<N: Node<I>, Collection: IntoIterator<Item = N>>(nodes: Collection) -> Self {
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
                Self::add_dependency_to_map(&mut rdeps, &node_dep, &id);
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
            Some(node_rdeps) => Box::new(node_rdeps.iter().cloned()),
            None => Box::new(std::iter::empty()),
        }
    }

    /// Incremental update of a node.
    pub fn update_node<N: Node<I>>(&self, node: &N) -> Self {
        let node_id = node.get_id();
        let node_deps = node.get_deps();

        let ready_nodes = if node_deps.is_empty() {
            self.ready_nodes.update(node_id.clone())
        } else {
            self.ready_nodes.without(&node_id)
        };
        let deps = self.deps.update(node_id.clone(), node_deps.into());

        let old_node_deps = match self.deps.get(&node_id) {
            Some(set) => set.clone(),
            None => HashSet::default(),
        };

        let node_deps_set: HashSet<I> = node_deps.iter().cloned().collect();
        let added_node_deps = node_deps_set
            .clone()
            .relative_complement(old_node_deps.clone());
        let removed_node_deps = old_node_deps
            .clone()
            .relative_complement(node_deps_set.clone());

        let mut rdeps = self.rdeps.clone();
        println!("new deps: {:?}", node_deps);
        println!("new deps set: {:?}", node_deps_set);
        println!("old deps: {:?}", old_node_deps);
        println!("added deps: {:?}", added_node_deps);
        println!("removed deps: {:?}", removed_node_deps);
        // Add rdeps for added_node_deps
        for node_dep in added_node_deps {
            Self::add_dependency_to_map(&mut rdeps, &node_dep, &node_id);
        }

        // Remove rdeps for removed_node_deps
        for node_dep in removed_node_deps {
            Self::remove_dependency_from_map(&mut rdeps, &node_dep, &node_id);
        }

        Self {
            ready_nodes,
            deps,
            rdeps,
        }
    }
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

    fn same_elements(a: Vec<i32>, b: Vec<i32>) -> bool {
        let mut sorted_a = a.clone();
        sorted_a.sort();
        let mut sorted_b = b.clone();
        sorted_b.sort();
        sorted_a == sorted_b
    }

    macro_rules! assert_same_elements {
        ($a:expr, $b:expr) => {
            assert!(
                same_elements($a, $b),
                "Vectors must have the same elements:\n{:?}\n{:?}",
                $a,
                $b
            );
        };
    }

    #[test]
    fn test_basic() {
        let nodes = vec![
            TestNode::new(1, vec![]),
            TestNode::new(2, vec![1, 3]),
            TestNode::new(3, vec![1]),
            TestNode::new(4, vec![3, 5]),
            TestNode::new(5, vec![]),
        ];

        let graph = DepGraph::with_nodes(nodes);
        assert_same_elements!(
            graph.get_direct_dependents(1).collect::<Vec<_>>(),
            vec![3, 2]
        );
        assert_same_elements!(
            graph.get_direct_dependents(3).collect::<Vec<_>>(),
            vec![2, 4]
        );
        assert_same_elements!(graph.get_direct_dependents(5).collect::<Vec<_>>(), vec![4]);

        let graph = graph.update_node(&TestNode::new(4, vec![1, 5]));
        // Rdep to 4 was added to 1.
        assert_same_elements!(
            graph.get_direct_dependents(1).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
        // Rdep to 4 was removed from 3.
        assert_same_elements!(graph.get_direct_dependents(3).collect::<Vec<_>>(), vec![2]);
        // 4 still depends on 4.
        assert_same_elements!(graph.get_direct_dependents(5).collect::<Vec<_>>(), vec![4]);
    }
}
