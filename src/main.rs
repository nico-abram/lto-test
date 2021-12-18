extern "C" {
    fn symbol() -> u32;
}
fn main() -> Result<(), ()> {
    if unsafe { symbol() } == 0 {
        Ok(())
    } else {
        Err(())
    }
}
