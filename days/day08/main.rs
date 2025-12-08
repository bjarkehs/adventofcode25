use std::collections::HashMap;

use adventofcode25::{input_path, read_lines};

const DAY: u8 = 8;

fn main() {
    let points = parse_input(&input_path(DAY));
    println!("Part 1: {}", solve_part1(&points, 1000));
    println!("Part 2: {}", solve_part2(&points));
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn distance_squared_to(&self, other: &Vector3) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse_input(input: &str) -> Vec<Vector3> {
    read_lines(input)
        .expect("Failed to read input file")
        .map_while(Result::ok)
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Vector3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

fn compute_sorted_edges(points: &[Vector3]) -> Vec<(Vector3, Vector3, i64)> {
    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].distance_squared_to(&points[j]);
            edges.push((points[i], points[j], dist));
        }
    }
    edges.sort_unstable_by_key(|(_, _, dist)| *dist);
    edges
}

struct UnionFind {
    parent: HashMap<Vector3, Vector3>,
    rank: HashMap<Vector3, usize>,
    size: HashMap<Vector3, usize>,
}

impl UnionFind {
    fn new(points: &[Vector3]) -> Self {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for &p in points {
            parent.insert(p, p);
            size.insert(p, 1);
        }
        Self {
            parent,
            rank: HashMap::new(),
            size,
        }
    }

    fn find(&mut self, point: Vector3) -> Vector3 {
        let p = *self.parent.get(&point).unwrap();
        if p != point {
            let root = self.find(p);
            self.parent.insert(point, root);
            root
        } else {
            point
        }
    }

    fn union(&mut self, p1: Vector3, p2: Vector3) -> bool {
        let root1 = self.find(p1);
        let root2 = self.find(p2);

        if root1 == root2 {
            return false;
        }

        let rank1 = *self.rank.get(&root1).unwrap_or(&0);
        let rank2 = *self.rank.get(&root2).unwrap_or(&0);
        let size1 = *self.size.get(&root1).unwrap();
        let size2 = *self.size.get(&root2).unwrap();

        let (new_root, old_root) = if rank1 < rank2 {
            (root2, root1)
        } else {
            if rank1 == rank2 {
                *self.rank.entry(root1).or_insert(0) += 1;
            }
            (root1, root2)
        };

        self.parent.insert(old_root, new_root);
        self.size.insert(new_root, size1 + size2);
        true
    }

    fn get_size(&mut self, point: Vector3) -> usize {
        let root = self.find(point);
        *self.size.get(&root).unwrap()
    }

    fn get_circuit_sizes(&mut self, points: &[Vector3]) -> Vec<usize> {
        points
            .iter()
            .filter_map(|&p| {
                let root = self.find(p);
                if root == p {
                    Some(*self.size.get(&root).unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}

fn solve_part1(points: &[Vector3], max_connections: usize) -> u64 {
    let edges = compute_sorted_edges(points);
    let mut uf = UnionFind::new(points);

    for (p1, p2, _) in edges.iter().take(max_connections) {
        uf.union(*p1, *p2);
    }

    let mut sizes = uf.get_circuit_sizes(points);
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product::<usize>() as u64
}

fn solve_part2(points: &[Vector3]) -> u64 {
    let edges = compute_sorted_edges(points);
    let mut uf = UnionFind::new(points);

    for (p1, p2, _) in &edges {
        if uf.union(*p1, *p2) && uf.get_size(*p1) == points.len() {
            return (p1.x * p2.x) as u64;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        let points = parse_input(&example_path(DAY));
        assert_eq!(solve_part1(&points, 10), 40);
    }

    #[test]
    fn part1_real() {
        let points = parse_input(&input_path(DAY));
        solve_part1(&points, 1000);
    }

    #[test]
    fn part2_example() {
        let points = parse_input(&example_path(DAY));
        assert_eq!(solve_part2(&points), 25272);
    }

    #[test]
    fn part2_real() {
        let points = parse_input(&input_path(DAY));
        solve_part2(&points);
    }
}
