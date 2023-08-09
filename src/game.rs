pub mod ship;
pub use ship::*;
use crate::gui::Vector2;

pub fn default_ships() -> Vec<Ship> 
{
	return vec![Ship::new("destroyer", 5)]
}

pub fn create_ships() -> Vec<Ship> {
	let ships_to_place = vec![
		Ship::new("Carrier", 5),
		Ship::new("Battleship", 4),
		Ship::new("Cruiser", 3),
		Ship::new("Submarine", 3),
		Ship::new("Destroyer", 2),
	];
	
	ships_to_place
}

pub fn position_hits_ship(ships: &Vec<Ship>, pos: (u32, u32)) -> bool
{
	for ship in ships 
	{
		if ship.point_hit(pos) { return true; }
	}

	false
}

