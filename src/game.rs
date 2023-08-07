pub mod ship;
pub use ship::*;

pub fn default_ships() -> Vec<Ship> {
	return vec![Ship::new("destroyer", 5)]
}

pub fn create_ships() -> Vec<Ship> {
	let s = Ship::new("destroyer", 5);
	vec![s]
}

pub fn position_hits_ship(ships: &Vec<Ship>, pos: (u32, u32)) -> bool {
	for ship in ships {
		if ship.point_hit(pos) { return true; }
	}

	false
}

pub fn all_ships_sunk(ships: &Vec<Ship>, hits: &Vec<Vector2>) -> bool {
	ships.iter().map(|s| s.is_sunk(hits)).all(|x| x)
}