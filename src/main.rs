use std::{
    io::{Stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, ContentStyle, Print, Attributes, Attribute},
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

    draw_window(&mut stdout, 0, 0, cols / 2, rows)?;
    draw_active_window(&mut stdout, cols / 2, 0, cols / 2, rows / 2)?;
    draw_window(&mut stdout, cols / 2, rows / 2, cols / 2, rows / 2)?;

    stdout.flush()?;

    sleep(Duration::from_secs(3));

    terminal::disable_raw_mode()?;

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
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
    for draw_lines in 1..height - 1 {
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
        cursor::MoveTo(x_offset, y_offset + height - 1),
        style::Print(string)
    )?;
    Ok(())
}

fn draw_active_window(
    stdout: &mut Stdout,
    x_offset: u16,
    y_offset: u16,
    width: u16,
    height: u16,
) -> Result<()> {
    let string = "╔".to_string() + &"═".to_string().repeat(width as usize - 2) + "╗";
    queue!(
        stdout,
        cursor::MoveTo(x_offset, y_offset),
        style::Print(string)
    )?;
    for draw_lines in 1..height - 1 {
        queue!(
            stdout,
            cursor::MoveTo(x_offset, y_offset + draw_lines),
            Print("║"),
            cursor::MoveTo(x_offset + width - 1, y_offset + draw_lines),
            Print("║")
        )?;
    }
    let string = "╚".to_string() + &"═".to_string().repeat(width as usize - 2) + "╝";
    queue!(
        stdout,
        cursor::MoveTo(x_offset, y_offset + height - 1),
        style::Print(string)
    )?;
    Ok(())
}
