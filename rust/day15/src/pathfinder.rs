use super::creature::{Creature, Race};
use super::point::Point;
use super::OccupiedPoints;
use std::cmp::Ordering;
use std::collections::HashSet;

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

/// Implements Breadth first search
/// `occupied_points` are walls and other unpassable obstacles
pub fn find_best_step(from: &Creature, occupied_points: &OccupiedPoints) -> Option<Point> {
    if adjacent_enemy(&from.position, &from.enemy_race(), occupied_points).is_some() {
        return None;
    }
    let first_moves = from
        .position
        .neighbours()
        .iter()
        .cloned()
        .filter(|p| !occupied_points.contains_key(p))
        .collect::<Vec<_>>();

    let mut best_moves = vec![];

    for point in first_moves.iter() {
        if adjacent_enemy(point, &from.enemy_race(), occupied_points).is_some() {
            best_moves.push((*point, 1, *point));
        }

        let mut seen = HashSet::new();
        seen.insert(from.position);
        seen.insert(*point);

        let mut stack = point
            .neighbours()
            .iter()
            .cloned()
            .filter(|p| !occupied_points.contains_key(p))
            .collect::<Vec<_>>();

        let mut i = 1; // Already moved 1 here
        let mut running = true;
        while running {
            i += 1;

            let mut new_stack = vec![];
            for p in stack.iter() {
                if seen.contains(p) {
                    continue;
                }
                seen.insert(*p);
                if adjacent_enemy(&p, &from.enemy_race(), occupied_points).is_some() {
                    best_moves.push((*point, i, *p));
                    running = false;
                }
                new_stack.append(
                    &mut p
                        .neighbours()
                        .iter()
                        .filter(|p| !occupied_points.contains_key(p) && !seen.contains(p))
                        .cloned()
                        .collect::<Vec<_>>(),
                );
            }
            new_stack.sort_unstable();
            new_stack.dedup();
            stack = new_stack;
            if stack.is_empty() {
                running = false;
            }
        }
    }
    get_best_move(best_moves)
}

pub fn adjacent_enemy(point: &Point, enemy: &Race, map: &OccupiedPoints) -> Option<Point> {
    for neighbour in point.neighbours().iter() {
        if map
            .get(&neighbour)
            .map_or(false, |r| r.clone().map_or(false, |race| race == *enemy))
        {
            return Some(*neighbour);
        }
    }
    None
}

/// best_move tuple: (where it came from, steps took, where it ended up)
fn get_best_move(mut best_moves: Vec<(Point, i32, Point)>) -> Option<Point> {
    if best_moves.is_empty() {
        return None;
    }
    // First condition - fewest number of moves away
    let min_steps = best_moves.iter().map(|(_, i, _)| *i).min().unwrap();
    best_moves.retain(|(_, i, _)| *i == min_steps);

    // Second condition - if tie, choose the first tile in reading order
    best_moves.sort_unstable_by_key(|(_, _, p)| *p);
    let mut first = best_moves[0].2;
    best_moves.retain(|(_, _, p)| *p == first);

    // Third condition - if tie, take the first step in reading order
    best_moves.sort_unstable_by_key(|(p, _, _)| *p);
    first = best_moves[0].0;
    best_moves.retain(|(p, _, _)| *p == first);
    best_moves.get(0).map_or(None, |(p, _, _)| Some(*p))
}

#[cfg(test)]
mod test {
    use super::*;
}

