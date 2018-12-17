use super::point::Point;
use super::OccupiedPoints;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub type Path = VecDeque<Point>;

#[derive(Debug, Ord, Eq, Clone)]
pub struct Node {
    pub point: Point,
    pub gcost: u32,
    pub hcost: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        let other = other.gcost + other.hcost;
        let this = self.gcost + self.hcost;

        this == other
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        let rhs = other.gcost + other.hcost;
        let lhs = self.gcost + self.hcost;

        Some(rhs.cmp(&lhs).then(self.point.cmp(&other.point)))
    }
}

/// Implements A*
/// `occupied_points` are walls and other unpassable obstacles
pub fn path_to(from: &Point, to: &Point, occupied_points: &OccupiedPoints) -> Option<Path> {
    let mut closed = HashSet::with_capacity(100);
    let mut came_from = HashMap::with_capacity(100);
    let mut open = BinaryHeap::with_capacity(100);
    open.push(Node {
        point: *from,
        gcost: 0,
        hcost: from.dist(to),
    });

    let mut tries = 0;
    while !open.is_empty() && tries <= 1000 {
        tries += 1;
        let current = open.pop().unwrap();
        if current.point.dist(to) == 1 {
            return Some(reconstruct_path(came_from, current.point));
        }

        closed.insert(current.point);
        current.point.neighbours().iter().for_each(|neighbour| {
            if closed.contains(&neighbour) || occupied_points.contains(&neighbour) {
                return;
            }
            let gcost = current.gcost + 1;
            let existing = open.iter().find_map(|node| {
                if node.point == *neighbour {
                    Some(node)
                } else {
                    None
                }
            });
            if let Some(existing) = existing {
                if existing.gcost <= gcost
                {
                    return;
                }
                // Remove existing from the heap
                let v = open
                    .iter()
                    .filter(|node| node.point != *neighbour)
                    .cloned()
                    .collect::<Vec<_>>();
                open = BinaryHeap::from(v);
            }
            open.push(Node {
                point: *neighbour,
                gcost: gcost,
                hcost: neighbour.dist(to),
            });
            came_from.insert(*neighbour, current.point);
        });
    }
    None
}

fn reconstruct_path(mut came_from: HashMap<Point, Point>, mut current: Point) -> Path {
    let mut result = Path::new();
    while came_from.contains_key(&current) {
        let (to, from) = came_from.remove_entry(&current).unwrap();
        current = from;
        result.push_front(to);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let start = Point::new(0, 0);
        let end = Point::new(1, 2);
        let occupied = [Point::new(0, 0), Point::new(1, 2)]
            .iter()
            .cloned()
            .collect();

        let result = path_to(&start, &end, &occupied).expect("Failed to find path");

        assert_eq!(result, vec![Point::new(1, 0), Point::new(1, 1),]);
    }

    #[test]
    fn test_unreachable() {
        let start = Point::new(0, 0);
        let end = Point::new(1, 2);
        let occupied = [
            Point::new(0, 0),
            Point::new(1, 2),
            Point::new(0, 2),
            Point::new(2, 2),
            Point::new(1, 1),
            Point::new(1, 3),
        ]
        .iter()
        .cloned()
        .collect();

        let result = path_to(&start, &end, &occupied);

        assert!(result.is_none());
    }

    #[test]
    fn test_shortest_path_with_obstacles() {
        let start = Point::new(0, 0);
        let end = Point::new(1, 2);
        let occupied = [
            Point::new(0, 0),
            Point::new(1, 2),
            Point::new(0, 2),
            Point::new(1, 1),
            Point::new(1, 3),
        ]
        .iter()
        .cloned()
        .collect();

        let result = path_to(&start, &end, &occupied).expect("Failed to find shortest path");
        assert_eq!(
            result,
            vec![
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(2, 1),
                Point::new(2, 2),
            ]
        );
    }
}

