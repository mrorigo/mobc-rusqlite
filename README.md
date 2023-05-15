# mobc-rusqlite

[rusqlite](https://crates.io/crates/rusqlite) adapter for [mobc](https://crates.io/crates/mobc)

## Example

```rust
use mobc::Manager;
use mobc_rusqlite::RusqliteConnectionManager;

#[tokio::main]
async fn main() {
    let m = RusqliteConnectionManager::new(None);

    match m.connect().await {
        Err(er) => panic!("{:?}", er),
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE test1 ( id   INTEGER PRIMARY KEY, value TEXT NOT NULL)",
                [],
            )
            .unwrap();

            conn.execute(
                "INSERT INTO test1 (id, value) VALUES (?1, ?2)",
                (&1, &"string"),
            )
            .unwrap();

            conn.query_row("SELECT id, value FROM test1", [], |row: &rusqlite::Row| {
                let v = row.get::<usize, usize>(0).unwrap();
                assert_eq!(v, 1);
                Ok(())
            })
            .unwrap();
        }
    }
}
```