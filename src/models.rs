use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub ident: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewTodo {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditTodo {
    pub ident: uuid::Uuid,
    pub name: String,
}

impl Todo {
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Todo>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
SELECT id
     , ident
     , name
  FROM todos.main
;
            "#
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn new(pool: &sqlx::PgPool, name: String) -> Result<Todo> {
        let res = sqlx::query!(
            r#"
INSERT INTO todos.main (name)
     VALUES ($1)
  RETURNING id, ident
;
                "#,
            name,
        )
        .fetch_one(pool)
        .await?;
        Ok(Todo {
            id: res.id,
            ident: res.ident,
            name,
        })
    }

    pub async fn get(pool: &sqlx::PgPool, ident: uuid::Uuid) -> Result<Todo> {
        Ok(
            sqlx::query_as!(Todo, r#"SELECT * FROM todos.main WHERE ident = $1;"#, ident,)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn edit(pool: &PgPool, ident: uuid::Uuid, name: String) -> Result<Todo> {
        let res = sqlx::query!(
            r#"
   UPDATE todos.main
      SET name = $1
    WHERE ident = $2
RETURNING id
;
            "#,
            &name,
            &ident,
        )
        .fetch_one(pool)
        .await?;
        Ok(Todo {
            id: res.id,
            ident,
            name,
        })
    }

    pub async fn delete(pool: &sqlx::PgPool, ident: uuid::Uuid) -> Result<usize> {
        Ok(sqlx::query!(
            r#"
DELETE FROM todos.main
      WHERE ident = $1
;
            "#,
            &ident,
        )
        .execute(pool)
        .await?
        .rows_affected() as usize)
    }

    pub fn render_display(&self) -> lfml::Markup {
        lfml::html! {
            li data-todo-id=(self.ident) {
                ."" {
                    p { (&self.name) }
                    ."" {
                        button hx-get=(format!("/todos/{}/edit", self.ident))
                            hx-target=(format!("[data-todo-id=\"{}\"]", self.ident))
                            hx-swap="outerHTML"
                        {
                            "\u{1F58D}\u{FE0F}"
                        }
                        button hx-delete=(format!("/todos/{}", self.ident))
                            hx-target=(format!("[data-todo-id=\"{}\"]", self.ident))
                            hx-swap="delete"
                        {
                            "\u{274C}"
                        }
                    }
                }
            }
        }
    }

    pub fn render_edit(&self) -> lfml::Markup {
        lfml::html! {
            li data-todo-id=(self.ident) {
                form hx-put=(format!("/todos/{}", self.ident)) {
                    // aria-labelledby
                    input name="ident" hidden value=(self.ident);
                    input name="name" type="text" value=(&self.name);
                    ."" {
                        button hx-get=(format!("/todos/{}", self.ident))
                            hx-target=(format!("[data-todo-id=\"{}\"]", self.ident))
                            hx-swap="outerHTML"
                        {
                            "\u{21A9}\u{FE0F}"
                        }
                        button hx-put=(format!("/todos/{}", self.ident))
                            hx-target=(format!("[data-todo-id=\"{}\"]", self.ident))
                            hx-swap="outerHTML"
                        {
                            "\u{2714}\u{FE0F}"
                        }
                    }
                }
            }
        }
    }
}
