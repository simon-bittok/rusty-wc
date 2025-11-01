use std::process::ExitCode;

use ccwc::App;

fn main() -> Result<ExitCode, ExitCode> {
    if let Err(e) = App::run() {
        eprintln!("ccwc: {}", e,);
        return Err(ExitCode::FAILURE);
    }

    Ok(ExitCode::SUCCESS)
}
