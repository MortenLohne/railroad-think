/// See: https://burn.dev/burn-book/basic-workflow/data.html
use rusqlite::{Connection, OpenFlags};

pub fn get_connection() -> Connection {
    let conn = Connection::open_with_flags(
        "./data.sqlite",
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    );

    match conn {
        Ok(c) => c,
        Err(_) => {
            let conn = Connection::open("./data.sqlite").unwrap();

            conn.execute(
                "CREATE TABLE matches (
                    id     INTEGER PRIMARY KEY,
                    board  TEXT NOT NULL,
                    move   TEXT NOT NULL,
                    score  INTEGER NOT NULL
                )",
                (), // empty list of parameters.
            )
            .unwrap();

            conn
        }
    }
}
