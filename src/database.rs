mod translation;
pub use translation::Translation;
mod word_status;
pub use word_status::WordStatus;

use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open("./database.db3").expect("Failed to connect to database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS dictionary(
            id          INTEGER PRIMARY KEY,
            word        TEXT NOT NULL,
            translated  TEXT NOT NULL,
            status	    INTEGER NOT NULL
            )",
            (),
        )
        .unwrap();

        let database = Self { conn };

        if database.count() == 0 {
            database.insert(&Translation {
                from: String::new(),
                to: String::new(),
                status: WordStatus::NotAWord,
            });
        }

        database
    }

    pub fn insert(&self, translation: &Translation) {
        self.conn
            .execute(
                "INSERT INTO dictionary (word, translated, status) VALUES (?1, ?2, ?3)",
                (
                    &translation.from,
                    &translation.to,
                    translation.status as usize,
                ),
            )
            .expect("Failed to insert dictionary");
    }

    pub fn update_status_by_from(&self, from: &str, status: WordStatus) {
        self.conn
            .execute(
                "UPDATE dictionary SET status = ?2 WHERE word = ?1",
                (from, status as usize),
            )
            .expect("Failed to update dictionary");
    }

    #[allow(unused)] // ? Remove if not necessary
    pub fn get_all(&self) -> Vec<Translation> {
        let stmt = self.conn.prepare("SELECT * FROM dictionary");

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        stmt.query_map([], |row| {
            Ok(Translation {
                from: row.get(1)?,
                to: row.get(2)?,
                status: match row.get(3)? {
                    0 => WordStatus::NotAWord,
                    1 => WordStatus::Learning,
                    2 => WordStatus::Mastered,
                    _ => panic!("Dictionary has wrong entry in status"),
                },
            })
        })
        .unwrap()
        .map(|dictionary| dictionary.expect("Failed to read translation from database"))
        .collect()
    }

    pub fn get_by_from(&self, from: &str) -> Option<Translation> {
        let stmt = self
            .conn
            .prepare("SELECT * FROM dictionary WHERE word = ?1 LIMIT 1");

        if stmt.is_err() {
            return None;
        }

        let mut stmt = stmt.unwrap();

        let entry = stmt.query_row([from], |row| {
            Ok(Translation {
                from: row.get(1)?,
                to: row.get(2)?,
                status: match row.get(3)? {
                    0 => WordStatus::NotAWord,
                    1 => WordStatus::Learning,
                    2 => WordStatus::Mastered,
                    _ => panic!("Dictionary has wrong entry in status"),
                },
            })
        });

        entry.ok()
    }

    pub fn get_by_search(&self, search: &str) -> Vec<Translation> {
        let stmt = self.conn.prepare(
            "SELECT * FROM dictionary 
                WHERE NOT status = 0 
                AND (word LIKE '%' || ?1 || '%' OR translated LIKE '%' || ?1 || '%')",
        );

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        stmt.query_map([search], |row| {
            Ok(Translation {
                from: row.get(1)?,
                to: row.get(2)?,
                status: match row.get(3)? {
                    0 => WordStatus::NotAWord,
                    1 => WordStatus::Learning,
                    2 => WordStatus::Mastered,
                    _ => panic!("Dictionary has wrong entry in status"),
                },
            })
        })
        .unwrap()
        .map(|dictionary| dictionary.expect("Failed to read translation from database"))
        .collect()
    }

    pub fn count(&self) -> usize {
        let stmt = self.conn.prepare(
            "SELECT COUNT(*)
                FROM dictionary",
        );

        if stmt.is_err() {
            return 0;
        }

        let mut stmt = stmt.unwrap();

        stmt.query_row([], |row| Ok(row.get(0)?)).unwrap()
    }

    pub fn count_by_status(&self, status: WordStatus) -> usize {
        let stmt = self.conn.prepare(
            "SELECT COUNT(status)
                FROM dictionary 
                WHERE status = ?1",
        );

        if stmt.is_err() {
            return 0;
        }

        let mut stmt = stmt.unwrap();

        stmt.query_row([status as usize], |row| Ok(row.get(0)?))
            .unwrap()
    }
}
