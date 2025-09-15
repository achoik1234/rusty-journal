mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // Get the command line arguments
    let CommandLineArgs { action, journal_file } = CommandLineArgs::from_args();

    // Unpack the journal file (PathBuf)
    let journal_file = journal_file.expect("Failed to find journal file");

    // Perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_task(journal_file),  // ✅ match your function name
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action");
}
