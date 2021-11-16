use crate::screen::Monitor;
use super::displays;

pub(crate) fn get_monitors() -> Vec<Monitor> {
    let globals = match displays::unwrap(&displays::GLOBAL) {
        Ok(g) => g,
        Err(cause) => {
            tracing::error!("unable to retrieve monitors {:?}", cause);
            return Vec::new();
        },
    };

    return Vec::new();
}