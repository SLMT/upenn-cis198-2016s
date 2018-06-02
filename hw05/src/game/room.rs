use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.
    pub fn add_hall(&mut self, hall: Rc<Hall>) {
        self.halls.push(hall);
    }

    pub fn neighbors_string(&self) -> String {
        // TODO: Implement
        unimplemented!();
    }
}
