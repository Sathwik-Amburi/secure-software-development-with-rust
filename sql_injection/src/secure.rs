mod common;
use common::User;
use rusqlite::{Connection, Result};

fn get_users_by_name_safe(conn: &Connection, name: &str) -> Result<Vec<User>> {
    let query = "SELECT id, name, age FROM users WHERE name = ?";
    let mut stmt = conn.prepare(query)?;
    let user_iter = stmt.query_map(&[name], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    let mut users = vec![];
    for user in user_iter {
        users.push(user?);
    }

    Ok(users)
}

fn main() -> Result<()> {
    let conn = Connection::open("users.db")?;
    common::create_and_insert_users(&conn)?;

    // Simulate user input
    let name = "Charlie";
    //SQL injection
    // let name = "Charlie' OR 1=1; --";

    // Demonstrate SQL injection vulnerability

    let users = get_users_by_name_safe(&conn, name)?;

    common::print_users(&users);

    Ok(())
}
