pub fn clear_console() {
    print!("{esc}c", esc = 27 as char);
}
