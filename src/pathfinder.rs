pub struct GridNode {
	pub x: u32,
	pub y: u32,
}

pub trait Node {
	fn neighbors(&self);
}

pub fn find_path<T: Node>(start: T, goal: T) {
	start.neighbors();
	goal.neighbors();
}

impl Node for GridNode {
	fn neighbors(&self) {
		println!("{} {}", self.x, self.y);
	}
}


#[test]
fn it_finds_path(){

	let a = GridNode {x: 10, y: 40};

	a.neighbors();
}