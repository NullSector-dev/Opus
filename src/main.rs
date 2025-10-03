mod app;
mod banner;
mod ui;
mod input;
mod project;
mod task;
mod store;

use anyhow::{Result, Context};
use app::App;

fn main() -> Result<()>
{
    let mut app = App::load().context("Failed to load the App")?;
    app.run().context("Failed to Run the App")?;
    Ok(())
}
