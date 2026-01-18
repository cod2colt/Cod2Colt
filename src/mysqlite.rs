//! # Todo (Rust + SQLite)
//!
//! A Todo List application implemented in Rust
//! using `rusqlite` and SQLite.
//!
//! ## Features
//! - Add a todo item
//! - List all todo items
//! - Toggle completed / pending status
//! - Delete a todo item
//!

use chrono::Local;
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

use crate::myrust::MyPrint;

/// SQLite State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SQLiteState {
    Add,
    List,
    Toggle,
    Delete,
    Standby,
}

/// Todo data model
///
/// Corresponds to the `todo` table in SQLite
#[derive(Debug)]
struct Todo {
    /// Unique todo ID
    id: i32,
    /// Todo title
    title: String,
    /// Completion status
    done: bool,
    /// Creation timestamp (string format)
    created_at: String,
}

/// My SQLite structure
pub struct MySQLite {
    pub enable: bool,
    pub state: SQLiteState,
    pub conn: Connection,
    pub input: String,
    pub output: String,
}
impl MySQLite {
    /// Create new MySQLite
    pub fn new() -> rusqlite::Result<Self> {
        let db = Self::db_dir("todo.db");
        let conn = Connection::open(db)?;
        Self::init_db(&conn)?;

        Ok(Self {
            enable: false,
            state: SQLiteState::List,
            conn,
            input: "".to_string(),
            output: "".to_string(),
        })
    }

    /// Application entry point
    pub fn my_sqlite(&mut self) {
        if self.state == SQLiteState::Standby {
            return;
        }
        let mut print_out = MyPrint::new();
        let mut message_out = MyPrint::new();
        message_out.print_line("=============================================");

        match self.state {
            SQLiteState::List => {
                self.state = SQLiteState::Standby;
                //self.list_todos(&mut print_out)
            }
            SQLiteState::Add => {
                self.state = SQLiteState::Standby;
                self.add_todo(&mut message_out);
            }
            SQLiteState::Delete => {
                self.state = SQLiteState::Standby;
                self.delete_todo(&mut message_out);
            }
            SQLiteState::Toggle => {
                self.state = SQLiteState::Standby;
                self.toggle_todo(&mut message_out);
            }
            _ => {}
        };

        // print out
        self.list_todos(&mut print_out);
        self.input.clear();
        self.output = print_out.flush() + &message_out.flush();
    }

    /// Find the parent directory of the current executable
    /// and construct a full path to a database file located
    /// in the same directory as the application binary.
    ///
    /// # Arguments
    /// * `db_name` - SQLite database file name (e.g. `"todo.db"`)
    ///
    /// # Returns
    /// A [`PathBuf`] pointing to:
    /// `<AppExecutableDir>/<db_name>`
    ///
    /// # Behavior
    /// - Resolves the absolute path of the running executable
    /// - Uses the executable's parent directory (not CWD)
    /// - Appends `db_name` to that directory
    ///
    /// # Panics
    /// - If the executable path cannot be determined
    /// - If the executable has no parent directory
    ///
    /// # Example
    /// ```no_run
    /// let db_path = db_dir("todo.db");
    /// let conn = rusqlite::Connection::open(db_path)?;
    /// ```
    fn db_dir(db_name: &str) -> PathBuf {
        let exe = std::env::current_exe().expect("Failed to get executable path");

        exe.parent()
            .expect("Failed to get executable directory")
            .join(db_name)
    }

