# 🦀 Rust Task Manager

##  This project is guided by the **Microsoft Reactor program** and aims to create a tool that allows users to **add, list, and complete tasks**. All task information is stored in a **JSON file**, ensuring data persists between program executions.  

---

## Project Description
The project is a **lightweight command-line application** that manages a to-do list stored in a local file.  

It offers three core functionalities:
- **Add a Task** → Create a new task with a description. Each task gets a timestamp when created.  
- **List Tasks** → Display all stored tasks with their creation time in a human-friendly format.  
- **Complete a Task** → Remove a task from the list once it’s finished.  

Each task is represented by a `Task` struct containing:
- `text` → A string describing the task  
- `created_at` → A timestamp of when the task was created (stored in UTC and displayed in the user’s local timezone).  

---

##  System Architecture
The system follows a **modular architecture** with clear responsibilities:  

### `main.rs`
- Program entry point  
- Receives parsed CLI arguments from the `cli` module  
- Chooses the correct action (`Add`, `List`, `Done`)  
- Delegates execution to the `tasks` module  

### `cli.rs`
Defines the **command-line interface** using the `structopt` library.  

Examples:
```bash
cargo run -- -j journal.json add "Buy milk"
cargo run -- -j journal.json list
cargo run -- -j journal.json done 1

### tasks.rs

Contains the core business logic:

Defines the Task struct

Implements add, list, and done functions

Handles serialization (save tasks to JSON) and deserialization (load tasks from JSON)

### journal.json

Stores the user’s task list in JSON format, ensuring data persistence.

---
## Libraries and Core Concepts

The project combines three key Rust libraries:

structopt → Simplifies parsing command-line arguments.

serde + serde_json → For serialization & deserialization of tasks into JSON.

chrono → For handling dates & timestamps.
---

## Core Functions

Add Task → Reads tasks from JSON, appends new task, saves updated list.

List Tasks → Reads all tasks and displays them with local timezone formatting.

Complete Task → Removes a task by index and updates the JSON file.

### Custom Display

The project implements the fmt::Display trait for clean output formatting.

This produces nice, readable output : Buy milk                                           [2025-09-14 21:00]
Instead of the raw struct: Task { text: "Buy milk", created_at: 2025-09-14T20:00:00Z }

--- 
## Traits in Rust

A trait in Rust is similar to an interface in Java or an abstract class in C++.
It defines a contract of methods a type must implement, without specifying how.

In this project:

fmt::Display is used to control how Task is formatted into a human-readable string.

This enables clean, customizable console output for each task.
