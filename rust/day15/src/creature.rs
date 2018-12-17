use super::pathfinder::{path_to, Path};
use super::point::Point;
use super::{Creatures, OccupiedPoints};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Creature {
    pub hp: i32,
    pub ap: i32,
    pub race: Race,
    pub position: Point,
}

impl Creature {
    pub fn new(x: i32, y: i32, race: Race) -> Creature {
        Creature {
            hp: 200,
            ap: 3,
            race: race,
            position: Point::new(x, y),
        }
    }

    /// moves and/or attacks
    /// returns if a target was found
    pub fn tick(
        &mut self,
        creatures: &mut Creatures,
        occupied_points: &mut OccupiedPoints,
    ) -> bool {
        let closest_enemy = self.closest_enemy(creatures, occupied_points);
        if closest_enemy.is_none() {
            // check if the reason of not moving is that there are no enemies left
            return creatures
                .iter()
                .any(|creature| creature.race != self.race && creature.hp > 0);
        }

        let (closest_enemy, path) = closest_enemy.unwrap();
        if self.position.dist(&closest_enemy.position) > 1 {
            let step = path.back().unwrap();
            occupied_points.remove(&self.position);
            occupied_points.insert(*step);
            self.position = *step;
        }
        if self.position.dist(&closest_enemy.position) == 1 {
            // is inrange
            closest_enemy.dmg(occupied_points, self.ap);
        }
        true
    }

    pub fn closest_enemy<'a>(
        &self,
        creatures: &'a mut Creatures,
        occupied_points: &OccupiedPoints,
    ) -> Option<(&'a mut Creature, Path)> {
        let creatures = creatures.iter_mut().filter(|c| c.race != self.race);
        find_closest_creature(&self.position, creatures, occupied_points)
    }

    pub fn dmg(&mut self, occupied_points: &mut OccupiedPoints, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 {
            occupied_points.remove(&self.position);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Race {
    Elf,
    Goblin,
}

fn find_closest_creature<'a, I>(
    point: &Point,
    creatures: I,
    occupied_points: &OccupiedPoints,
) -> Option<(&'a mut Creature, Path)>
where
    I: Iterator<Item = &'a mut Creature>,
{
    let mut min = 10_000_000;
    let mut result_path = Path::new();
    let closest = creatures.fold(None, |result, creature| {
        let path = path_to(&creature.position, point, occupied_points);
        if let Some(path) = path {
            let d = path.len();
            if d < min
                || (d == min
                    && d > 0
                    && result_path
                        .front()
                        .map_or(false, |p| *path.front().unwrap() < *p))
                || (d <= 1 && better_target(&result, creature))
            {
                min = d;
                result_path = path;
                Some(creature)
            } else {
                result
            }
        } else {
            result
        }
    });

    closest.map_or(None, |closest| Some((closest, result_path)))
}

fn better_target(base: &Option<&mut Creature>, new: &Creature) -> bool {
    if base.is_none() {
        return true;
    }
    let base = base.as_ref().map(|x| x.clone()).unwrap();
    if new.hp < base.hp || (base.hp == new.hp && new.position < base.position) {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_closeset_enemy() {
        let mut creatures = vec![
            Creature::new(2, 1, Race::Goblin),
            Creature::new(0, 2, Race::Elf),
            Creature::new(2, 2, Race::Goblin),
        ];

        let mut occupied_points = [Point::new(2, 1), Point::new(0, 2), Point::new(2, 2)]
            .iter()
            .map(|x| *x)
            .collect();

        let closest = creatures
            .get_mut(1)
            .unwrap()
            .clone()
            .closest_enemy(&mut creatures, &mut occupied_points)
            .expect("Failed to find the closest enemy");
        assert_eq!(*closest.0, Creature::new(2, 2, Race::Goblin),);

        let closest = creatures
            .get_mut(0)
            .unwrap()
            .clone()
            .closest_enemy(&mut creatures, &mut occupied_points)
            .expect("Failed to find the closest enemy");
        assert_eq!(*closest.0, Creature::new(0, 2, Race::Elf),);
    }

    #[test]
    fn test_chooses_correct_step() {
        let mut creatures = vec![
            Creature::new(2, 1, Race::Goblin),
            Creature::new(0, 2, Race::Elf),
            Creature::new(2, 2, Race::Goblin),
        ];

        let mut occupied_points = [Point::new(2, 1), Point::new(0, 2), Point::new(2, 2)]
            .iter()
            .map(|x| *x)
            .collect();

        let (_closest, path) = creatures
            .get_mut(1)
            .unwrap()
            .clone()
            .closest_enemy(&mut creatures, &mut occupied_points)
            .expect("Failed to find the closest enemy");

        assert_eq!(*path.front().unwrap(), Point::new(1, 2));
    }

    #[test]
    fn test_chooses_correct_step_2() {
        let mut creatures = vec![
            Creature::new(3, 1, Race::Elf),
            Creature::new(6, 1, Race::Goblin),
            Creature::new(1, 2, Race::Goblin),
        ];

        let mut occupied_points = [
            Point::new(3, 1),
            Point::new(6, 1),
            Point::new(1, 3),
            // Walls
            Point::new(2, 3),
            Point::new(3, 2),
            Point::new(2, 2),
        ]
        .iter()
        .map(|x| *x)
        .collect();

        let (_closest, path) = creatures
            .get_mut(0)
            .unwrap()
            .clone()
            .closest_enemy(&mut creatures, &mut occupied_points)
            .expect("Failed to find the closest enemy");

        assert_eq!(*path.back().unwrap(), Point::new(2, 1));
    }
}

