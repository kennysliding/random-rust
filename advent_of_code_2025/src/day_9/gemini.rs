pub struct SolutionDay9Gemini;
use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl SolutionDay9Gemini {
    fn read_input(filename: &str) -> Vec<Point> {
        let content = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return Vec::new();
            }
        };

        content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() == 2 {
                    Some(Point {
                        x: parts[0].parse().unwrap_or(0),
                        y: parts[1].parse().unwrap_or(0),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn solve_1(filename: &str) -> u64 {
        let points = Self::read_input(filename);
        let n = points.len();
        if n < 2 {
            return 0;
        }

        let mut max_area = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = &points[i];
                let p2 = &points[j];

                let width = (p1.x - p2.x).abs() + 1;
                let height = (p1.y - p2.y).abs() + 1;

                let area = width as u64 * height as u64;
                if area > max_area {
                    max_area = area;
                }
            }
        }

        println!("Part 1: {}", max_area);
        max_area
    }

    pub fn solve_2(filename: &str) -> u64 {
        let points = Self::read_input(filename);
        let n = points.len();
        if n < 4 {
            return 0;
        } // Need at least 4 points to form a closed loop with area

        let mut max_area = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = &points[i];
                let p2 = &points[j];

                let width = (p1.x - p2.x).abs() + 1;
                let height = (p1.y - p2.y).abs() + 1;
                let area = width as u64 * height as u64;

                // Optimization: Don't check validity if smaller than current max
                if area <= max_area {
                    continue;
                }

                // Check if this rectangle is valid (inside polygon)
                if Self::is_valid_rect(p1, p2, &points) {
                    max_area = area;
                }
            }
        }

        println!("Part 2: {}", max_area);
        max_area
    }

    fn is_valid_rect(p1: &Point, p2: &Point, poly: &[Point]) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        // 1. Check if the other two corners are inside or on boundary
        // Corners: (min_x, min_y), (max_x, min_y), (max_x, max_y), (min_x, max_y)
        // Two are p1/p2 (valid by definition), need to check the other two.
        let c3 = (min_x as f64, max_y as f64);
        let c4 = (max_x as f64, min_y as f64);

        if !Self::is_inside_or_boundary(c3, poly) || !Self::is_inside_or_boundary(c4, poly) {
            return false;
        }

        // 2. Check if center is inside or on boundary
        // This prevents cases where corners are valid but rect is outside (e.g. U-shape)
        let center = (
            (min_x as f64 + max_x as f64) / 2.0,
            (min_y as f64 + max_y as f64) / 2.0,
        );
        if !Self::is_inside_or_boundary(center, poly) {
            return false;
        }

        // 3. Check if any polygon edge intersects the INTERIOR of the rectangle
        // If an edge cuts through, the rect is invalid.
        // Interior range: (min_x, max_x) and (min_y, max_y) strictly
        let n = poly.len();
        for i in 0..n {
            let a = &poly[i];
            let b = &poly[(i + 1) % n];

            // Check intersection with strictly interior of Rect
            // Edge is either vertical or horizontal
            if a.x == b.x {
                // Vertical Edge
                let edge_x = a.x;
                let edge_y_min = a.y.min(b.y);
                let edge_y_max = a.y.max(b.y);

                // Does edge X fall strictly inside rect X range?
                if edge_x > min_x && edge_x < max_x {
                    // Does edge Y overlap strictly with rect Y range?
                    // We check if intervals (edge_y_min, edge_y_max) and (min_y, max_y) overlap
                    if Self::intervals_overlap(edge_y_min, edge_y_max, min_y, max_y) {
                        return false;
                    }
                }
            } else {
                // Horizontal Edge
                let edge_y = a.y;
                let edge_x_min = a.x.min(b.x);
                let edge_x_max = a.x.max(b.x);

                if edge_y > min_y && edge_y < max_y {
                    if Self::intervals_overlap(edge_x_min, edge_x_max, min_x, max_x) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn intervals_overlap(start1: i64, end1: i64, start2: i64, end2: i64) -> bool {
        // Check if two ranges (start, end) have any common values
        // Note: For intersection check, we care about the open intervals inside the rect.
        // The rect interval is (start2, end2). The edge interval is [start1, end1].
        // Overlap if max(start1, start2) < min(end1, end2)
        start1.max(start2) < end1.min(end2)
    }

    fn is_inside_or_boundary(p: (f64, f64), poly: &[Point]) -> bool {
        // 1. Check boundary explicitly
        let n = poly.len();
        for i in 0..n {
            let p1 = &poly[i];
            let p2 = &poly[(i + 1) % n];
            if Self::is_on_segment(p, p1, p2) {
                return true;
            }
        }

        // 2. Ray Casting for interior check
        let (px, py) = p;
        let mut inside = false;

        for i in 0..n {
            let p1 = &poly[i];
            let p2 = &poly[(i + 1) % n];

            let y1 = p1.y as f64;
            let y2 = p2.y as f64;

            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // Check if ray at py crosses the vertical range of this edge
            // Use [min, max) to handle vertices
            if py >= min_y && py < max_y {
                let x1 = p1.x as f64;
                let x2 = p2.x as f64;

                // Calculate intersection X
                // For rectilinear, this is simple, but general form works too
                let intersect_x = x1 + (py - y1) * (x2 - x1) / (y2 - y1);

                if intersect_x > px {
                    inside = !inside;
                }
            }
        }
        inside
    }

    fn is_on_segment(p: (f64, f64), a: &Point, b: &Point) -> bool {
        let (px, py) = p;
        let ax = a.x as f64;
        let ay = a.y as f64;
        let bx = b.x as f64;
        let by = b.y as f64;

        let min_x = ax.min(bx);
        let max_x = ax.max(bx);
        let min_y = ay.min(by);
        let max_y = ay.max(by);

        // Epsilon for float comparison
        let eps = 1e-9;

        if px < min_x - eps || px > max_x + eps || py < min_y - eps || py > max_y + eps {
            return false;
        }

        // For rectilinear, cross product check essentially checks alignment
        // (bx - ax) * (py - ay) - (px - ax) * (by - ay) == 0
        let cross = (bx - ax) * (py - ay) - (px - ax) * (by - ay);
        cross.abs() < eps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let result = SolutionDay9Gemini::solve_1("src/day_9/input_test.txt");
        assert_eq!(result, 50);
    }

    #[test]
    fn test_solve_2() {
        let result = SolutionDay9Gemini::solve_2("src/day_9/input_test.txt");
        assert_eq!(result, 24);
    }

    #[test]
    fn test_solve_1_real() {
        // verified by Advent of Code
        let result = SolutionDay9Gemini::solve_1("src/day_9/input.txt");
        assert_eq!(result, 4748769124);
    }

    #[test]
    fn test_solve_2_real() {
        // verified by Advent of Code
        let result = SolutionDay9Gemini::solve_2("src/day_9/input.txt");
        assert_eq!(result, 1525991432);
    }
}
