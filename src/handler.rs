use std::convert::Infallible;

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::Document, bson::to_document, options::FindOptions};
use warp::{http::StatusCode};

use crate::model::book::Book;
use crate::model::database::MongoDB;
use crate::model::list_option::ListOptions;

pub async fn insert_one(mongo: MongoDB, book: Book) -> Result<impl warp::Reply, Infallible> {
    let collection = mongo.db.collection::<Document>("books");
    let doc = to_document(&book).expect("Error parsing");
    collection.insert_one(doc, None).await.expect("Error inserting");
    Ok(StatusCode::CREATED)
}

pub async fn list_books(mongo: MongoDB, list_filter: ListOptions) -> Result<impl warp::Reply, Infallible> {
    let collection = mongo.db.collection::<Document>("books");
    let filter =
        if list_filter.sort == "asc" {
            doc! { list_filter.field.clone(): 1 }
        } else if list_filter.sort == "desc" {
            doc! { list_filter.field.clone(): -1 }
        } else {
            Document::new()
        };
    let mut f = FindOptions::default();
    f.sort = Option::from(filter.clone());
    let mut cursor = collection.find(None, f).await.expect("Error finding cursor");
    let mut result = Vec::new();
    while let Some(book) = cursor.try_next().await.expect("Error reading doc") {
        result.push(book);
    }
    Ok(warp::reply::json(&mut result))
}

