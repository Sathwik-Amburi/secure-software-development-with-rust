mod common;

use common::create_and_insert_users;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("secure.db")?;
    conn.execute("BEGIN", [])?;
    create_and_insert_users(&conn)?;

    let users = common::get_users(&conn)?;
    common::print_users(&users);
    conn.execute("COMMIT", [])?;

    Ok(())
}
