pub mod ship;
pub use ship::*;
pub use rand::*;
pub mod rand;

pub fn default_ships() -> Vec<Ship> {
	return vec![Ship::new("destroyer", 5)]
}

pub fn random_ship_pos(ship: &Ship) -> Vector2 {
	(js_rand(0, 10), js_rand(0, 10))
}

pub fn create_ships() -> Vec<Ship> {
	let mut ships_to_place = vec![
		Ship::new("Carrier", 5),
		Ship::new("Battleship", 4),
		Ship::new("Cruiser", 3),
		Ship::new("Submarine", 3),
		Ship::new("Destroyer", 2),
	];

	let mut placed_ships = Vec::new();

	while let Some(mut ship) = ships_to_place.pop() {
		ship.set_position(random_ship_pos(&ship));
		placed_ships.push(ship);
	}

	placed_ships
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