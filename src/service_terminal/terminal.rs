use crossterm::{
    cursor::{
        Hide,
        MoveTo,
        Show
    },

    event::{
        self,
        Event,
        KeyCode,
        KeyEvent
    },

    style::{
        Color,
        Print,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor,
        Stylize,
        },
    
    terminal::{
        self,
        size,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        ScrollUp,
        SetSize,
        SetTitle,
    },
    
    execute
};

use std::{
    io::{self, stdout, Write, Result as IOResult},
    time::Duration,
};



#[allow(dead_code)]
struct Terminal {
    fg_color: Color,
    bg_color: Color,
    title: String,
    state: State,
}

enum State {
    PAUSED,
    ACTIVE,
    QUIT,
}

#[allow(dead_code)]
impl Terminal {
    fn new(title: String) -> Self {
        Self { 
            fg_color: Color::White,
            bg_color: Color::Black,
            state: State::ACTIVE,
            title,
        }
    }


    // TODO: impl term runing logic
    
    // pub fn test_term_logic(&mut self, barrier: std::sync::Arc<std::sync::Barrier>) -> io::Result<()> {
    //     execute!(
    //         stdout(),
    //         EnterAlternateScreen,
    //         SetTitle(&self.title)
    //     )?;

    //     terminal::enable_raw_mode()?;
    //     self.clear_term()?;

    //     let (width, height) = terminal::size()?;

    //     barrier.wait();

    //     Ok(())
    // }
    
    fn handle_event(&mut self, curr_event: Event) -> IOResult<()> {

        match curr_event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Char('Q'),
                        ..
            }) => {
                self.state = State::QUIT;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('s') | KeyCode::Char('S'),
                        ..
            }) => {
                self.state = match self.state {
                    State::ACTIVE => State::PAUSED,
                    State::PAUSED => State::ACTIVE,
                    State::QUIT => State::QUIT,
                }
            }    
            _ => {} 
            }

        Ok(())
    }

    fn clear_term(&self) -> IOResult<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Hide,
            SetForegroundColor(self.fg_color),
            SetBackgroundColor(self.bg_color),
            MoveTo(0, 0),
        )?;

        stdout().flush()?;
        Ok(())
    }

    fn cleanup_term(&self) -> IOResult<()> {
        execute!(
            stdout(),
            ResetColor,
            Clear(ClearType::All),
            Show,
            LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn draw_frame(&self, string: &str) -> IOResult<()> {
        let print_string = |string: &str| {
            let mut out = stdout();
            execute!(
                out,
                MoveTo(0, 0),
                Print(string),
                MoveTo(0, 0)
            )?;
            
            out.flush()?;
            Ok(())
        };

        print_string(string)
    }

}
