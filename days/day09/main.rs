use adventofcode25::{input_path, read_lines};

const DAY: u8 = 9;

fn main() {
    solve_part1(&input_path(DAY));
    solve_part2(&input_path(DAY));
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn subtract(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn cross(&self, other: &Point) -> i64 {
        self.x * other.y - self.y * other.x
    }
}

struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn is_rectangle_of_points_inside(&self, point1: &Point, point2: &Point) -> bool {
        let min_x = point1.x.min(point2.x);
        let max_x = point1.x.max(point2.x);
        let min_y = point1.y.min(point2.y);
        let max_y = point1.y.max(point2.y);

        let rectangle_corners = vec![
            Point { x: min_x, y: min_y },
            Point { x: min_x, y: max_y },
            Point { x: max_x, y: min_y },
            Point { x: max_x, y: max_y },
        ];

        for corner in rectangle_corners {
            if !self.is_point_inside(&corner) {
                return false;
            }
        }

        let segments = vec![
            (Point { x: min_x, y: min_y }, Point { x: min_x, y: max_y }),
            (Point { x: min_x, y: max_y }, Point { x: max_x, y: max_y }),
            (Point { x: max_x, y: max_y }, Point { x: max_x, y: min_y }),
            (Point { x: max_x, y: min_y }, Point { x: min_x, y: min_y }),
        ];

        for segment in segments {
            if !self.segment_inside(&segment.0, &segment.1) {
                return false;
            }
        }
        true
    }

    fn is_point_inside(&self, point: &Point) -> bool {
        let mut inside = false;

        for i in 0..self.points.len() {
            let edge_start = &self.points[i];
            let edge_end = &self.points[(i + 1) % self.points.len()];

            if self.is_point_on_edge(point, edge_start, edge_end) {
                return true;
            }

            if (edge_start.y > point.y) != (edge_end.y > point.y) {
                let cross_product = (edge_end.x - edge_start.x) * (point.y - edge_start.y)
                    - (edge_end.y - edge_start.y) * (point.x - edge_start.x);

                if (cross_product > 0) == (edge_end.y > edge_start.y) {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn is_point_on_edge(&self, point: &Point, edge_start: &Point, edge_end: &Point) -> bool {
        if edge_start.x == edge_end.x {
            if point.x == edge_start.x
                && point.y >= edge_start.y.min(edge_end.y)
                && point.y <= edge_start.y.max(edge_end.y)
            {
                return true;
            }
        } else if edge_start.y == edge_end.y {
            if point.y == edge_start.y
                && point.x >= edge_start.x.min(edge_end.x)
                && point.x <= edge_start.x.max(edge_end.x)
            {
                return true;
            }
        }
        false
    }

    fn largest_area(&self) -> u64 {
        let mut largest_area = 0;
        for i in 0..self.points.len() - 1 {
            let point1 = &self.points[i];
            for j in i + 1..self.points.len() {
                let point2 = &self.points[j];
                if self.is_rectangle_of_points_inside(point1, point2) {
                    let area =
                        (point1.x.abs_diff(point2.x) + 1) * (point1.y.abs_diff(point2.y) + 1);
                    if area > largest_area {
                        largest_area = area;
                    }
                }
            }
        }
        largest_area
    }

    fn segment_inside(&self, a: &Point, b: &Point) -> bool {
        for i in 0..self.points.len() {
            let c = &self.points[i];
            let d = &self.points[(i + 1) % self.points.len()];

            if self.segments_properly_intersect(a, b, c, d) {
                return false;
            }
        }
        true
    }

    fn segments_properly_intersect(&self, a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
        let o1 = self.orientation(a, b, c);
        let o2 = self.orientation(a, b, d);
        let o3 = self.orientation(c, d, a);
        let o4 = self.orientation(c, d, b);

        (o1 * o2 < 0) && (o3 * o4 < 0)
    }

    fn orientation(&self, p: &Point, q: &Point, r: &Point) -> i8 {
        let val = q.subtract(p).cross(&r.subtract(p));
        if val > 0 {
            1
        } else if val < 0 {
            -1
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    read_lines(input)
        .expect("Failed to read input file")
        .map_while(Result::ok)
        .map(|line| {
            let coords = line
                .split(',')
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect::<Vec<i64>>();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> u64 {
    let points = parse_input(input);
    let mut largest_area = 0;
    for i in 0..points.len() - 1 {
        let point1 = &points[i];
        for j in i + 1..points.len() {
            let point2 = &points[j];
            let area = (point1.x.abs_diff(point2.x) + 1) * (point1.y.abs_diff(point2.y) + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    let result = largest_area;
    println!("Part 1: {}", result);
    result
}

fn solve_part2(input: &str) -> u64 {
    let points = parse_input(input);
    let polygon = Polygon { points };

    let result = polygon.largest_area();
    println!("Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode25::example_path;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&example_path(DAY)), 50);
    }

    #[test]
    fn part1_real() {
        solve_part1(&input_path(DAY));
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&example_path(DAY)), 24);
    }

    #[test]
    fn part2_real() {
        solve_part2(&input_path(DAY));
    }
}
