use std;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::curio::Curio;
use super::room::Room;

const MAX_HP: i32 = 25;

pub enum Command {
    Go(String),
    Shoot(String),
}

pub struct Player {
    pub location: Rc<RefCell<Room>>,
    pub hp: i32,
    pub gold: i32,
    won: bool,
}

impl Player {
    pub fn new(location: Rc<RefCell<Room>>) -> Player {
        Player {
            location: location,
            hp: MAX_HP,
            gold: 0,
            won: false,
        }
    }

    pub fn use_curio(&mut self, curio: Curio) {
        match curio {
            Curio::Chest(gold) => {
                println!("You open the chest and gain {} gold.", gold);
                self.gold += gold;
            },
            Curio::SpikeTrap(dmg) => {
                println!("You take {} damage from the spikes.", dmg);
                self.hp -= dmg;
            },
            Curio::Food(heal) => {
                println!("You shove a wall chicken into your gob and heal {} HP.", heal);
                self.hp = std::cmp::min(MAX_HP, self.hp + heal);
            },
            Curio::IronMaiden(sub, dmg) => {
                println!("Dude I love Iron Maiden! This one's pointy, though.");
                println!("You cut yourself on the spikes inside for {} damage.", dmg);
                self.hp -= dmg;
                println!("You open the iron maiden and...");
                self.use_curio(*sub);
            },
            Curio::FallenAdventurer(sub) => {
                println!("You pilfer the corpse and...");
                self.use_curio(*sub);
            },
        }
    }

    /// Execute the given command on the player and board state.
    pub fn act(&mut self, cmd: Command) -> Result<(), ()> {
        match cmd {
            Command::Go(room_name) => {
                let next_room = try!(self.find_room(room_name));

                // Step into the room
                println!("You step into room {}", next_room.borrow().name);
                while let Some(content) = next_room.borrow_mut().take_content() {
                    self.use_curio(content);
                }

                // Update the location
                self.location = next_room;
            },
            Command::Shoot(room_name) => {
                let target_room = try!(self.find_room(room_name));

                println!("You shoot an arrow into room {}", target_room.borrow().name);
                if target_room.borrow().wumpus {
                    println!("You killed Wumpus!");
                    target_room.borrow_mut().kill_wumpus();
                    self.won = true;
                } else {
                    println!("You missed. There is nothing there.");
                }
            }
        }
        Ok(())
    }

    pub fn is_won(&self) -> bool {
        self.won
    }

    /// Find one of the neighbors of the current room based on its name. Case insensitive.
    fn find_room(&self, room: String) -> Result<Rc<RefCell<Room>>, ()> {
        let room_name = room.to_lowercase();
        let current = self.location.borrow();
        for hall in &current.halls {
            let other_room = hall.other(&current);
            if other_room.borrow().name.to_lowercase() == room_name {
                return Ok(other_room.clone());
            }
        }
        Err(())
    }
}

/**/impl fmt::Display for Player {
/**/    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
/**/        write!(f, "You find yourself in {}.\n\nYou have {} HP and {} gold.",
/**/               self.location.borrow().name, self.hp, self.gold)
/**/    }
/**/}
