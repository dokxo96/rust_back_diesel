#[macro_use]

extern crate diesel;
pub mod schema;
pub mod models;


use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::post;


fn main(){
    dotenv().ok();

    let db_url=env::var("DATABASE_URL").expect("DB urlNotFound");

    let conn=PgConnection::establish(&db_url).expect("cannot connecto to the DB");
    println!("{}",db_url);



    use self::schema::post::dsl::*;
    use self::schema::post;
    use self::models::{Post,NewPost};


    let np =NewPost{
        title:"mi tercer post",
        body:"lorem iptsu",
        slug:"primero post",
    };

    //insert

    let postne:Post=diesel::insert_into(post::table).values(&np).get_result(&conn).expect("Insert error");

    let posts_res =post.load::<Post>(&conn).expect("Query error");

    for pst in posts_res{
        println!("{}",pst.title);
    }

}
