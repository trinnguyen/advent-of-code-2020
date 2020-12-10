use std::{fmt, io::Read};
use std::collections::{LinkedList, HashSet, HashMap};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(&str));
    println!("result part-2: {:?}", part_2(&str));
}

fn part_1(str: &str) -> usize {
    let mut gr = RuleGraph::new();
    for line in str.lines() {
        let rule = Rule::from(line);
        
        // add rule
        for dst in &rule.vec_dst {
            gr.add_edge(&dst.0, &rule.src, 0);
        }
    }
    
    gr.get_all_reachable(gr.get_index("shiny gold")).len()
}

fn part_2(str: &str) -> u32 {
    let mut gr = RuleGraph::new();
    for line in str.lines() {
        let rule = Rule::from(line);
        
        // build reversed graph compared to part 1
        for dst in &rule.vec_dst {
            gr.add_edge(&rule.src, &dst.0, dst.1);
        }
    }

    // input
    let idx: usize = gr.get_index("shiny gold");

    // travel graph and sum
    return gr.sum_reachable_and_weight(idx);
}

#[derive(Debug)]
struct Rule {
    src: String,
    vec_dst: Vec<(String, u32)>,
}

impl Rule {
    /// get rule from line
    fn from(line: &str) -> Self {
        let mut iter = line.split(" bags contain ");
        match (iter.next(), iter.next()) {
            (Some(src), Some(trail)) => {
                // childs
                let vec: Vec<(String, u32)> = if trail.eq("no other bags.") {
                    Vec::new()
                } else {
                    trail.split(", ").map(|t| Rule::get_child(t)).collect()
                };

                return Rule {
                    src: src.to_string(),
                    vec_dst: vec
                };
            },
            _ => panic!("line format is invalid")
        }
    }

    /// get child from line
    ///
    /// input:
    ///   example: 1 bright white bag
    ///   example: 2 muted yellow bags
    ///
    /// output:
    ///   (bright white, 1)
    ///   (muted yellow, 2)
    fn get_child(str: &str) -> (String, u32) {
        let mut iter = str.split(' ');
        let x = iter.next();
        match x {
            Some(s_num) => {
                let n: u32 = s_num.parse().unwrap();

                // get name
                let mut name = String::new();
                loop {
                    match iter.next() {
                        Some("bag") | Some("bag.") | Some("bags") | Some("bags.") => break,
                        Some(t) => { 
                            if !name.is_empty() {
                                name.push(' ');
                            }
                            name.push_str(t); 
                        },
                        _ => break
                    }
                }

                return (name, n)
            },
            _ => panic!("missing number")
        }
    }
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
    
    /// each item in the linked list represents an index and weight
    list: Vec<LinkedList<(usize, u32)>>,
    map_name: HashMap<String, usize>
}

impl std::fmt::Display for RuleGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "{:?}", self.map_name);
        let mut idx: usize = 0;
        for elem in &self.list {
            let _ = writeln!(f, "{}: {:?}", idx, elem);
            idx = idx + 1;
        }

        Ok(())
    }
}

impl RuleGraph {

    fn new() -> RuleGraph {
        return RuleGraph {
            list: Vec::new(),
            map_name: HashMap::new()
        }
    }

    fn add_edge(&mut self, src: &str, dst: &str, weight: u32) {
        let idx_src = self.get_or_update_name(src);
        let idx_dst = self.get_or_update_name(dst);
        if let Some(linked_src) = self.list.get_mut(idx_src) {
            (*linked_src).push_back((idx_dst, weight))
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
        for (item, _) in linked_list.iter() {
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

    fn get_index(&self, name: &str) -> usize {
        match self.map_name.get(name) {
            Some(idx) => *idx,
            _ => panic!("no bag name {} is exist", name)
        }
    }

    fn sum_reachable_and_weight(&self, index: usize) -> u32 {
        if index >= self.list.len() {
            panic!("out of range")
        }

        let mut sum: u32 = 0;

        // iterate over all successors
        let linked_list = &self.list[index];
        for (item, weight) in linked_list.iter() {
            let s_child = self.sum_reachable_and_weight(*item);
            sum = sum + weight * (1 + s_child);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    
    use std::collections::HashSet;
    use crate::{RuleGraph, Rule};

    #[test]
    fn test_graph() {
        let mut gr = RuleGraph::new();
        gr.add_edge("A" ,"B", 0);
        gr.add_edge("A" ,"C", 0);
        gr.add_edge("B" ,"D", 0);
        gr.add_edge("E" ,"D", 0);
    
        println!("{:?}", gr);
    
        let reach = gr.get_all_reachable(0);
        let mut expected : HashSet<usize> = HashSet::new();
        expected.insert(3);
        expected.insert(2);
        expected.insert(1);
        assert_eq!(expected, reach);
    }

    #[test]
    fn test_get_child() {
        assert_eq!(("bright white".to_string(), 1), Rule::get_child("1 bright white bag"));
        assert_eq!(("muted yellow".to_string(), 2), Rule::get_child("2 muted yellow bags"));
    }
}