pub mod ship;

pub fn create_ships() 
{
	let s = ship::Ship::new("destroyer", 5, (4, 5), true);
}