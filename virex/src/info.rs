pub fn get_current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn get_current_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn get_current_authors() -> &'static str {
    env!("CARGO_PKG_AUTHORS")
}

pub fn get_current_description() -> &'static str {
    env!("CARGO_PKG_DESCRIPTION")
}
