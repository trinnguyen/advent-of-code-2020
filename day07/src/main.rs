use std::{collections::HashSet, io::Read};
use std::str::Lines;
use std::collections::{LinkedList, HashMap};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(""));
}

fn part_1(str: &str) -> usize {
    for line in str.lines() {
        
    }
    return 0;
}

/// model the bag rules using a directed graph (reversed)
/// A graph contain all rules
/// Each node represents a bag
/// Each edge represent whether a bag (src) can be contained in another bag (dst)
///
/// If bagA contains bagB and bagC:
/// 3 nodes: bagA, bagB, bagC.
/// 2 edges: bagC -> bagA, bagB -> bagA
///
/// Use adjacency list to represent the directed graph
/// Use indices in list
/// Use an additional map to transform the bag's name to index
///
/// Example:
/// 0 -> 1 -> 2
/// 1 -> 3
/// 2 -> 4
#[derive(Debug)]
struct RuleGraph {
    list: Vec<LinkedList<usize>>,
    map_name: HashMap<String, usize>
}

impl RuleGraph {

    fn new() -> RuleGraph {
        return RuleGraph {
            list: Vec::new(),
            map_name: HashMap::new()
        }
    }

    fn add_edge(&mut self, src: &str, dst: &str) {
        let idx_src = self.get_or_update_name(src);
        let idx_dst = self.get_or_update_name(dst);
        if let Some(linked_src) = self.list.get_mut(idx_src) {
            (*linked_src).push_back(idx_dst)
        } else {
            panic!("unexpected error on getting linked list")
        }
    }

    fn get_or_update_name(&mut self, name: &str) -> usize {
        return if let Some(index) = self.map_name.get(name) {
            *index
        } else {
            let index = self.list.len();
            self.list.push(LinkedList::new());
            self.map_name.insert(String::from(name), index);
            index
        }
    }

    fn update_all_child_reachable(&self, index: usize, set: &mut HashSet<usize>) {
        if index >= self.list.len() {
            panic!("out of range")
        }

        // iterate over all successors
        let linked_list = &self.list[index];
        for item in linked_list.iter() {
            if set.contains(item) {
                continue;
            }

            set.insert(*item);
            self.update_all_child_reachable(*item, set);
        }
    }

    fn get_all_reachable(&self, index: usize) -> HashSet<usize> {
        let mut set = HashSet::new();
        self.update_all_child_reachable(index, &mut set);
        return set;
    }

    fn count_all_reachable_by_name(&self, name: &str) -> usize {
        if let Some(idx) = self.map_name.get(name) {
            return self.get_all_reachable(*idx).len();
        }

        panic!("no bag name {} is exist", name)
    }

}

#[cfg(test)]
mod tests {
    
    use std::collections::HashSet;
    use crate::RuleGraph;

    #[test]
    fn test_graph() {
        let mut gr = RuleGraph::new();
        gr.add_edge("A" ,"B");
        gr.add_edge("A" ,"C");
        gr.add_edge("B" ,"D");
        gr.add_edge("E" ,"D");
    
        println!("{:?}", gr);
    
        let reach = gr.get_all_reachable(0);
        let mut expected : HashSet<usize> = HashSet::new();
        expected.insert(3);
        expected.insert(2);
        expected.insert(1);
        assert_eq!(expected, reach);
    }
}