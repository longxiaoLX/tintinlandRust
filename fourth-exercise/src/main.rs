use std::ops::Add;

trait PointCalculation {
    fn distance_from_origion(&self) -> f64;
}

// start: the point in two dimention
struct Point2D {
    x: i32,
    y: i32,
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PointCalculation for Point2D {
    fn distance_from_origion(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
// start: the point in two dimention

// start: the point in three dimention
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl PointCalculation for Point3D {
    fn distance_from_origion(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }
}
// start: the point in three dimention

fn dis_from_o(point: Box<dyn PointCalculation>) -> f64 {
    point.distance_from_origion()
}

fn main() {
    let p1 = Point2D { x: 1, y: 2 };
    let p2 = Point2D { x: 3, y: 4 };

    let dis1 = dis_from_o(Box::new(p1 + p2));
    println!("dis1 = {dis1}");

    let p3 = Point3D { x: 1, y: 2, z: 3 };
    let p4 = Point3D { x: 4, y: 5, z: 6 };
    let dis2 = dis_from_o(Box::new(p3 + p4));
    println!("dis2 = {dis2}");
}
