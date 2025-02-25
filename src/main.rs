use kmon::args;
use kmon::event::Events;
use kmon::kernel::Kernel;
use std::error::Error;
use std::io::stdout;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

/**
 * Entry point.
 *
 * @return Result
 */
fn main() -> Result<(), Box<dyn Error>> {
	let args = args::get_args().get_matches();
	let kernel = Kernel::new(&args);
	let events = Events::new(
		args.get_one::<String>("rate")
			.unwrap()
			.parse::<u64>()
			.unwrap_or(250),
		&kernel.logs,
	);
	if !cfg!(test) {
		let stdout = stdout().into_raw_mode()?.into_alternate_screen()?;
		let stdout = MouseTerminal::from(stdout);
		let backend = TermionBackend::new(stdout);
		kmon::start_tui(Terminal::new(backend)?, kernel, &events)
	} else {
		Ok(())
	}
}
