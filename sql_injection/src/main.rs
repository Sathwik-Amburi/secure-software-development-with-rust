use rusqlite::{Connection, Result};

struct User {
    id: i32,
    name: String,
    age: i32,
}

fn create_users_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  age INTEGER NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

fn insert_dummy_users(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES
                  ('Alice', 24),
                  ('Bob', 30),
                  ('Charlie', 18)",
        [],
    )?;
    Ok(())
}

fn get_users_by_name(conn: &Connection, name: &str) -> Result<Vec<User>> {
    let query = format!("SELECT id, name, age FROM users WHERE name = '{}'", name);
    let mut stmt = conn.prepare(&query)?;
    let user_iter = stmt.query_map([], |row| {
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

// fn get_users_by_name_safe(conn: &Connection, name: &str) -> Result<Vec<User>> {
//     let query = "SELECT id, name, age FROM users WHERE name = ?";
//     let mut stmt = conn.prepare(query)?;
//     let user_iter = stmt.query_map(&[name], |row| {
//         Ok(User {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             age: row.get(2)?,
//         })
//     })?;
//     let mut users = vec![];
//     for user in user_iter {
//         users.push(user?);
//     }

//     Ok(users)
// }

fn print_users(users: &Vec<User>) {
    for user in users {
        println!("User {} - Name: {}, Age: {}", user.id, user.name, user.age);
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("users.db")?;
    create_users_table(&conn)?;
    insert_dummy_users(&conn)?;

    // Simulate user input
    // let name = "Charlie";
    //SQL injection
    let name = "Charlie' OR 1=1; --";

    // Demonstrate SQL injection vulnerability
    let users = get_users_by_name(&conn, name)?;

    // let users = get_users_by_name_safe(&conn, name)?;

    print_users(&users);

    Ok(())
}
