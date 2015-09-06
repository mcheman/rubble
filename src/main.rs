mod pathfinder;

fn main() {
    println!("Hello, world!");

    let a = pathfinder::GridNode {x: 130, y: 430};
    let b = pathfinder::GridNode {x: 50, y: 10};

    pathfinder::find_path(a, b);
}
