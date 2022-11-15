use anyhow::Result;
use std::fs;

pub fn p() -> Result<String> {
    /*
    Triangle containment
    Problem 102
    Three distinct points are plotted at random on a Cartesian plane, for which -1000 ≤ x, y ≤ 1000, such that a triangle is
    formed.

    Consider the following two triangles:

    A(-340,495), B(-153,-910), C(835,-947)

    X(-175,41), Y(-421,-714), Z(574,-645)

    It can be verified that triangle ABC contains the origin, whereas triangle XYZ does not.

    Using triangles.txt (right click and 'Save Link/Target As...'), a 27K text file containing the co-ordinates of one thousand
    "random" triangles, find the number of triangles for which the interior contains the origin.

    NOTE: The first two examples in the file represent the triangles in the example given above.
    */

    let data =
        fs::read_to_string("src/problems/data/problem102.txt").expect("Problem opening the file");
    let triangles = data.lines().map(|line| {
        let values: Vec<f64> = line.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
        Triangle(
            Point(values[0], values[1]),
            Point(values[2], values[3]),
            Point(values[4], values[5]),
        )
    });

    let nb_trianles_containing_origin = triangles
        .filter(|triangle| triangle.contains_origin())
        .count();
    Ok(nb_trianles_containing_origin.to_string())
}

#[derive(Debug, Clone, Copy)]
struct Point(f64, f64);

#[derive(Debug)]
struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn contains_origin(&self) -> bool {
        let Triangle(a, b, c) = self;
        let segments = [Segment(*a, *b), Segment(*b, *c), Segment(*c, *a)];
        let segment_cross_origin_with_positive = segments.iter().any(|segment| {
            if let Some(y) = segment.y_at_origin() {
                y > 0.0
            } else {
                false
            }
        });
        let segment_cross_origin_with_negative = segments.iter().any(|segment| {
            if let Some(y) = segment.y_at_origin() {
                y < 0.0
            } else {
                false
            }
        });
        segment_cross_origin_with_positive && segment_cross_origin_with_negative
    }
}

#[derive(Debug, Clone, Copy)]
struct Segment(Point, Point);

impl Segment {
    pub fn y_at_origin(&self) -> Option<f64> {
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;
        let (dx, dy) = (x2 - x1, y2 - y1);
        if dx == 0.0 || !self.cross_origin() {
            None
        } else {
            let m = dy / dx;
            Some(y1 - m * x1)
        }
    }
    fn cross_origin(&self) -> bool {
        let Point(x1, _) = self.0;
        let Point(x2, _) = self.1;
        (x1.is_sign_negative() && x2.is_sign_positive())
            || (x1.is_sign_positive() && x2.is_sign_negative())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "228");
    }
}
