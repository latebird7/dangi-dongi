use std::io;

fn main() -> io::Result<()> {
    dangi_dongi::tui::start_tui().unwrap();
    Ok(())
}
