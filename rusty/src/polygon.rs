use crate::point::Point;
use std::slice;
use std::os::raw::c_uint;
use std::os::raw::c_char;
use std::ffi::CString;

extern "C" {
    fn point_equals(left: Point, right: Point) -> bool;
}

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon { points: vec![] }
    }

    pub fn length(&self) -> f64 {
        self.points.windows(2).fold(0.0, |length, pair| {
            length + pair[0].distance(pair[1])
        })
    }
    
    pub fn description(&self) -> String {
        let len = self.points.len();
        let status = if len > 1 && unsafe { point_equals(self.points[0], self.points[1]) } { 
            "closed"  
        } else { 
            "opened"
        };
        format!("Polygon containing {len} points ({status})")
    }
}

#[no_mangle]
pub extern "C" fn polygon_new() -> *mut Polygon {
    Box::into_raw(Box::new(Polygon::new()))
}

#[no_mangle]
pub extern "C" fn polygon_free(ptr: *mut Polygon) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn polygon_length(ptr: *mut Polygon) -> f64 {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    polygon.length()
}

#[no_mangle]
pub extern "C" fn polygon_points(ptr: *mut Polygon, len: *mut c_uint) -> *mut Point {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let slice = polygon.points.clone().into_boxed_slice();
    unsafe {
        *len = slice.len() as c_uint;
    }
    Box::into_raw(slice) as *mut Point
}

#[no_mangle]
pub extern "C" fn free_points(points: *mut Point) {
    drop(unsafe { Box::from_raw(points) });
}

#[no_mangle]
pub extern "C" fn polygon_set_points(ptr: *mut Polygon, points: *const Point, len: c_uint) {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let slice = unsafe {
        slice::from_raw_parts(points, len as usize)
    };
    polygon.points.clear();
    polygon.points.extend_from_slice(slice)
}

#[no_mangle]
pub extern "C" fn polygon_push(ptr: *mut Polygon, point: Point) {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    polygon.points.push(point)
}

#[no_mangle]
pub extern "C" fn polygon_remove(ptr: *mut Polygon, i: i64) {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    polygon.points.remove(usize::try_from(i).unwrap());
}


#[no_mangle]
pub extern "C" fn polygon_description(ptr: *mut Polygon) -> *const c_char {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    CString::new(polygon.description()).unwrap().into_raw()
}
