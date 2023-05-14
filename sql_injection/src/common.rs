use rusqlite::{Connection, Result};

pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

pub fn create_and_insert_users(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    let table_exists: bool =
        conn.query_row("SELECT EXISTS(SELECT 1 FROM users LIMIT 1)", [], |row| {
            row.get(0)
        })?;

    if !table_exists {
        conn.execute(
            "INSERT INTO users (name, age) VALUES
                ('Alice', 24),
                ('Bob', 30),
                ('Charlie', 18)",
            [],
        )?;
    }

    Ok(())
}

pub fn print_users(users: &Vec<User>) {
    for user in users {
        println!("User {} - Name: {}, Age: {}", user.id, user.name, user.age);
    }
}
