use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    name: &'a str,
}

impl<'a> Vertex<'a> {
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex { name }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

/// Problem 1,
pub fn problem1(input: &str) -> i32 {
    let mut adjacency_list:HashMap<Vertex<'_>, Vec<(Vertex<'_>, usize)>> = HashMap::new();
    //TODO Need to define start first
    for line in input.lines() {
        let (left, distance ) = line.split_once("=").unwrap();
        let distance: i32 = distance.trim().parse().unwrap();
        let (start, destination) = left.split_once("to").unwrap();
        let start = start.trim();
        let destination = destination.trim();

        let vertex = Vertex::new("start");

    }
    // let distances = dijkstra(s, &adjacency_list);
    // for (v, d) in &distances {
    //     println!("name: {}, distance: {}", v.name, d);
    // }
    0
}

/// Problem 2,
pub fn problem2(input: &str) -> i32 {
    todo!()
}

fn dijkstra<'a>(start: Vertex<'a>, adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>, ) -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

// Breath First Search
fn bfs<'a>(start: Vertex<'a>, adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>) -> HashMap<Vertex<'a>, usize> {
    let mut queue = Vec::new();
    // This should be a vec or a datastructure in constant time
    let mut visited: HashMap<Vertex<'a>, bool> = HashMap::new();
    
    visited.insert(start, true);
    queue.push(start);
    
    while ! queue.is_empty() {
        //TODO could also incorperate this into while condition above with some
        let curr = queue.pop().expect("Shouldn't be empty.");
        println!("{:?}",curr);

        for (vertex, cost) in adjacency_list.get(&curr).unwrap() {
            if !visited.contains_key(vertex) {
                visited.insert(*vertex, true);
                queue.push(*vertex);
            }
        }

    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct Solution {
        id: String,
        first: i64,
        second: i64,
    }

    fn get_solution(day: String, problem: i8) -> i64 {
        let json_string =
            fs::read_to_string("data/solutions.json").expect("JSON file doesn't exist!");
        let json: Vec<Solution> =
            serde_json::from_str(&json_string).expect("JSON was not well-formatted");
        let solution = json.iter().find(|x| x.id == day).unwrap();
        return if problem == 1 {
            solution.first
        } else {
            solution.second
        };
    }

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_dijkstra() {
        let s = Vertex::new("s");
        let t = Vertex::new("t");
        let x = Vertex::new("x");
        let y = Vertex::new("y");
        let z = Vertex::new("z");

        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(s, vec![(t, 10), (y, 5)]);
        adjacency_list.insert(t, vec![(y, 2), (x, 1)]);
        adjacency_list.insert(x, vec![(z, 4)]);
        adjacency_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
        adjacency_list.insert(z, vec![(s, 7), (x, 6)]);

        let distances = dijkstra(s, &adjacency_list);

        for (v, d) in &distances {
            println!("name: {}, distance: {}", v.name, d);
        }

        assert_eq!(distances.get(&t), Some(&8));
        assert_eq!(distances.get(&s), Some(&0));
        assert_eq!(distances.get(&y), Some(&5));
        assert_eq!(distances.get(&x), Some(&9));
        assert_eq!(distances.get(&z), Some(&7));
    }

    #[test]
    fn test_dijkstra_aoc() {
        let london = Vertex::new("London");
        let dublin = Vertex::new("Dublin");
        let belfast = Vertex::new("Belfast");

        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(london, vec![(dublin, 464), (belfast, 518)]);
        adjacency_list.insert(dublin, vec![(belfast, 141), (london, 464)]);
        adjacency_list.insert(belfast, vec![(dublin, 141), (london, 518)]);

        let distances = dijkstra(london, &adjacency_list);

        for (v, d) in &distances {
            println!("name: {}, distance: {}", v.name, d);
        }

        assert_eq!(distances.get(&london), Some(&8));
        assert_eq!(distances.get(&dublin), Some(&0));
        assert_eq!(distances.get(&belfast), Some(&5));
    }

    #[test]
    fn test_bfs_aoc() {
        let london = Vertex::new("London");
        let dublin = Vertex::new("Dublin");
        let belfast = Vertex::new("Belfast");

        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(london, vec![(dublin, 464), (belfast, 518)]);
        adjacency_list.insert(dublin, vec![(belfast, 141), (london, 464)]);
        adjacency_list.insert(belfast, vec![(dublin, 141), (london, 518)]);

        let distances = bfs(london, &adjacency_list);

        for (v, d) in &distances {
            println!("name: {}, distance: {}", v.name, d);
        }

        assert_eq!(distances.get(&london), Some(&8));
        assert_eq!(distances.get(&dublin), Some(&0));
        assert_eq!(distances.get(&belfast), Some(&5));
    }
    
    #[test]
    fn test_problem1() {
        // Sample
        let input = fs::read_to_string("data/sample/day_08.txt").expect("Data file doesn't exist!");
        let expected = 12;
        assert_eq!(problem1(&input), expected);
        //Actual
        let input = fs::read_to_string("data/day_08.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day08".to_string(), 1);
        assert_eq!(problem1(&input) as i64, expected);
    }

    #[test]
    fn test_problem2() {
        // Sample
        let input = "^v";
        let expected = 3;
        assert_eq!(problem2(input), expected);
        //Actual
        let input = fs::read_to_string("data/day_06.txt").expect("Data file doesn't exist!");
        let expected = get_solution("day06".to_string(), 2);
        assert_eq!(problem2(&input) as i64, expected);
    }

}
