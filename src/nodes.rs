use std::cmp::Ordering;
use std::collections::BTreeSet;

struct Node {}

struct Group {
    idx: u32,
    nodes: Vec<Node>,
}

impl Eq for Group {}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.nodes.len() == other.nodes.len() {
            self.idx.cmp(&other.idx)
        } else {
            self.nodes.len().cmp(&other.nodes.len())
        }
    }
}

impl PartialEq<Self> for Group {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl PartialOrd<Self> for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.nodes.len() == other.nodes.len() {
            self.idx.partial_cmp(&other.idx)
        } else {
            self.nodes.len().partial_cmp(&other.nodes.len())
        }
    }
}

impl Group {
    fn new(idx: u32) -> Self {
        Self { idx, nodes: vec![] }
    }
}

struct GroupManager {
    groups: BTreeSet<Group>,
}

impl GroupManager {
    fn init(amount: usize) -> Self {
        fn init_groups(amount: usize) -> Vec<Group> {
            let mut v = vec![];
            for i in 0..amount {
                v.push(Group::new(i as u32));
            }

            v
        }
        let groups = init_groups(amount);
        let groups: BTreeSet<Group> = groups.into_iter().collect();
        Self { groups }
    }

    fn add_node(&mut self, node: Node) -> Result<(), ()> {
        if let Some(mut group) = self.groups.pop_first() {
            group.nodes.push(node);
            self.groups.insert(group);
            Ok(())
        } else {
            Err(()) // shouldn't really happen
        }
    }

    fn print_stats(&self) {
        print!("{} :", self.groups.len());
        for i in self.groups.iter() {
            print!("{} ", i.nodes.len())
        }
    }
}

#[test]
fn test_1() {
    fn gen_nodes(amount: u32) -> Vec<Node> {
        let mut nodes = vec![];
        for _ in 0..amount {
            nodes.push(Node {})
        }
        nodes
    }
    let mut group_manager = GroupManager::init(10);
    let nodes = gen_nodes(97);
    for i in nodes.into_iter() {
        group_manager.add_node(i).unwrap();
    }
    group_manager.print_stats();
}
