use itertools::Itertools;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut nodes = do_the_line(input);
    let total = nodes.len();
    let mut current_best_cut = i64::MAX;
    let mut best_partition = Vec::new();
    let mut combined: HashMap<String, Vec<String>> = HashMap::new();

    while nodes.len() > 1 {
        let (t, s, weight) = minimum_cut_phase(&nodes);

        let mut t_set;
        if combined.contains_key(&t) {
            t_set = combined.get(&t).unwrap().clone();
            t_set.push(t.clone());
            combined.remove(&t);
        } else {
            t_set = vec![t.clone()]
        }
        if combined.contains_key(&s) {
            combined.get_mut(&s).unwrap().append(&mut t_set.clone());
        } else {
            combined.insert(s.clone(), t_set.clone());
        }

        if current_best_cut > weight {
            current_best_cut = weight;
            best_partition = t_set.clone();
        }

        merge_nodes(&mut nodes, s, t);
    }
    println!("Best cut: {:?}", current_best_cut);
    println!("Total: {}", total);
    println!("Part len: {}", best_partition.len());

    best_partition.len() as i64 * (total - best_partition.len()) as i64
}

fn merge_nodes(nodes: &mut HashMap<String, HashMap<String, i64>>, s: String, t: String) {
    println!("Nodes remaining {:?}", nodes.len());
    let edges_s = nodes.get(&s).unwrap().clone();
    let edges_t = nodes.get(&t).unwrap().clone();

    for e in edges_t.keys() {
        if edges_s.contains_key(e) {
            let weight = edges_s.get(e).unwrap() + edges_t.get(e).unwrap();
            nodes.get_mut(&s).unwrap().insert(e.to_string(), weight);
            nodes.get_mut(e).unwrap().insert(s.to_string(), weight);
            nodes.get_mut(e).unwrap().remove(&t);
        } else {
            if e == &s {
                continue;
            }
            nodes
                .get_mut(&s)
                .unwrap()
                .insert(e.to_string(), *edges_t.get(e).unwrap());

            nodes
                .get_mut(e)
                .unwrap()
                .insert(s.to_string(), *edges_t.get(e).unwrap());
            nodes.get_mut(e).unwrap().remove(&t);
        }
    }
    nodes.remove(&t);
    nodes.get_mut(&s).unwrap().remove(&t);
}

fn minimum_cut_phase(nodes: &HashMap<String, HashMap<String, i64>>) -> (String, String, i64) {
    let mut found_set = Vec::new();
    let mut cut_weight = Vec::new();
    let mut candidates: HashSet<String> = nodes.keys().map(|x| x.to_string()).collect();
    let start = candidates.iter().next().unwrap().to_string();
    candidates.remove(&start);
    found_set.push(start.clone());
    let mut b_heap = BinaryHeap::new();
    for nbr in nodes.get(&start).unwrap().keys() {
        b_heap.push((
            (get_weight(nodes, &found_set, nbr.to_string())),
            nbr.to_string(),
        ));
    }

    while candidates.len() > 0 {
        let next = b_heap.pop().unwrap();

        found_set.push(next.1.clone());
        for nbr in nodes.get(&next.1).unwrap().keys() {
            if candidates.contains(nbr) {
                let weight = get_weight(nodes, &found_set, nbr.to_string());
                b_heap.retain(|x| &x.1 != nbr);
                b_heap.push(((weight), nbr.to_string()));
            }
        }

        candidates.remove(&next.1);
        cut_weight.push(next.0);
    }

    return (
        found_set.pop().unwrap().to_string(),
        found_set.pop().unwrap().to_string(),
        cut_weight.pop().unwrap(),
    );
}

fn get_weight(
    nodes: &HashMap<String, HashMap<String, i64>>,
    found_set: &Vec<String>,
    nbr: String,
) -> i64 {
    let mut weight = 0;
    for next in nodes.get(&nbr).unwrap().keys() {
        if found_set.contains(next) {
            weight += nodes.get(&nbr).unwrap().get(next).unwrap();
        }
    }
    weight
}

fn do_the_line(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let mut nodes: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let mut vertices = HashSet::new();
    for line in input.lines() {
        let (a, b) = line.split(": ").collect_tuple().unwrap();
        let c = b.split(" ").collect_vec();
        for d in c.iter() {
            vertices.insert((a.to_string(), d.to_string()));
        }
    }
    for (n1, n2) in vertices.clone() {
        if nodes.contains_key(&n1) {
            nodes.get_mut(&n1).unwrap().insert(n2.to_string(), 1);
        } else {
            nodes.insert(
                n1.to_string(),
                HashMap::from_iter(vec![(n2.to_string(), 1)]),
            );
        }
        if nodes.contains_key(&n2) {
            nodes.get_mut(&n2).unwrap().insert(n1.to_string(), 1);
        } else {
            nodes.insert(
                n2.to_string(),
                HashMap::from_iter(vec![(n1.to_string(), 1)]),
            );
        }
    }
    println!("{:?}", nodes.len());
    let ns2 = nodes.clone();
    let mut nodes2 = ns2.iter().collect_vec();
    nodes2.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    nodes
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), 54);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 550080);
    }
}
