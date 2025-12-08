use std::collections::HashMap;

use adventofcode25::{input_path, read_lines};

const DAY: u8 = 8;

fn main() {
    let points = parse_input(&input_path(DAY));
    solve_part1(&points, 1000);
    solve_part2(&points);
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
    let lines = read_lines(input).expect("Failed to read input file");
    let mut points: Vec<Vector3> = Vec::new();
    for line in lines.map_while(Result::ok) {
        let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        points.push(Vector3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }
    points
}

fn solve_part1(points: &Vec<Vector3>, max_connections: usize) -> u64 {
    solve(points.clone(), 1, Some(max_connections))
}

fn distances_between_points(points: &Vec<Vector3>) -> HashMap<(Vector3, Vector3), i64> {
    let mut distances: HashMap<(Vector3, Vector3), i64> = HashMap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].distance_squared_to(&points[j]);
            distances.insert((points[i], points[j]), dist);
        }
    }

    distances
}

fn solve_part2(points: &Vec<Vector3>) -> u64 {
    solve(points.clone(), 2, None)
}

fn find_parent(cluster_map: &mut HashMap<Vector3, Vector3>, point: &Vector3) -> Vector3 {
    let parent = cluster_map.get(point).cloned().unwrap_or(*point);
    if &parent != point {
        let grandparent = find_parent(cluster_map, &parent);
        cluster_map.insert(*point, grandparent);
        grandparent
    } else {
        parent
    }
}

fn union_clusters(
    cluster_map: &mut HashMap<Vector3, Vector3>,
    rank_map: &mut HashMap<Vector3, usize>,
    point1: &Vector3,
    point2: &Vector3,
) {
    let parent1 = find_parent(cluster_map, point1);
    let parent2 = find_parent(cluster_map, point2);

    if parent1 == parent2 {
        return;
    }

    let rank1 = rank_map.get(&parent1).cloned().unwrap_or(0);
    let rank2 = rank_map.get(&parent2).cloned().unwrap_or(0);

    if rank1 < rank2 {
        cluster_map.insert(parent1, parent2);
    } else if rank1 > rank2 {
        cluster_map.insert(parent2, parent1);
    } else {
        cluster_map.insert(parent2, parent1);
        let rank = rank_map.entry(parent1).or_insert(0);
        *rank += 1;
    }
}

fn solve(points: Vec<Vector3>, part: u8, max_connections: Option<usize>) -> u64 {
    let distances = distances_between_points(&points);
    let mut cluster_map: HashMap<Vector3, Vector3> = HashMap::new();
    let mut rank_map: HashMap<Vector3, usize> = HashMap::new();
    let mut cluster_sizes: HashMap<Vector3, usize> = HashMap::new();

    for point in &points {
        cluster_map.insert(*point, *point);
        rank_map.insert(*point, 0);
        cluster_sizes.insert(*point, 1);
    }

    let mut edges: Vec<(&(Vector3, Vector3), &i64)> = distances.iter().collect();
    edges.sort_unstable_by(|a, b| a.1.cmp(b.1));

    for (i, ((p1, p2), _)) in edges.iter().enumerate() {
        let root1 = find_parent(&mut cluster_map, p1);
        let root2 = find_parent(&mut cluster_map, p2);

        if root1 != root2 {
            union_clusters(&mut cluster_map, &mut rank_map, p1, p2);
            let new_root = find_parent(&mut cluster_map, p1);
            cluster_sizes.insert(
                new_root,
                cluster_sizes.get(&root1).unwrap_or(&0) + cluster_sizes.get(&root2).unwrap_or(&0),
            );

            if part == 2 {
                let size = cluster_sizes.get(&new_root).unwrap_or(&0);
                if *size == points.len() {
                    let result = (p1.x * p2.x) as u64;
                    println!("Part 2: {result}");
                    return result;
                }
            }

            if let Some(max) = max_connections {
                if i + 1 >= max {
                    break;
                }
            }
        }
    }

    if part == 1 {
        let mut final_sizes: Vec<usize> = Vec::new();
        for point in &points {
            let root = find_parent(&mut cluster_map, point);
            if root == *point {
                final_sizes.push(*cluster_sizes.get(&root).unwrap_or(&0));
            }
        }
        final_sizes.sort_unstable_by(|a, b| b.cmp(a));
        let result: usize = final_sizes.iter().take(3).product();
        println!("Part 1: {result}");
        return result as u64;
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
