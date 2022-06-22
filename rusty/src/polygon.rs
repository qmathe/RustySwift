use crate::point::Point;
use std::slice;
use std::os::raw::c_uint;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ffi::{CString, CStr};
use libc;
use uuid::Uuid;

extern "C" {
    fn point_equals(left: Point, right: Point) -> bool;
}

#[derive(Clone)]
pub struct Polygon {
    id: Uuid,
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon { id: random_uuid_no_copy(), points: vec![] }
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
        let id = self.id;
        format!("Polygon containing {len} points ({status}) [{id}]")
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

// Converting Rust String to C String

// The returned string must be deallocated on Swift with free_polygon_description().
//
// Using free() to deallocate would corrupt memory (not always resulting in an immediate crash), 
// since Rust allocates memory with a different allocator than libc.
//
// CString::from_raw() takes *mut c_char as argument rather than *const c_char, so the return type 
// cannot be *const c_char as you would expect.
#[no_mangle]
pub extern "C" fn polygon_description(ptr: *mut Polygon) -> *mut c_char {
    let polygon = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    CString::new(polygon.description()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_polygon_description(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// Converting C String to Rust String + Calling Swift C function from Rust

extern "C" {
    fn random_uuid_str() -> *const c_char;
}

// If you keep the string around after the function, use into_owned() as shown here.
//
// For this function, the string is not needed after parse_str(), so using into_owned() is superfluous.
fn random_uuid() -> Uuid {
    let str = unsafe {
        let ptr = random_uuid_str();
        // to_string_lossy() returns Cow<str> (copy-on-write string slice) and into_owned() turns 
        // this slice into a new Rust string
        let str = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        libc::free(ptr as *mut c_void);
        str
    };
    let uuid = Uuid::parse_str(&str).unwrap();
    uuid
}

// If you don't keep the string around at the end, remove into_owned() to eliminate a string copy.
fn random_uuid_no_copy() -> Uuid {
    let (str, ptr) = unsafe { 
        let ptr = random_uuid_str();
        (CStr::from_ptr(ptr).to_string_lossy(), ptr)
    };
    let uuid = Uuid::parse_str(&str).unwrap();
    unsafe { 
        libc::free(ptr as *mut c_void);
    }
    uuid
}
