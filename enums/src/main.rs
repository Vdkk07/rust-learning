enum Direction {
    North,
    South,
    East,
    West
}

fn main(){
    let direction = Direction::North;

    steer(direction);
}


fn steer(dir:Direction){
    match dir {
        Direction::East => println!("East direction"),
        Direction::West => println!("West direction"),
        // Direction::North => println!("North direction"),
        // Direction::South => println!("South direction"),

        // ? Catch of other cases
        _ => println!("Horizontal direction"),

    }
}