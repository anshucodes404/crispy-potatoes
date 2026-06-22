use crate::model::*;
use sqlx::PgPool;

pub struct CredentialRepository {
    pub pool: PgPool,
}

impl CredentialRepository {
    pub fn new(pool: PgPool) -> Self {
        println!("Successfully connected to DB");
        Self { pool }
    }

    pub async fn insert(&self, credential: &NewCredential) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                INSERT INTO credentials
                (website_name, user_name, password)
                VALUES ($1, $2, $3);
            "#,
        )
        .bind(&credential.website_name)
        .bind(&credential.user_name)
        .bind(&credential.password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM credentials
                WHERE id = $1;
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_website_name(
        &self,
        website_name: String,
    ) -> Result<Vec<Credential>, sqlx::Error> {
        let credentials = sqlx::query_as::<_, Credential>(
            r#"
                SELECT * FROM credentials
                WHERE website_name = $1;
            "#,
        )
        .bind(website_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }

    pub async fn find_by_user_name(
        &self,
        user_name: String,
    ) -> Result<Vec<Credential>, sqlx::Error> {
        let credentials = sqlx::query_as::<_, Credential>(
            r#"
                SELECT * FROM credentials
                WHERE user_name = $1;
            "#,
        )
        .bind(user_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }

    pub async fn find_by_website_name_and_user_name(
        &self,
        website_name: String,
        user_name: String
    ) -> Result<Vec<Credential>, sqlx::Error> {
        let credentials = sqlx::query_as::<_, Credential>(
            r#"
                SELECT * FROM credentials
                WHERE website_name = $1 AND user_name = $2;
            "#,
        )
        .bind(website_name)
        .bind(user_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }

    pub async fn find_all(&self) -> Result<Vec<Credential>, sqlx::Error> {
        let credentials = sqlx::query_as::<_, Credential>(
            r#"
                SELECT * FROM credentials;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(credentials)
    }
}
