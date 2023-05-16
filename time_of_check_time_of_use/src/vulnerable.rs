mod common;
use common::create_and_insert_users;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("vulnerable.db")?;
    create_and_insert_users(&conn)?;

    let users = common::get_users(&conn)?;
    common::print_users(&users);

    Ok(())
}
