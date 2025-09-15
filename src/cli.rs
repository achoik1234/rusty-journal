
// structopt is most commonly used to parse command line arguments
// by defining a struct that represents them.

// Import PathBuf to conveniently handle filesystem paths.
use std::path::PathBuf;

// Import the StructOpt trait to enable parsing command-line arguments into structs.
use structopt::StructOpt;

// What is #[derive(...)]?
// It's an attribute macro that tells Rust to automatically generate implementations
// of specified traits for your struct or enum.
// This saves you from writing repetitive boilerplate code.

// What is a trait?
// A trait in Rust is like an interface in other languages.
// It defines required methods that a type must implement.

// #[derive(Debug, StructOpt)] automatically implements:
// - Debug: allows printing the struct or enum with {:?} for debugging.
// - StructOpt: allows parsing CLI arguments into the struct or enum.

#[derive(Debug, StructOpt)]
pub enum Action { // 3 subcommands: Add, Done, List
    // Add a task to the journal file
    Add { 
        text: String,  // The text of the task to add
    },

    // Remove an entry from the journal by its position (index)
    Done {
        position: usize,  // Position of task to remove
    },

    // List all tasks in the journal file
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line todo app written in Rust"
)]
pub struct CommandLineArgs {
    // Specify that the next argument will be a subcommand from the Action enum
    #[structopt(subcommand)]
    pub action: Action,

    /// Optional path to the journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}


// What happens in this file?
//
// This file defines the command-line interface (CLI) for a Rust todo app using the structopt crate.
//
// It defines an Action enum representing three subcommands the user can call:
//
//     Add: add a task with a text string.
//
//     Done: mark a task done by its position number.
//
//     List: list all tasks.
//
// It also defines a CommandLineArgs struct representing the full CLI input, which includes:
//
//     The subcommand chosen (Action).
//
//     An optional path to a journal file.
//
// The #[derive(StructOpt, Debug)] macros automatically generate all the parsing logic and debugging helpers.
//
// This setup allows the program to easily parse and handle user input from the command line in a clean, declarative way.
