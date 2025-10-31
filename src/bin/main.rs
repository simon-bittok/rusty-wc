use std::process::ExitCode;

use ccwc::App;

fn main() -> Result<ExitCode, ExitCode> {
    if let Err(_e) = App::run() {
        return Err(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}
