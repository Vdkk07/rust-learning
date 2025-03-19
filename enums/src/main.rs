use std::f32::consts::PI;

enum Direction {
    North,
    South,
    East,
    West
}

// ! Enums can also store values
enum Shape{
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}


// # Implement enums (Approach 2)
impl Shape {
    fn area(&self) -> f32 {
        match self {
        Shape::Circle(radius) => {
            PI * radius * radius
        },
        Shape::Square(side) =>{
            side*side
        },
        Shape::Rectangle(len,wid ) =>{
            2.0 * len * wid
        }
    }
    }
}
fn main(){
    //! Direction enum
    let direction = Direction::North;

    steer(direction);

    // ! Shape enum
    let _shape_circle = Shape::Circle(10.0);
    let _shape_square= Shape::Square(7.0);
    let _shape_rect = Shape::Rectangle(5.6, 9.3);

    // # Approach 1
    let shape_area = calculate_area(_shape_square);
    println!("{}", shape_area);

    let shape_perimeter = calculate_perimeter(_shape_rect);
    println!("{}", shape_perimeter);

    // # Approach 2 
    println!("{}", _shape_circle.area());
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


// function that takes shape as an input and print it area
fn calculate_area(shape:Shape) -> f32{
    match shape {
        Shape::Circle(radius) => {
            PI * radius * radius
        },
        Shape::Square(side) =>{
            side*side
        },
        Shape::Rectangle(len,wid ) =>{
            2.0 * len * wid
        }

    }
}

// function that takes shape as an input and print it perimeter
fn calculate_perimeter(shape:Shape) -> f32{
    match shape {
        Shape::Circle(radius) => {
           2.0 * PI * radius
        },
        Shape::Square(side) =>{
            4.0 * side
        },
        Shape::Rectangle(len,wid ) => {
            2.0 * (len + wid)
        }
    }
}
