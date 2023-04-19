use anyhow::{anyhow, Result};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use surf::{http::headers::USER_AGENT, Client, Config, Url};
use tokio::task::JoinSet;

use crate::books::resolver::{Book, BookSearch};

static LIMIT: i32 = 20;

#[derive(Debug, Clone)]
pub struct OpenlibraryService {
    image_url: String,
    image_size: String,
    client: Client,
}

impl OpenlibraryService {
    pub fn new(url: &str, image_url: &str, image_size: &str) -> Self {
        let client = Config::new()
            .add_header(USER_AGENT, "ignisda/trackona")
            .unwrap()
            .set_base_url(Url::parse(url).unwrap())
            .try_into()
            .unwrap();
        Self {
            image_url: image_url.to_owned(),
            image_size: image_size.to_owned(),
            client,
        }
    }
}

impl OpenlibraryService {
    pub async fn details(
        &self,
        identifier: &str,
        query: &str,
        offset: Option<i32>,
        index: i32,
    ) -> Result<Book> {
        let mut detail = self.search(query, offset).await?.books[index as usize].clone();
        #[derive(Debug, Serialize, Deserialize, Clone)]
        struct OpenlibraryKey {
            key: String,
        }
        #[derive(Debug, Serialize, Deserialize, Clone)]
        struct OpenlibraryAuthor {
            author: OpenlibraryKey,
        }
        #[derive(Debug, Serialize, Deserialize, Clone)]
        struct OpenlibraryBook {
            description: Option<String>,
            covers: Vec<i64>,
            authors: Vec<OpenlibraryAuthor>,
        }
        let mut rsp = self
            .client
            .get(format!("works/{}.json", identifier))
            .await
            .map_err(|e| anyhow!(e))?;
        let data: OpenlibraryBook = rsp.body_json().await.map_err(|e| anyhow!(e))?;
        let mut set = JoinSet::new();
        #[derive(Debug, Serialize, Deserialize)]
        struct OpenlibraryAuthorPartial {
            name: String,
        }
        for author in data.authors.into_iter() {
            let client = self.client.clone();
            set.spawn(async move {
                let mut rsp = client
                    .get(format!("{}.json", author.author.key))
                    .await
                    .unwrap();
                let OpenlibraryAuthorPartial { name } = rsp.body_json().await.unwrap();
                name
            });
        }
        let mut authors = vec![];
        while let Some(Ok(result)) = set.join_next().await {
            authors.push(result);
        }
        detail.description = data.description;
        detail.images = data
            .covers
            .into_iter()
            .map(|c| self.get_cover_image_url(c))
            .collect();
        detail.author_names = authors;
        Ok(detail)
    }

    pub async fn search(&self, query: &str, offset: Option<i32>) -> Result<BookSearch> {
        #[derive(Serialize, Deserialize)]
        struct Query {
            q: String,
            fields: String,
            offset: i32,
            limit: i32,
            #[serde(rename = "type")]
            lot: String,
        }
        #[derive(Debug, Serialize, Deserialize, SimpleObject)]
        pub struct OpenlibraryBook {
            key: String,
            title: String,
            author_name: Option<Vec<String>>,
            cover_i: Option<i64>,
            publish_year: Option<Vec<i32>>,
            first_publish_year: Option<i32>,
            number_of_pages_median: Option<i32>,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct OpenLibrarySearchResponse {
            num_found: i32,
            docs: Vec<OpenlibraryBook>,
        }

        let mut rsp = self
            .client
            .get("search.json")
            .query(&Query {
                q: query.to_owned(),
                fields: [
                    "key",
                    "title",
                    "author_name",
                    "cover_i",
                    "publish_year",
                    "first_publish_year",
                    "number_of_pages_median",
                ]
                .join(","),
                offset: offset.unwrap_or_default(),
                limit: LIMIT,
                lot: "work".to_owned(),
            })
            .unwrap()
            .await
            .map_err(|e| anyhow!(e))?;
        let search: OpenLibrarySearchResponse = rsp.body_json().await.map_err(|e| anyhow!(e))?;

        let resp = search
            .docs
            .into_iter()
            .map(|d| {
                let images = if let Some(c) = d.cover_i {
                    vec![self.get_cover_image_url(c)]
                } else {
                    vec![]
                };
                Book {
                    identifier: d.key,
                    title: d.title,
                    description: None,
                    author_names: d.author_name.unwrap_or_default(),
                    publish_year: d.first_publish_year,
                    num_pages: d.number_of_pages_median,
                    images,
                }
            })
            .collect::<Vec<_>>();
        Ok(BookSearch {
            total: search.num_found,
            books: resp,
            limit: LIMIT,
        })
    }

    fn get_cover_image_url(&self, c: i64) -> String {
        format!(
            "{}/id/{}-{}.jpg?default=false",
            self.image_url, c, self.image_size
        )
    }
}
