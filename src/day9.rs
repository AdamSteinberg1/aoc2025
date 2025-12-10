use crate::solution::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [x, y] = s
            .split(',')
            .map(str::parse)
            .collect_array()
            .with_context(|| format!("failed to parse point {}", s))?;
        Ok(Self { x: x?, y: y? })
    }
}

struct Rectangle {
    //defined by two opposite corners
    small_corner: Point, // the smaller x and y (closer to origin)
    big_corner: Point,   // the bigger x and y (farther from origin)
}

impl Rectangle {
    fn new(p1: &Point, p2: &Point) -> Self {
        let small_corner = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let big_corner = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));
        Self {
            small_corner,
            big_corner,
        }
    }

    fn intersects_segment(&self, l1: &Point, l2: &Point) -> bool {
        //returns true if self intersects the line segment formed by l1 and l2
        let line_left = l1.x.min(l2.x);
        let line_right = l1.x.max(l2.x);
        let line_bottom = l1.y.min(l2.y);
        let line_top = l1.y.max(l2.y);

        let rect_left = self.small_corner.x;
        let rect_right = self.big_corner.x;
        let rect_bottom = self.small_corner.y;
        let rect_top = self.big_corner.y;

        // Check if they're separated on the x-axis
        let separated_horizontally = line_right <= rect_left || rect_right <= line_left;

        // Check if they're separated on the y-axis
        let separated_vertically = line_top <= rect_bottom || rect_top <= line_bottom;

        // They intersect if they're not separated on either axis
        !separated_horizontally && !separated_vertically
    }

    fn is_valid(&self, perimeter: &[Point]) -> bool {
        //a rectangle is valid if it only contains red and green tiles
        //that is equivalent to the rectangle not intersecting the perimeter
        perimeter
            .iter()
            .circular_tuple_windows()
            .all(|(l1, l2)| !self.intersects_segment(l1, l2))
    }

    fn area(&self) -> usize {
        let length = self.big_corner.x - self.small_corner.x + 1;
        let width = self.big_corner.y - self.small_corner.y + 1;
        length * width
    }
}

fn parse_points(input: &str) -> Result<Vec<Point>> {
    input.lines().map(str::parse).try_collect()
}

fn possible_rectangles(points: &[Point]) -> impl Iterator<Item = Rectangle> {
    points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| Rectangle::new(p1, p2))
}

#[derive(Default)]
pub struct Day9;
impl Solution for Day9 {
    type Part1Output = usize;
    fn part1(&self, input: &str) -> Result<Self::Part1Output> {
        let points = parse_points(input)?;
        possible_rectangles(&points)
            .map(|rectangle| rectangle.area())
            .max()
            .context("no points found")
    }

    type Part2Output = usize;
    fn part2(&self, input: &str) -> Result<Self::Part2Output> {
        let points = parse_points(input)?;
        possible_rectangles(&points)
            .filter(|rectangle| rectangle.is_valid(&points))
            .map(|rectangle| rectangle.area())
            .max()
            .context("unable to find valid rectangle")
    }
}
