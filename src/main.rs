use std::{
    io::{Stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor, execute, queue,
    style::{self, Print},
    terminal::{self, ClearType},
    Result,
};

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();

    let (cols, rows) = terminal::size()?;

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(
        stdout,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(1, 1)
    )?;

    draw_window(&mut stdout, 0, 0, 30, 10)?;

    stdout.flush()?;

    sleep(Duration::from_secs(3));

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_window(
    stdout: &mut Stdout,
    x_offset: u16,
    y_offset: u16,
    width: u16,
    height: u16,
) -> Result<()> {
    let string = "┌".to_string() + &"─".to_string().repeat(width as usize - 2) + "┐";
    queue!(
        stdout,
        cursor::MoveTo(x_offset, y_offset),
        style::Print(string)
    )?;
    for draw_lines in 1..height {
        queue!(
            stdout,
            cursor::MoveTo(x_offset, y_offset + draw_lines),
            Print("│"),
            cursor::MoveTo(x_offset + width - 1, y_offset + draw_lines),
            Print("│")
        )?;
    }
    let string = "└".to_string() + &"─".to_string().repeat(width as usize - 2) + "┘";
    queue!(
        stdout,
        cursor::MoveTo(x_offset, y_offset + height),
        style::Print(string)
    )?;
    Ok(())
}
