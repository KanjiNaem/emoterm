use crossterm::{
    execute,
    
        terminal::{
    //     self,
        size,
        SetSize,
        ScrollUp,
        EnterAlternateScreen, 
    //     LeaveAlternateScreen, 
    //     SetTitle,
    //     Clear,
    //     ClearType,
    // },
    // style::{
    //     Color,
        // Print,
        // ResetColor,
        // SetBackgroundColor,
        // SetForegroundColor,
        // Stylize,
        },
};

use std::{
    io::{self, stdout, Write, Result as IOResult},
    time::Duration,
};

pub fn test() -> io::Result<()> {
    let (cols, rows) = size()?;
    
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        ScrollUp(20)
    )?;

    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}


// #[allow(dead_code)]
// struct Terminal {
//     fg_color: Color,
//     bg_color: Color,
//     title: String,
// }

// #[allow(dead_code)]
// impl Terminal {
//     fn new(title: String) -> Self {
//         Self { fg_color: Color::White,
//             bg_color: Color::Black,
//             title 
//         }

//     }
// }

// fn run(&mut self, barrier: std::sync::Arc<std::sync::Barrier>) -> Result<(), E> {
//     execute!(stdout(), EnterAlternateScreen, SetTitle(&self.title))?;
//     terminal::enable_raw_mode()?; // its rawr time

//     self.clear();

//     Ok(())

// }