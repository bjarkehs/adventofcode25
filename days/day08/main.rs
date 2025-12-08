use std::cmp;
use std::collections::HashMap;

use adventofcode25::{input_path, read_lines};

const DAY: u8 = 8;

fn main() {
    solve_part1(&input_path(DAY), 1000);
    solve_part2(&input_path(DAY));
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

fn solve_part1(input: &str, max_connections: usize) -> u64 {
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
    let distances = distances_between_points(&points);
    let mut sorted_distances: Vec<(&(Vector3, Vector3), &i64)> = distances.iter().collect();
    sorted_distances.sort_unstable_by(|a, b| a.1.cmp(b.1));
    let mut clusters: Vec<u32> = Vec::new();
    let mut cluster_map: HashMap<Vector3, usize> = HashMap::new();
    for i in 0..cmp::min(max_connections, sorted_distances.len()) {
        let ((p1, p2), _) = sorted_distances[i];
        let cluster1 = cluster_map.get(p1);
        let cluster2 = cluster_map.get(p2);

        match (cluster1, cluster2) {
            (None, None) => {
                let cluster_id = clusters.len();
                cluster_map.insert(*p1, cluster_id);
                cluster_map.insert(*p2, cluster_id);
                clusters.push(2);
            }
            (Some(&c1), None) => {
                cluster_map.insert(*p2, c1);
                clusters[c1] += 1;
            }
            (None, Some(&c2)) => {
                cluster_map.insert(*p1, c2);
                clusters[c2] += 1;
            }
            (Some(&c1), Some(&c2)) => {
                if c1 != c2 {
                    let lower = cmp::min(c1, c2);
                    let higher = cmp::max(c1, c2);
                    for (_point, cluster) in cluster_map.iter_mut() {
                        if *cluster == higher {
                            *cluster = lower;
                        }
                    }
                    clusters[lower] += clusters[higher];
                    clusters[higher] = 0;
                }
            }
        }
    }
    println!("Clusters: {:?}", clusters);
    clusters.sort_unstable_by(|a, b| b.cmp(a));
    let result: u32 = clusters.iter().take(3).product();
    println!("Part 1: {}", result);
    result as u64
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

fn solve_part2(input: &str) -> u64 {
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
    let distances = distances_between_points(&points);
    let mut sorted_distances: Vec<(&(Vector3, Vector3), &i64)> = distances.iter().collect();
    sorted_distances.sort_unstable_by(|a, b| a.1.cmp(b.1));
    let mut clusters: Vec<u32> = Vec::new();
    let mut cluster_map: HashMap<Vector3, usize> = HashMap::new();
    let mut result = 0u64;
    for i in 0..sorted_distances.len() {
        let ((p1, p2), _) = sorted_distances[i];
        let cluster1 = cluster_map.get(p1);
        let cluster2 = cluster_map.get(p2);

        match (cluster1, cluster2) {
            (None, None) => {
                let cluster_id = clusters.len();
                cluster_map.insert(*p1, cluster_id);
                cluster_map.insert(*p2, cluster_id);
                clusters.push(2);
            }
            (Some(&c1), None) => {
                cluster_map.insert(*p2, c1);
                clusters[c1] += 1;

                if clusters[c1] == points.len() as u32 {
                    result = (p1.x * p2.x) as u64;
                    println!("All points connected after {} connections", i + 1);
                    break;
                }
            }
            (None, Some(&c2)) => {
                cluster_map.insert(*p1, c2);
                clusters[c2] += 1;
                if clusters[c2] == points.len() as u32 {
                    result = (p1.x * p2.x) as u64;
                    println!("All points connected after {} connections", i + 1);
                    break;
                }
            }
            (Some(&c1), Some(&c2)) => {
                if c1 != c2 {
                    let lower = cmp::min(c1, c2);
                    let higher = cmp::max(c1, c2);
                    for (_point, cluster) in cluster_map.iter_mut() {
                        if *cluster == higher {
                            *cluster = lower;
                        }
                    }
                    clusters[lower] += clusters[higher];
                    clusters[higher] = 0;

                    if clusters[lower] == points.len() as u32 {
                        result = (p1.x * p2.x) as u64;
                        println!("All points connected after {} connections", i + 1);
                        break;
                    }
                }
            }
        }
    }
    println!("Part 2: {}", result);
    result as u64
}

// fn find_parent(cluster_map: &mut HashMap<Vector3, Vector3>, point: &Vector3) -> Vector3 {
//     let parent = cluster_map.get(point).cloned().unwrap_or(*point);
//     if &parent != point {
//         let grandparent = find_parent(cluster_map, &parent);
//         cluster_map.insert(*point, grandparent);
//         grandparent
//     } else {
//         parent
//     }
// }

// fn union_clusters(
//     cluster_map: &mut HashMap<Vector3, Vector3>,
//     rank_map: &mut HashMap<Vector3, usize>,
//     point1: &Vector3,
//     point2: &Vector3,
// ) {
//     let parent1 = find_parent(cluster_map, point1);
//     let parent2 = find_parent(cluster_map, point2);

//     if parent1 == parent2 {
//         return;
//     }

//     let rank1 = rank_map.get(&parent1).cloned().unwrap_or(0);
//     let rank2 = rank_map.get(&parent2).cloned().unwrap_or(0);

//     if rank1 < rank2 {
//         cluster_map.insert(parent1, parent2);
//     } else if rank1 > rank2 {
//         cluster_map.insert(parent2, parent1);
//     } else {
//         cluster_map.insert(parent2, parent1);
//         let rank = rank_map.entry(parent1).or_insert(0);
//         *rank += 1;
//     }
// }

// fn solve(points: Vec<Vector3>, part: u8, max_connections: Option<usize>) -> u64 {
//     let distances = distances_between_points(&points);
//     let mut cluster_map: HashMap<Vector3, Vector3> = HashMap::new();
//     let mut rank_map: HashMap<Vector3, usize> = HashMap::new();
//     let mut cluster_sizes: HashMap<Vector3, usize> = HashMap::new();

//     for point in &points {
//         cluster_map.insert(*point, *point);
//         rank_map.insert(*point, 0);
//         cluster_sizes.insert(*point, 1);
//     }

//     let mut edges: Vec<(&(Vector3, Vector3), &i64)> = distances.iter().collect();
//     edges.sort_unstable_by(|a, b| a.1.cmp(b.1));

//     for (i, ((p1, p2), _)) in edges.iter().enumerate() {
//         let root1 = find_parent(&mut cluster_map, p1);
//         let root2 = find_parent(&mut cluster_map, p2);

//         if root1 != root2 {
//             union_clusters(&mut cluster_map, &mut rank_map, p1, p2);
//             let new_root = find_parent(&mut cluster_map, p1);
//             cluster_sizes.insert(
//                 new_root,
//                 cluster_sizes.get(&root1).unwrap_or(&0) + cluster_sizes.get(&root2).unwrap_or(&0),
//             );

//             if part == 2 {
//                 let size = cluster_sizes.get(&new_root).unwrap_or(&0);
//                 if *size == points.len() {
//                     let result = (p1.x * p2.x) as u64;
//                     return result;
//                 }
//             }

//             if let Some(max) = max_connections {
//                 if i >= max {
//                     break;
//                 }
//             }
//         }
//     }

//     if part == 1 {
//         let largest_clusters: Vec<usize> = cluster_sizes.values().cloned().collect();
//         let mut sorted_sizes = largest_clusters;
//         sorted_sizes.sort_unstable_by(|a, b| b.cmp(a));
//         let result: usize = sorted_sizes.iter().take(3).product();
//         return result as u64;
//     }

//     0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY), 10), 40);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY), 1000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 25272);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
