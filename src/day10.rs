use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<i64, Node>,
}
#[derive(Debug)]
struct Node {
    id: usize,
    pathcount: usize,
    children: Vec<i64>,
}

fn count_paths(graph: &mut Graph, nodenum: i64) -> usize {
    let node = graph.nodes.get_mut(&nodenum).unwrap();
    if node.pathcount != 0 {
        node.pathcount
    } else if node.children.is_empty() {
        node.pathcount = 1;
        1
    } else {
        let mut sum = 0;
        let kids = node.children.to_vec();
        for c in kids {
            {
                let paths = count_paths(graph, c);
                sum += paths;
            }
        }
        graph.nodes.get_mut(&nodenum).unwrap().pathcount = sum;
        sum
    }
}

fn make_graph(nums: &[i64]) -> Graph {
    let mut sortednums = nums.to_vec();
    sortednums.push(0);
    sortednums.sort_unstable();
    let maxplus3 = sortednums[sortednums.len() - 1] + 3;
    sortednums.push(maxplus3);
    sortednums.sort_unstable();

    let last_node = Node {
        id: maxplus3 as usize,
        pathcount: 0,
        children: vec![],
    };
    let mut nodes: HashMap<i64, Node> = HashMap::new();
    nodes.insert(maxplus3, last_node);
    for i in (0..sortednums[sortednums.len() - 1]).rev() {
        if sortednums.contains(&i) {
            let mut children = Vec::new();
            for j in i + 1..i + 4 {
                if nodes.contains_key(&j) {
                    children.push(j);
                }
            }
            let node = Node {
                id: i as usize,
                pathcount: 0,
                children,
            };
            nodes.insert(i, node);
        }
    }
    Graph { nodes }
}

fn day10b(nums: &[i64]) -> i64 {
    let mut graph = make_graph(nums);
    let paths = count_paths(&mut graph, 0);
    paths as i64
}

fn day10a(nums: &[i64]) -> i64 {
    let mut sortednums = nums.to_vec();
    sortednums.push(0);
    sortednums.sort_unstable();
    let maxplus3 = sortednums[sortednums.len() - 1] + 3;
    sortednums.push(maxplus3);
    sortednums.sort_unstable();
    let a = &sortednums[0..sortednums.len() - 1];
    let b = &sortednums[1..sortednums.len()];
    let diffs: Vec<i64> = a.iter().zip(b).map(|(x, y)| y - x).collect();
    let num_ones = diffs.iter().filter(|x| **x == 1).count();
    let num_threes = diffs.iter().filter(|x| **x == 3).count();
    (num_ones * num_threes) as i64
}

pub fn day10(nums: &[i64], part: char) -> i64 {
    match part {
        'a' => day10a(nums),
        'b' => day10b(nums),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use day10::{count_paths, make_graph};

    use crate::day10;
    #[test]
    fn test_case() {
        let input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";

        let input2 = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

        let nums: Vec<i64> = input
            .split('\n')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        let nums2: Vec<i64> = input2
            .split('\n')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        assert_eq!(day10::day10a(&nums), 35);
        assert_eq!(day10::day10a(&nums2), 220);

        let mut g1 = make_graph(&nums);
        let mut g2 = make_graph(&nums2);
        assert_eq!(count_paths(&mut g1, 0), 8);
        assert_eq!(count_paths(&mut g2, 0), 19208);
    }
}
