mod ui;
mod app;
use std::{error::Error, io};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use crate::{app::App, ui::ui};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // 设置窗口大小
    execute!(terminal.backend_mut(), crossterm::terminal::SetSize(140, 85))?;
    // Create app and run it
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) ->Result<(), Box<dyn Error>>{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip release events
            }

            match app.current_screen {
                crate::app::CurrentScreen::Main => match key.code {
                    KeyCode::Enter => {
                        if let Ok(amount) = app.amount_input.parse::<f64>() {
                            app.amount = amount; // 更新用户输入的金额
                            app.convert_currency()?; // 调用转换函数
                        } else {
                            app.converted_amount = "请输入有效的数字".to_string();
                        }
                    }
                    KeyCode::Char(c) => {
                        app.amount_input.push(c); // 添加字符到输入中
                    }
                    KeyCode::Up => {
                        app.selected_index += 1;
                        if app.selected_index > 8 {
                            app.selected_index = 1; // 超过8则变为1
                        }
                    }
                    KeyCode::Down => {
                        if app.selected_index == 1 {
                            app.selected_index = 8; // 小于1则变为8
                        } else {
                            app.selected_index -= 1;
                        }
                    }
                    KeyCode::Backspace => {
                        app.amount_input.pop(); // 删除最后一个字符
                    }
                    KeyCode::Esc => {
                        app.current_screen = crate::app::CurrentScreen::Exiting; // 退出
                    }
                    //更换币种

                    _ => {}
                },
                _ => {}
            }
        }
    }
}
