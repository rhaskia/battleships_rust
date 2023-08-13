use gloo_console::log;

pub type Vector2 = (u32, u32);

#[derive(Clone, PartialEq, Debug)]
pub struct Ship 
{
	pub name: String,
	length: u32,
	position: Vector2,
	vertical: bool,
}

impl Ship 
{
	pub fn new(name_str: &str, length: u32) -> Ship {
		let name = name_str.to_string();
		Ship { name, length, position: (0, 0), vertical: false }
	} 

	pub fn set_position(&mut self, pos: Vector2)  { self.position = pos; }

	pub fn get_position(&self) -> Vector2 { self.position }

	pub fn rotate(&self) -> Ship {
		let mut ship = self.clone();
		ship.vertical = !ship.vertical;
		ship
	}
	
	pub fn positions(&self) -> Vec<Vector2> {
		match self.vertical {
			false => (self.position.0..(self.position.0 + self.length))
					.map(|x| (x, self.position.1))
					.collect(),
			
			true => (self.position.1..(self.position.1 + self.length))
					.map(|y| (self.position.0, y))
					.collect(),
		} 
	}
	
	pub fn collides(&self, other: &Ship) -> bool {
		self.positions().iter()
		.any(|item| other.positions().contains(item))
	}	

	pub fn touches(&self, other: &Ship) -> bool {
		let (mut x1, mut y1) = self.get_position();
		if x1 > 0 { x1 -= 1; }
		if y1 > 0 { y1 -= 1; }

		let (mut w1, mut h1) = self.size();
		w1 += 2;
		h1 += 2;

		let (x2, y2) = other.get_position();
		let (w2, h2) = other.size();

		x1 < x2 + w2 &&
		x1 + w1 > x2 &&
		y1 < y2 + h2 &&
		y1 + h1 > y2
	}	

	pub fn point_hit(&self, point: Vector2) -> bool {
		match self.vertical {
			true => point.0 == self.position.0 &&
					point.1 >= self.position.1 &&
					point.1 < self.position.1 + self.length,
			
			false => point.1 == self.position.1 &&
					point.0 >= self.position.0 &&
					point.0 < self.position.0 + self.length,
		}
	}

	pub fn size(&self) -> Vector2 {
		if self.vertical {(1, self.length)} else {(self.length, 1)}
	}

	pub fn is_vertical(&self) -> bool { self.vertical }

	pub fn is_sunk(&self, hits: &Vec<Vector2>) -> bool {
		let sf = self.positions();
		log!(format!("{:?}, {:?}", sf, hits));
		sf.iter().all(|item| hits.contains(item))
	}
}

#[cfg(test)]
mod tests {
	use crate::game::ship::*;
	
  	#[test]
  	fn ship_positions() {
		let ship = Ship::new("destroyer", 5);
		assert_eq!(ship.positions(), 
		vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
  	}

	fn ship_positions_rotated() {
		let ship = Ship::new("destroyer", 5);
		let ship = ship.rotate();
		assert_eq!(ship.positions(), 
		vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);
  	}

	#[test]
  	fn colliding() {
		let mut ship = Ship::new("destroyer", 5);
		ship.set_position((0, 2));
		let mut ship2 = Ship::new("destroyer", 5).rotate();
		ship2.set_position((2, 0));
		
		assert!(ship.collides(&ship2));
  	}

	#[test]
	fn touches() {
		let mut ship = Ship::new("destroyer", 3);
		ship.set_position((0, 0));
		let mut ship2 = Ship::new("destroyer", 5);
		ship2.set_position((0, 1));

		assert!(ship.touches(&ship2));
	}

	#[test]
  	fn hit_start() {
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((0, 0)));
  	}

	#[test]
  	fn hit_end() {
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((4, 0)));
  	}

	#[test]
	#[should_panic]
  	fn hit_out() {
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((5, 0)));
  	}
}