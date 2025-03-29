use std::num::ParseIntError;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    fn symetric(xy: i32) -> Point {
        Point { x: xy, y: xy }
    }

    fn area(&self, other: &Point, shape: &Shape) -> i64 {
        match shape {
            Shape::Circle => self.circle_area(other),
            Shape::Rectangle => self.rectangle_area(other),
        }
    }

    fn rectangle_area(&self, other: &Point) -> i64 {
        let x_diff = i32::abs(self.x - other.x);
        let y_diff = i32::abs(self.y - other.y);

        (x_diff * y_diff) as i64
    }

    fn circle_area(&self, other: &Point) -> i64 {
        let x_diff = i32::abs(self.x - other.x);
        let y_diff = i32::abs(self.y - other.y);

        let radius = ((i32::pow(x_diff, 2) + i32::pow(y_diff, 2)) as f64).sqrt() / 2.0;

        (std::f64::consts::PI * radius * radius) as i64
    }

    fn distance(&self, other: &Point) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;

        ((i32::pow(x_diff, 2) + i32::pow(y_diff, 2)) as f64).sqrt()
    }

    fn divide_x(&self, divide_by: f64) -> Option<f64> {
        if divide_by == 0.0 {
            None
        } else {
            Some(self.x as f64 / divide_by)
        }
    }

    fn parse(text: &str) -> Result<Point, PointParseError> {
        // "123, 321"
        // do not use unwrap

        let mut iter = text.split(",");
        // do not use unwrap
        // use match instead

        // Get first number
        let x_str = match iter.next() {
            Some(val) => val.trim(),
            None => return Err(PointParseError::MissingValue),
        };

        // Get second number
        let y_str = match iter.next() {
            Some(val) => val.trim(),
            None => return Err(PointParseError::MissingValue),
        };

        // Check no extra parts
        if iter.next().is_some() {
            return Err(PointParseError::TooManyValues);
        }

        // Parse numbers
        let x = match x_str.parse::<i32>() {
            Ok(num) => num,
            Err(e) => return Err(PointParseError::from(e)),
        };

        let y = match y_str.parse::<i32>() {
            Ok(num) => num,
            Err(e) => return Err(PointParseError::from(e)),
        };

        Ok(Point { x: x, y: y })
    }
}

#[derive(Debug)]
enum PointParseError {
    MissingValue,
    TooManyValues,
    ParseError(ParseIntError),
}

impl From<ParseIntError> for PointParseError {
    fn from(err: ParseIntError) -> Self {
        PointParseError::ParseError(err)
    }
}

enum Shape {
    Circle,
    Rectangle,
}

impl Shape {
    fn to_string(&self) -> &str {
        match self {
            Shape::Circle => "Circle!",
            Shape::Rectangle => "Rectangle!",
        }
    }
}

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
