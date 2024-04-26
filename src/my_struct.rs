use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Shape {
    Circle {
        radius: f32,
    },
    Rectangle {
        length: f32,
        width: f32,
    },
    Square {
        size: f32,
    },
    Cube {
        size: f32,
    },
    Cuboid {
        length: f32,
        width: f32,
        height: f32,
    },
    Parallelogram {
        base: f32,
        height: f32,
    },
    Trapezium {
        a: f32,
        b: f32,
        height: f32,
    },
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle { radius } => PI * radius.powf(2.0),
            Shape::Rectangle { length, width } => length * width,
            Shape::Square { size } => size.powf(2.0),
            Shape::Cube { size } => size.powf(3.0),
            Shape::Cuboid {
                length,
                width,
                height,
            } => length * width * height,
            Shape::Parallelogram { base, height } => base * height,
            Shape::Trapezium { a, b, height } => (a + b) * height / 2.0,
        }
    }
    fn perimeter(&self) -> Option<f32> {
        match self {
            Shape::Circle { radius } => Some(2.0 * PI * radius),
            Shape::Rectangle { length, width } => Some(2.0 * (length + width)),
            Shape::Square { size } => Some(4.0 * size),
            Shape::Cube { size } => Some(12.0 * size),
            Shape::Cuboid {
                length,
                width,
                height,
            } => Some(2.0 * (length + width + height)),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub fn my_struct() {
    let shapes = vec![
        Shape::Circle { radius: 10.0 },
        Shape::Rectangle {
            length: 5.0,
            width: 5.0,
        },
        Shape::Square { size: 10.0 },
        Shape::Cube { size: 5.0 },
        Shape::Cuboid {
            length: 5.0,
            width: 2.0,
            height: 3.0,
        },
        Shape::Parallelogram {
            base: 5.0,
            height: 10.0,
        },
        Shape::Trapezium {
            a: 1.0,
            b: 2.0,
            height: 3.0,
        },
    ];
    for i in shapes.iter() {
        match i.perimeter() {
            Some(perimeter) => println!("{:?}\nArea: {}\nPerimeter: {}\n", *i, i.area(), perimeter),
            None => println!("{:?}\nArea: {}\n", *i, i.area()),
        }
    }
}
