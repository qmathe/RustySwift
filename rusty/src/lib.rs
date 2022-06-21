pub mod point;
pub mod polygon;

#[no_mangle]
pub extern "C" fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
