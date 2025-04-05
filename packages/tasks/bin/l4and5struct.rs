use tasks::l4and5struct_task::{Point, Shape};

fn main() {
    let point1 = Point::symetric(1);
    let point2 = Point { x: 5, y: 7 };

    println!(
        "The area of the rectangle is {} square pixels.",
        point1.area(&point2, &Shape::Rectangle)
    );

    println!(
        "The area of the circle is {} square pixels.",
        point1.area(&point2, &Shape::Circle)
    );

    println!(
        "The distance between the two points is {} pixels.",
        point1.distance(&point2)
    );

    println!("point1 is {:#?}", point1);

    match point1.divide_x(0.0) {
        Some(val) => println!("point1 divided by 2 is {}", val),
        None => println!("Cannot divide by zero!"),
    }

    match point1.divide_x(2.0) {
        Some(val) => println!("point1 divided by 2 is {}", val),
        None => println!("Cannot divide by zero!"),
    }

    match Point::parse("123, 321") {
        Ok(point) => println!("Parsed point is {:#?}", point),
        Err(e) => println!("Error parsing point: {:?}", e),
    }

    match Point::parse("123, 321, 321") {
        Ok(point) => println!("Parsed point is {:#?}", point),
        Err(e) => println!("Error parsing point: {:?}", e),
    }

    println!("Shape is {}", Shape::Circle.to_string());
}
