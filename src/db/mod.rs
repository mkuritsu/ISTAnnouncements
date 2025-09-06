mod structs;

use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
pub use structs::*;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Database, sqlx::Error> {
        let opts = SqliteConnectOptions::from_str(url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(opts).await?;
        let db = Database { pool };
        db.create_table_courses().await?;
        Ok(db)
    }

    async fn create_table_courses(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS Courses (
            id INT8 PRIMARY KEY,
            name TEXT NOT NULL,
            rss_url TEXT NOT NULL,
            color INTEGER NOT NULL,
            role_id TEXT NOT NULL,
            last_announcement INT8
        )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_course(
        &self,
        id: i64,
        name: &str,
        rss_url: &str,
        color: i32,
        role_id: String,
        last_announcement: Option<i64>,
    ) -> Result<Course, sqlx::Error> {
        sqlx::query_as::<_, Course>(
            "INSERT INTO Courses (id, name, rss_url, color, role_id, last_announcement) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(rss_url)
        .bind(color)
        .bind(role_id)
        .bind(last_announcement)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        sqlx::query_as::<_, Course>("SELECT * FROM Courses")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_course_last_message(
        &self,
        course: &Course,
        new_value: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE Courses SET last_announcement = $1 WHERE Courses.id = $2")
            .bind(new_value)
            .bind(course.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_course(&self, course_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM Courses WHERE Courses.id = $1")
            .bind(course_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
