use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
    // Initialize the SQLite database
    let db_name = "example.db";
    let conn = Arc::new(Mutex::new(Connection::open(&db_name)?));

    // Create the users table if it doesn't exist
    {
        let conn = conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, balance INTEGER NOT NULL)",
            [],
        )?;
    }

    // Add a user with a 1000 balance
    {
        let conn = conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO users (id, balance) VALUES (?1, ?2)",
            params![1, 1000],
        )?;
    }

    // Create a function to simulate a TOCTOU vulnerability
    let transfer_money = |conn: Arc<Mutex<Connection>>, user_id: i32, amount: i32| {
        // Check if the user has enough balance to make the transaction
        let balance: i32 = {
            let conn = conn.lock().unwrap();
            conn.query_row(
                "SELECT balance FROM users WHERE id = ?1",
                params![user_id],
                |row| row.get(0),
            )
            .unwrap()
        };

        // Introduce a delay to make the TOCTOU vulnerability more apparent
        thread::sleep(std::time::Duration::from_secs(2));

        if balance >= amount {
            // Update the user's balance
            let conn = conn.lock().unwrap();
            conn.execute(
                "UPDATE users SET balance = balance - ?1 WHERE id = ?2",
                params![amount, user_id],
            )
            .unwrap();
            println!(
                "Transaction successful: User {} transferred {}.",
                user_id, amount
            );
        } else {
            println!(
                "Transaction failed: User {} has insufficient balance.",
                user_id
            );
        }
    };

    // Spawn two threads simulating concurrent requests to transfer_money function
    let conn1 = Arc::clone(&conn);
    let conn2 = Arc::clone(&conn);

    let thread1 = thread::spawn(move || {
        transfer_money(conn1, 1, 700);
    });

    let thread2 = thread::spawn(move || {
        transfer_money(conn2, 1, 700);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final balance of user 1
    let final_balance: i32 = {
        let conn = conn.lock().unwrap();
        conn.query_row(
            "SELECT balance FROM users WHERE id = ?1",
            params![1],
            |row| row.get(0),
        )
        .unwrap()
    };
    println!("Final balance for User 1: {}", final_balance);

    Ok(())
}
