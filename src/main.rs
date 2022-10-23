use crossterm::event::{self, Event, KeyCode};
use std::{error::Error, io};

mod ui;
use ui::{UiTrait, UI};

mod app;
use app::App;

mod events;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut app = App::default();
    app.create_request(app::Request::default());

    let mut app_ui: UI = UiTrait::init(&app);

    loop {
        app_ui.render();

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    app_ui.close();

    Ok(())
}
