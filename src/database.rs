mod translation;
pub use translation::Translation;
mod word_status;
pub use word_status::WordStatus;
mod dict_item;
pub use dict_item::DictItem;

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
            "CREATE TABLE IF NOT EXISTS user_dictionary(
            id          INTEGER PRIMARY KEY,
            word        TEXT NOT NULL,
            status	    INTEGER NOT NULL
            )",
            (),
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS dict_dictionary(
            id          INTEGER PRIMARY KEY,
            left_word	TEXT NOT NULL,
            right_word	TEXT NOT NULL,
            classes		TEXT NOT NULL,
            genders		TEXT NOT NULL
            )",
            (),
        )
        .unwrap();

        let database = Self { conn };

        if database.count() == 0 {
            database.insert(&Translation {
                word: String::new(),
                status: WordStatus::NotAWord,
            });
        }

        database
    }

    pub fn insert(&self, translation: &Translation) {
        self.conn
            .execute(
                "INSERT INTO user_dictionary (word, status) VALUES (?1, ?2)",
                (&translation.word, translation.status as usize),
            )
            .expect("Failed to insert user_dictionary");
    }

    pub fn update_status_by_word(&self, word: &str, status: WordStatus) {
        self.conn
            .execute(
                "UPDATE user_dictionary SET status = ?2 WHERE word = ?1",
                (word, status as usize),
            )
            .expect("Failed to update user_dictionary");
    }

    #[allow(unused)] // ? Remove if not necessary
    pub fn get_all(&self) -> Vec<Translation> {
        let stmt = self.conn.prepare("SELECT * FROM user_dictionary");

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        stmt.query_map([], |row| {
            Ok(Translation {
                word: row.get(1)?,
                status: match row.get(2)? {
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

    pub fn get_by_word(&self, from: &str) -> Option<Translation> {
        let stmt = self
            .conn
            .prepare("SELECT * FROM user_dictionary WHERE word = ?1 LIMIT 1");

        if stmt.is_err() {
            return None;
        }

        let mut stmt = stmt.unwrap();

        let entry = stmt.query_row([from], |row| {
            Ok(Translation {
                word: row.get(1)?,
                status: match row.get(2)? {
                    0 => WordStatus::NotAWord,
                    1 => WordStatus::Learning,
                    2 => WordStatus::Mastered,
                    _ => panic!("Dictionary has wrong entry in status"),
                },
            })
        });

        entry.ok()
    }

    pub fn search_user_entries(&self, search: &str) -> Vec<Translation> {
        let stmt = self.conn.prepare(
            "SELECT * FROM user_dictionary 
                WHERE word LIKE '%' || ?1 || '%'",
        );

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        stmt.query_map([search], |row| {
            Ok(Translation {
                word: row.get(1)?,
                status: match row.get(2)? {
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
                FROM user_dictionary",
        );

        if stmt.is_err() {
            return 0;
        }

        let mut stmt = stmt.unwrap();

        stmt.query_row([], |row| row.get(0)).unwrap()
    }

    pub fn count_by_status(&self, status: WordStatus) -> usize {
        let stmt = self.conn.prepare(
            "SELECT COUNT(status)
                FROM user_dictionary 
                WHERE status = ?1",
        );

        if stmt.is_err() {
            return 0;
        }

        let mut stmt = stmt.unwrap();

        stmt.query_row([status as usize], |row| row.get(0)).unwrap()
    }

    pub fn search_dict_entries(&self, search: &str) -> Vec<DictItem> {
        let stmt = self.conn.prepare(
            "SELECT * FROM dict_database
			            WHERE LOWER(left_word) LIKE '%' || ?1 || '%'
						ORDER BY LENGTH(left_word)
                        LIMIT 100",
        );

        if stmt.is_err() {
            return vec![];
        }

        let mut stmt = stmt.unwrap();

        stmt.query_map([search], |row| {
            Ok(DictItem {
                left_word: row.get(1)?,
                right_word: row.get(2)?,
                classes: row
                    .get::<usize, String>(3)?
                    .split(",")
                    .map(|x| x.to_string())
                    .collect(),
                genders: row
                    .get::<usize, String>(4)?
                    .split(",")
                    .map(|x| x.to_string())
                    .collect(),
            })
        })
        .unwrap()
        .map(|dictionary| dictionary.expect("Failed to read translation from database"))
        .collect()
    }
}
