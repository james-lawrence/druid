use crate::screen::Monitor;
use super::displays;

pub(crate) fn get_monitors() -> Vec<Monitor> {
    let globals = displays::unwrap(&displays::GLOBAL)?;
    // displays::monitors(&globals)
    todo!()
}