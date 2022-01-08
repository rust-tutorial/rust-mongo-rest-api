use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use warp::Filter;

use crate::model::database::MongoDB;
use crate::model::list_option::ListOptions;
use crate::handler::{insert_one, list_books};

fn with_db(db: MongoDB) -> impl Filter<Extract=(MongoDB, ), Error=std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub async fn run(m: MongoDB, port: &u16) {
    let mongo = m.to_owned();
    let post_book =
        warp::post().and(warp::path("books"))
            .and(warp::path("add"))
            .and(warp::path::end())
            //.and(warp::path::param::<u32>())
            // Only accept bodies smaller than 16kb...
            .and(warp::body::content_length_limit(1024 * 16))
            .and(with_db(mongo.clone()))
            .and(warp::body::json())
            .and_then(insert_one);

    let list_books = warp::get()
        .and(warp::path("books"))
        .and(with_db(mongo.clone()))
        .and(warp::query::<ListOptions>()).and_then(list_books);

    let welcome = warp::path("welcome").map(|| "Welcome");
    let get = warp::get().and(
        welcome
            .or(list_books)
    );
    let post = warp::post().and(post_book);

    let routes = post.or(get);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port.clone());
    println!("HTTP server is listening at {}", port.clone().to_string());
    warp::serve(routes).run(socket).await;
}