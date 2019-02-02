#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewPost, Post};
use diesel::result::Error;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use self::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn all_posts(conn: PgConnection, include_unpublished: bool) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    posts.filter(published.eq(!include_unpublished))
        .load::<Post>(&conn)
        .expect("Error loading posts")
}

pub fn get_post(conn: &PgConnection, id: i32) -> Post {
    use self::schema::posts::dsl::{posts};

    posts.find(id)
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use self::schema::posts::dsl::{posts};

    diesel::delete(posts.find(id)).execute(conn)
}
