use std::rc::Rc;

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
    pub fn add_hall(&mut self, hall: Rc<Hall>) {
        self.halls.push(hall);
    }

    pub fn take_content(&mut self) -> Option<Curio> {
        self.contents.pop()
    }

    pub fn kill_wumpus(&mut self) {
        self.wumpus = false;
    }

    pub fn neighbors_string(&self) -> String {
        // "There are XX rooms connecting to here"
        // For each room, "one room has XXX, OOO, ###."
        let mut result = String::new();

        result.push('\n');

        for hall in &self.halls {
            let room = hall.other(self);
            result += &format!("- to {}", room.borrow().name);

            // If there is anything in the room
            if !room.borrow().contents.is_empty() {
                result += " contains ";
                for content in &room.borrow().contents {
                    result += &format!("{}, ", content.to_string())
                }
                let len = result.len();
                result.remove(len - 1);
                result.remove(len - 2);
            }

            if room.borrow().wumpus {
                result += ". Wumpus is in the room"
            }
            
            result += ".\n"
        }

        result
    }
}