    /// Initialize the SQLite database schema
    ///
    /// Creates the `todo` table if it does not already exist.
    ///
    /// # Errors
    /// - Returns an error if the database write fails
    fn init_db(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            done INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
            [],
        )?;
        Ok(())
    }

    /// List all todo items
    ///
    /// - ✔ indicates completed
    /// - ✗ indicates pending
    ///
    /// # Errors
    /// - Returns an error if database query fails
    fn list_todos(&self, print_out: &mut MyPrint) {
        let mut stmt = match self
            .conn
            .prepare("SELECT id, title, done, created_at FROM todo ORDER BY id")
        {
            Ok(s) => s,
            Err(e) => {
                // other errors
                print_out.print_line(format!("{}", e));
                return;
            }
        };

        let todos = match stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                done: row.get::<_, i32>(2)? == 1,
                created_at: row.get(3)?,
            })
        }) {
            Ok(s) => s,
            Err(e) => {
                // other errors
                print_out.print_line(format!("{}", e));
                return;
            }
        };

        print_out.print_line("\n ID |Status|    Title   | Created At");
        print_out.print_line("---------------------------------------------");

        for todo in todos {
            let t = match todo {
                Ok(n) => n,
                Err(e) => {
                    // other errors
                    print_out.print_line(format!("{}", e));
                    return;
                }
            };
            let status = if t.done { "✅" } else { "❌" };
            print_out.print_line(format!(
                "{:>4}|{:^5}| {:<10} | {:^12}",
                t.id, status, t.title, t.created_at
            ));
        }
    }
    /// Add a new todo item
    ///
    /// - Default status is `pending`
    /// - Automatically records creation time
    ///
    /// # Errors
    /// - Returns an error if database insertion fails
    fn add_todo(&self, print_out: &mut MyPrint) {
        let title = self.input.clone();
        if title.trim().is_empty() {
            print_out.print_line("❗ Todo TITLE is EMPTY!");
            return;
        }
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match self.conn.execute(
            "INSERT INTO todo (title, done, created_at) VALUES (?1, 0, ?2)",
            params![title, now],
        ) {
            Ok(_) => {}
            Err(e) => {
                // other errors
                print_out.print_line(format!("{}", e));
                return;
            }
        };

        print_out.print_line("📝 Todo added successfully");
    }

    /// Toggle the completion status of a todo item
    ///
    /// - Completed → Pending
    /// - Pending → Completed
    ///
    /// # Errors
    /// - Returns an error if query or update fails
    fn toggle_todo(&self, print_out: &mut MyPrint) {
        let id = match self.input.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                print_out.print_line("❗ Todo ID is WRONG!");
                return;
            }
        };

        use rusqlite::Error;

        let current_result: Result<i32, Error> =
            self.conn
                .query_row("SELECT done FROM todo WHERE id = ?1", params![id], |row| {
                    row.get::<_, i32>(0)
                });

        let current: i32 = match current_result {
            Ok(v) => v,
            Err(Error::QueryReturnedNoRows) => {
                print_out.print_line("❗ Todo ID not found!");
                return;
            }
            Err(e) => {
                // other errors
                print_out.print_line(format!("{}", e));
                return;
            }
        };

        let new_status = if current == 1 { 0 } else { 1 };

        match self.conn.execute(
            "UPDATE todo SET done = ?1 WHERE id = ?2",
            params![new_status, id],
        ) {
            Ok(_) => {}
            Err(e) => {
                // other errors
                print_out.print_line(format!("{}", e));
                return;
            }
        };

        print_out.print_line(format!("🔄 {} Todo status updated", id));
    }

    /// Delete a todo item by ID
    ///
    /// # Errors
    /// - Returns an error if deletion fails
    fn delete_todo(&self, print_out: &mut MyPrint) {
        let id = match self.input.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                print_out.print_line("❗ Todo ID is WRONG!");
                return;
            }
        };
        let rows_deleted = match self
            .conn
            .execute("DELETE FROM todo WHERE id = ?1", params![id])
        {
            Ok(n) => n,
            Err(_) => {
                print_out.print_line("❗ Todo ID not found!");
                return;
            }
        };

        if rows_deleted == 0 {
            print_out.print_line("❗ Todo ID not found!");
        } else {
            print_out.print_line(&format!("🗑 Deleted todo {}", id));
        }
    }
}
