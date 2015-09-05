// loops over actions, executing them
use std::collections::VecDeque;

use actions::Actions;

pub fn game_loop() {
	let mut action_queue = VecDeque::new();

	action_queue.push_back(Actions::Drill);
	action_queue.push_back(Actions::DestroyWall);
	action_queue.push_back(Actions::Drill);
	action_queue.push_back(Actions::Quit);

	let mut running = true;

	while (running) {
		if let Some(action) = action_queue.pop_front() {
			match action {
				Actions::DestroyWall => println!("destroy"),
				Actions::Drill => println!("drill"),
				Actions::Quit => running = false,
			};
		}
	}


}
