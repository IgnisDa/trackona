use std::fs;

use async_graphql::Result;
use csv::Reader;
use database::{MetadataLot, MetadataSource};
use itertools::Itertools;
use serde::Deserialize;

use crate::{
    importer::{DeployGoodreadsImportInput, ImportFailStep, ImportFailedItem, ImportResult},
    providers::openlibrary::OpenlibraryService,
};

#[derive(Debug, Deserialize)]
struct Book {
    #[serde(rename = "Book Id")]
    id: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "ISBN13")]
    isbn13: String,
    #[serde(rename = "My Rating")]
    my_rating: i32,
    #[serde(rename = "Date Read")]
    date_read: Option<String>,
    #[serde(rename = "Bookshelves")]
    bookshelves: String,
    #[serde(rename = "My Review")]
    my_review: Option<String>,
    #[serde(rename = "Read Count")]
    read_count: i32,
}

pub async fn import(
    input: DeployGoodreadsImportInput,
    openlibrary_service: &OpenlibraryService,
) -> Result<ImportResult> {
    let lot = MetadataLot::Book;
    let source = MetadataSource::Openlibrary;
    let mut media = vec![];
    let mut failed_items = vec![];
    let export = fs::read_to_string(&input.csv_path)?;
    let ratings_reader = Reader::from_reader(export.as_bytes())
        .deserialize()
        .collect_vec();
    let total = ratings_reader.len();
    for (idx, result) in ratings_reader.into_iter().enumerate() {
        let record: Book = match result {
            Ok(r) => r,
            Err(e) => {
                failed_items.push(ImportFailedItem {
                    lot,
                    step: ImportFailStep::InputTransformation,
                    identifier: idx.to_string(),
                    error: Some(e.to_string()),
                });
                continue;
            }
        };
        tracing::debug!(
            "Getting details for {title:?} ({idx}/{total})",
            title = record.title
        );
        let isbn = record.isbn13[2..record.isbn13.len() - 1].to_owned();
        if let Some(identifier) = openlibrary_service.id_from_isbn(&isbn).await {
            dbg!(&identifier);
        } else {
            failed_items.push(ImportFailedItem {
                lot,
                step: ImportFailStep::InputTransformation,
                identifier: record.title,
                error: Some(format!(
                    "Could not convert ISBN: {} to Openlibrary ID",
                    isbn,
                )),
            })
        }
    }
    Ok(ImportResult {
        collections: vec![],
        media,
        failed_items,
        workouts: vec![],
    })
}
