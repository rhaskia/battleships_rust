type Vector2 = (u32, u32);

pub struct Ship 
{
	pub name: String,
	pub length: u32,
	position: Vector2,
	vertical: bool,
}

impl Ship 
{
	pub fn new(name_str: &str, length: u32, 
				 position: Vector2, vertical: bool) -> Ship 
	{
			let name = name_str.to_string();
			Ship { name, length, position, vertical }
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

	pub fn point_hit(self, position: Vector2) -> bool 
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
}

#[cfg(test)]
mod tests {
	use crate::game::ship::*;
	
  #[test]
  fn ship_positions() 
	{
		let ship = Ship::new("destroyer", 5, (0, 0), true);
		assert_eq!(ship.positions(), 
							 vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);
  }

	#[test]
  fn colliding() 
	{
		let ship = Ship::new("destroyer", 5, (2, 0), true);
		let ship2 = Ship::new("destroyer", 5, (0, 2), false);
		
		assert!(ship.collides(&ship2));
  }

	#[test]
  fn hit_start() 
	{
		let ship = Ship::new("destroyer", 5, (0, 0), true);
		assert!(ship.point_hit((0, 0)));
  }

	#[test]
  fn hit_end() 
	{
		let ship = Ship::new("destroyer", 5, (0, 0), true);
		assert!(ship.point_hit((0, 4)));
  }

	#[test]
	#[should_panic]
  fn hit_out() 
	{
		let ship = Ship::new("destroyer", 5, (0, 0), true);
		assert!(ship.point_hit((0, 5)));
  }
}