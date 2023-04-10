use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE users (
            username TEXT PRIMARY KEY,
            password TEXT NOT NULL
        )",
        (),
    )?;

    let user1 = User {
        username: "alice".to_string(),
        password: "123456".to_string(),
    };
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        (&user1.username, &user1.password),
    )?;

    let user2 = User {
        username: "bob".to_string(),
        password: "password123".to_string(),
    };
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        (&user2.username, &user2.password),
    )?;

    println!("Please enter your username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username);

    println!("Please enter your password:");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password);

    let mut stmt =
        conn.prepare("SELECT username, password FROM users WHERE username = ?1 AND password = ?2")?;
    let user_iter = stmt.query_map((&username.trim(), &password.trim()), |row| {
        Ok(User {
            username: row.get(0)?,
            password: row.get(1)?,
        })
    })?;

    if user_iter.count() > 0 {
        println!("Login successful!");
    } else {
        println!("Incorrect username or password.");
    }

    //unsafe rust code for sql injection attack. (not recommended)
    // let _stmt = conn.prepare(
    //     "SELECT username, password FROM users WHERE username ={username} AND password = {password}",
    // )?;

    //' OR 1=1; DROP TABLE users; --

    let mut stmt = conn.prepare("SELECT username, password FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            password: row.get(1)?,
        })
    })?;

    println!("List of users in the database:");
    for user in user_iter {
        println!("{:?}", user.unwrap());
    }

    Ok(())
}
