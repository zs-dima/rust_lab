pub mod l4and5struct {
    use std::num::ParseIntError;

    #[derive(Debug)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn zero() -> Point {
            Point { x: 0, y: 0 }
        }

        pub fn symetric(xy: i32) -> Point {
            Point { x: xy, y: xy }
        }

        pub fn area(&self, other: &Point, shape: &Shape) -> i64 {
            match shape {
                Shape::Circle => self.circle_area(other),
                Shape::Rectangle => self.rectangle_area(other),
            }
        }

        pub fn rectangle_area(&self, other: &Point) -> i64 {
            let x_diff = i32::abs(self.x - other.x);
            let y_diff = i32::abs(self.y - other.y);

            (x_diff * y_diff) as i64
        }

        pub fn circle_area(&self, other: &Point) -> i64 {
            let x_diff = i32::abs(self.x - other.x);
            let y_diff = i32::abs(self.y - other.y);

            let radius = ((i32::pow(x_diff, 2) + i32::pow(y_diff, 2)) as f64).sqrt() / 2.0;

            (std::f64::consts::PI * radius * radius) as i64
        }

        pub fn distance(&self, other: &Point) -> f64 {
            let x_diff = self.x - other.x;
            let y_diff = self.y - other.y;

            ((i32::pow(x_diff, 2) + i32::pow(y_diff, 2)) as f64).sqrt()
        }

        pub fn divide_x(&self, divide_by: f64) -> Option<f64> {
            if divide_by == 0.0 {
                None
            } else {
                Some(self.x as f64 / divide_by)
            }
        }

        pub fn parse(text: &str) -> Result<Point, PointParseError> {
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
    pub enum PointParseError {
        MissingValue,
        TooManyValues,
        ParseError(ParseIntError),
    }

    impl From<ParseIntError> for PointParseError {
        fn from(err: ParseIntError) -> Self {
            PointParseError::ParseError(err)
        }
    }

    pub enum Shape {
        Circle,
        Rectangle,
    }

    impl Shape {
        pub fn to_string(&self) -> &str {
            match self {
                Shape::Circle => "Circle!",
                Shape::Rectangle => "Rectangle!",
            }
        }
    }
}
