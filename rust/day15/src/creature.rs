use super::pathfinder::find_best_step;
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
        if !creatures.iter().any(|c| c.race != self.race && c.hp > 0) {
            return false;
        }
        self.step(occupied_points);
        self.attack(creatures, occupied_points);
        true
    }

    pub fn step(&mut self, occupied_points: &mut OccupiedPoints) {
        let step = find_best_step(self, occupied_points);
        if let Some(step) = step {
            occupied_points.remove(&self.position);
            self.position = step;
            occupied_points.insert(self.position, Some(self.race));
        }
    }

    pub fn attack(&self, creatures: &mut Creatures, occupied_points: &mut OccupiedPoints) {
        let target = creatures
            .iter_mut()
            .filter(|c| c.race == self.enemy_race() && self.inrange(c))
            .fold(None, |result, c| {
                if better_target(&result, c) {
                    Some(c)
                } else {
                    result
                }
            });
        if let Some(target) = target {
            target.dmg(occupied_points, self.ap);
        }
    }

    fn inrange(&self, other: &Creature) -> bool {
        self.position.dist(&other.position) <= 1
    }

    pub fn dmg(&mut self, occupied_points: &mut OccupiedPoints, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 {
            occupied_points.remove(&self.position);
        }
    }

    pub fn enemy_race(&self) -> Race {
        match self.race {
            Race::Elf => Race::Goblin,
            Race::Goblin => Race::Elf,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Race {
    Elf,
    Goblin,
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
    fn test_chooses_correct_step() {
        let mut creature = Creature::new(0, 2, Race::Elf);

        let mut occupied_points = [
            (Point::new(2, 1), Some(Race::Goblin)),
            (Point::new(2, 1), Some(Race::Elf)),
            (Point::new(2, 2), Some(Race::Goblin)),
        ]
        .iter()
        .cloned()
        .collect();

        creature.step(&mut occupied_points);

        assert_eq!(creature.position, Point::new(1, 2));
    }

    #[test]
    fn test_chooses_correct_step_2() {
        let mut creature = Creature::new(3, 1, Race::Elf);

        let mut occupied_points = [
            (Point::new(3, 1), Some(Race::Elf)),
            (Point::new(6, 1), Some(Race::Goblin)),
            (Point::new(1, 3), Some(Race::Goblin)),
            // Walls
            (Point::new(3, 2), None),
            (Point::new(2, 3), None),
            (Point::new(2, 2), None),
        ]
        .iter()
        .cloned()
        .collect();

        creature.step(&mut occupied_points);

        // assert_eq!(creature.position, Point::new(2, 1));
    }
}

