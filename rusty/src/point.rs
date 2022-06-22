// 
// PART 1: RUST CODE
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

// 
// PART 2: C COMPATIBILITY CODE
//

#[no_mangle]
pub extern "C" fn distance_to(point: Point, other: Point) -> f64 {
    point.distance(other)
}
