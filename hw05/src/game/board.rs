use std::cell::RefCell;
use std::rc::Rc;
use std::result;
use std::io::Read;

use serde_json;
use serde_json::{Value, Error};

use super::curio::Curio;
use super::hall::Hall;
use super::room::Room;

pub type Result<T> = result::Result<T, String>;

pub struct Board {
    pub rooms: Vec<Rc<RefCell<Room>>>,
}

impl Board {
    pub fn build_board(reader: &mut Read) -> Result<Board> {
        let mut board = Board { rooms: Vec::new(), };

        let board_json: Value = try!(serde_json::from_reader(reader).map_err(|_| "Unable to create JSON reader".to_string()));

        try!(board.parse_rooms(&board_json).map_err(|_| "Unable to parse rooms".to_string()));
        try!(board.parse_halls(&board_json).map_err(|_| "Unable to parse halls".to_string()));

        Ok(board)
    }

    fn parse_rooms(&mut self, json: &Value) -> Result<()> {
        // Parse each room
        let rooms = try!(json["rooms"].as_array().ok_or("Unable to parse rooms".to_string()));
        for r in rooms {

            // Parse room name
            let name: &str = try!(r["name"].as_str().ok_or("Unable to parse name".to_string()));

            // Parse curios
            let n: u64 = try!(r["curios"].as_u64().ok_or("Unable to parse curio".to_string()));
            let curios: Vec<Curio> = Curio::generate_n(n as usize);

            // Wumpus?
            let wumpus: bool = {
                if let Value::Bool(wumpus) = r["wumpus"] {
                    wumpus
                } else {
                    false
                }
            };

            // Add the new room to self.rooms
            self.rooms.push(Rc::new(RefCell::new(Room {
                name: name.to_string(),
                contents: curios,
                halls: vec![],
                wumpus: wumpus
            })));
        }
        Ok(())
    }

    fn parse_halls(&mut self, json: &Value) -> Result<()> {
        // Find hall list
        let halls = try!(json["halls"].as_array().ok_or("Unable to parse halls".to_string()));

        // Parse each hall
        for h in halls {
            let h = try!(h.as_array().ok_or("Unable to parse halls".to_string()));
            if h.len() > 2 { return Err("Invalid number of rooms per hall".to_string()); }

            // Add room links to halls
            let r1 = try!(h[0].as_u64().ok_or("Unable to parse room id".to_string())) as usize;
            let r2 = try!(h[1].as_u64().ok_or("Unable to parse room id".to_string())) as usize;
            let hall = Hall::new(self.rooms[r1].clone(), self.rooms[r2].clone());

            // Add hall links to rooms
            let hall = Rc::new(hall);
            self.rooms[r1].borrow_mut().add_hall(hall.clone());
            self.rooms[r2].borrow_mut().add_hall(hall.clone());
        }
        Ok(())
    }

    pub fn spawn_location(&self) -> Rc<RefCell<Room>> {
        self.rooms[0].clone()
    }
}
