type Vector2 = (u32, u32);

#[derive(Clone)]
pub struct Ship 
{
	pub name: String,
	length: u32,
	position: Vector2,
	vertical: bool,
}

impl Ship 
{
	pub fn new(name_str: &str, length: u32) -> Ship 
	{
		let name = name_str.to_string();
		Ship { name, length, position: (0, 0), vertical: false }
	} 

	pub fn set_position(mut self, pos: Vector2) 
	{
		self.position = pos;
	}
	pub fn get_position(&self) -> Vector2 { self.position }

	pub fn rotate(&self) -> Ship
	{
		let mut ship = self.clone();
		ship.vertical = !ship.vertical;
		ship
	}
	
	pub fn positions(&self) -> Vec<Vector2>
	{
		match self.vertical 
		{
			false => (self.position.0..(self.position.1 + self.length))
					.map(|x| (x, self.position.1))
					.collect(),
			
			true => (self.position.0..(self.position.1 + self.length))
					.map(|y| (self.position.0, y))
					.collect(),
		} 
	}
	
	pub fn collides(self, other: &Ship) -> bool
	{
		self.positions().iter()
		.any(|item| other.positions().contains(item))
	}	

	pub fn point_hit(&self, position: Vector2) -> bool 
	{
		match self.vertical 
		{
			true => position.0 == self.position.0 &&
					self.position.1 <= position.1 &&
					position.1 < self.position.1 + self.length,
			
			false => position.1 == self.position.1 &&
					self.position.0 <= position.0 &&
					position.0 < self.position.0 + self.length,
		}
	}

	pub fn size(&self) -> Vector2
	{
		if self.vertical {(1, self.length)} else {(self.length, 1)}
	}

	pub fn is_vertical(&self) -> bool { self.vertical }
}

#[cfg(test)]
mod tests {
	use crate::game::ship::*;
	
  #[test]
  fn ship_positions() 
  {
		let ship = Ship::new("destroyer", 5);
		assert_eq!(ship.positions(), 
				   vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);
  }

	#[test]
  	fn colliding() 
	{
		let ship = Ship::new("destroyer", 5);
		ship.set_position((0, 2));
		let ship2 = Ship::new("destroyer", 5);
		ship.set_position((2, 0));
		
		assert!(ship.collides(&ship2));
  	}

	#[test]
  	fn hit_start() 
	{
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((0, 0)));
  	}

	#[test]
  	fn hit_end() 
	{
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((0, 4)));
  	}

	#[test]
	#[should_panic]
  	fn hit_out() 
	{
		let ship = Ship::new("destroyer", 5);
		assert!(ship.point_hit((0, 5)));
  	}
}