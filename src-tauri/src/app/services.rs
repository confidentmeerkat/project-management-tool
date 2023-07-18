extern crate mongodb;

use futures::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    sync::Collection,
};

pub struct Project {
    path: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone)]
pub struct ProjectService {
    collection: Collection<Document>,
}

impl ProjectService {
    pub fn new(collection: Collection<Document>) -> ProjectService {
        ProjectService { collection }
    }

    pub fn index(&self) -> Result<Vec<Document>, mongodb::error::Error> {
        match self.collection.find(doc! {}, None) {
            Err(err) => Err(err),
            Ok(cursor) => {
                let result: Vec<Result<Document, _>> = cursor.collect();

                Ok(result.into_iter().map(|item| item.unwrap()).collect())
            }
        }
    }
}
