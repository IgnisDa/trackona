use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use application_utils::get_base_http_client;
use async_trait::async_trait;
use chrono::Datelike;
use common_models::{
    ApplicationCacheKey, IdObject, NamedObject, PersonSourceSpecifics, SearchDetails, StoredUrl,
};
use common_utils::{ryot_log, PAGE_SIZE};
use database_models::metadata_group::MetadataGroupWithoutId;
use dependent_models::{ApplicationCacheValue, IgdbSettings, SearchResults};
use enums::{MediaLot, MediaSource};
use itertools::Itertools;
use media_models::{
    MetadataDetails, MetadataGroupSearchItem, MetadataImageForMediaDetails, MetadataPerson,
    MetadataPersonRelated, MetadataSearchItem, MetadataVideo, MetadataVideoSource,
    PartialMetadataPerson, PartialMetadataWithoutId, PeopleSearchItem, VideoGameSpecifics,
};
use reqwest::{
    header::{HeaderName, HeaderValue, AUTHORIZATION},
    Client,
};
use rust_decimal::Decimal;
use rust_iso3166::from_numeric;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{formats::Flexible, serde_as, TimestampSeconds};
use supporting_service::SupportingService;
use traits::{MediaProvider, MediaProviderLanguages};

static URL: &str = "https://api.igdb.com/v4";
static IMAGE_URL: &str = "https://images.igdb.com/igdb/image/upload";
static AUTH_URL: &str = "https://id.twitch.tv/oauth2/token";

static GAME_FIELDS: &str = "
fields
    id,
    name,
    summary,
    cover.*,
    first_release_date,
    involved_companies.company.name,
    involved_companies.company.logo.*,
    involved_companies.*,
    artworks.*,
    rating,
    similar_games.id,
    similar_games.name,
    similar_games.cover.*,
    platforms.name,
    collection.id,
    videos.*,
    genres.*;
where version_parent = null;
";
static INVOLVED_COMPANY_FIELDS: &str = "
fields
    *,
    company.id,
    company.name,
    company.country,
    company.description,
    company.logo.*,
    company.websites.url,
    company.start_date,
    company.url,
    company.developed.id,
    company.developed.name,
    company.developed.cover.image_id,
    company.published.id,
    company.published.name,
    company.published.cover.image_id;
";
static COMPANY_FIELDS: &str = "
fields
    id,
    name,
    logo.*,
    start_date;
";
static COLLECTION_FIELDS: &str = "
fields
    id,
    name,
    games.id,
    games.name,
    games.cover.*,
    games.version_parent;
";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IgdbWebsite {
    url: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct IgdbCompany {
    id: i32,
    name: String,
    logo: Option<IgdbImage>,
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    start_date: Option<DateTimeUtc>,
    country: Option<i32>,
    description: Option<String>,
    websites: Option<Vec<IgdbWebsite>>,
    developed: Option<Vec<IgdbItemResponse>>,
    published: Option<Vec<IgdbItemResponse>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IgdbVideo {
    video_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IgdbInvolvedCompany {
    id: i32,
    company: IgdbCompany,
    developer: bool,
    publisher: bool,
    porting: bool,
    supporting: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IgdbImage {
    image_id: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct IgdbItemResponse {
    id: i32,
    name: Option<String>,
    rating: Option<Decimal>,
    games: Option<Vec<IgdbItemResponse>>,
    summary: Option<String>,
    cover: Option<IgdbImage>,
    #[serde_as(as = "Option<TimestampSeconds<i64, Flexible>>")]
    first_release_date: Option<DateTimeUtc>,
    involved_companies: Option<Vec<IgdbInvolvedCompany>>,
    videos: Option<Vec<IgdbVideo>>,
    artworks: Option<Vec<IgdbImage>>,
    genres: Option<Vec<NamedObject>>,
    platforms: Option<Vec<NamedObject>>,
    similar_games: Option<Vec<IgdbItemResponse>>,
    version_parent: Option<i32>,
    collection: Option<IdObject>,
    #[serde(flatten)]
    rest_data: Option<HashMap<String, Value>>,
}

#[derive(Clone)]
pub struct IgdbService {
    image_url: String,
    image_size: String,
    supporting_service: Arc<SupportingService>,
}

impl MediaProviderLanguages for IgdbService {
    fn supported_languages() -> Vec<String> {
        ["us"].into_iter().map(String::from).collect()
    }

    fn default_language() -> String {
        "us".to_owned()
    }
}

impl IgdbService {
    pub async fn new(ss: Arc<SupportingService>) -> Self {
        let config = ss.config.video_games.clone();
        Self {
            supporting_service: ss,
            image_url: IMAGE_URL.to_owned(),
            image_size: config.igdb.image_size.to_string(),
        }
    }
}

#[async_trait]
impl MediaProvider for IgdbService {
    #[allow(unused_variables)]
    async fn metadata_group_search(
        &self,
        query: &str,
        page: Option<i32>,
        _display_nsfw: bool,
    ) -> Result<SearchResults<MetadataGroupSearchItem>> {
        let client = self.get_client_config().await?;
        let req_body = format!(
            r#"
{fields}
search "{query}";
limit {limit};
offset: {offset};
            "#,
            fields = COLLECTION_FIELDS,
            query = query,
            limit = PAGE_SIZE,
            offset = (page.unwrap_or(1) - 1) * PAGE_SIZE
        );
        let rsp = client
            .post(format!("{}/collections", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let details: Vec<IgdbItemResponse> = rsp.json().await.map_err(|e| anyhow!(e))?;
        let resp = details
            .into_iter()
            .map(|d| MetadataGroupSearchItem {
                identifier: d.id.to_string(),
                name: d.name.unwrap(),
                image: d.cover.map(|c| self.get_cover_image_url(c.image_id)),
                parts: d.games.map(|g| g.len()),
            })
            .collect_vec();
        Ok(SearchResults {
            details: SearchDetails {
                total: resp.len().try_into().unwrap(),
                next_page: Some(page.unwrap_or(1) + 1),
            },
            items: resp,
        })
    }

    async fn metadata_group_details(
        &self,
        identifier: &str,
    ) -> Result<(MetadataGroupWithoutId, Vec<PartialMetadataWithoutId>)> {
        let client = self.get_client_config().await?;
        let req_body = format!(
            r"
{fields}
where id = {id};
            ",
            fields = COLLECTION_FIELDS,
            id = identifier
        );
        let details: IgdbItemResponse = client
            .post(format!("{}/collections", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?
            .json::<Vec<_>>()
            .await
            .map_err(|e| anyhow!(e))?
            .pop()
            .unwrap();
        let items = details
            .games
            .unwrap_or_default()
            .into_iter()
            .flat_map(|g| {
                if g.version_parent.is_some() {
                    None
                } else {
                    Some(PartialMetadataWithoutId {
                        identifier: g.id.to_string(),
                        title: g.name.unwrap(),
                        image: g.cover.map(|c| self.get_cover_image_url(c.image_id)),
                        source: MediaSource::Igdb,
                        lot: MediaLot::VideoGame,
                        is_recommendation: None,
                    })
                }
            })
            .collect_vec();
        Ok((
            MetadataGroupWithoutId {
                display_images: vec![],
                parts: items.len().try_into().unwrap(),
                identifier: details.id.to_string(),
                title: details.name.unwrap_or_default(),
                description: None,
                images: vec![],
                lot: MediaLot::VideoGame,
                source: MediaSource::Igdb,
            },
            items,
        ))
    }

    async fn people_search(
        &self,
        query: &str,
        page: Option<i32>,
        _source_specifics: &Option<PersonSourceSpecifics>,
        _display_nsfw: bool,
    ) -> Result<SearchResults<PeopleSearchItem>> {
        let client = self.get_client_config().await?;
        let req_body = format!(
            r#"
{fields}
search "{query}";
limit {limit};
offset: {offset};
            "#,
            fields = COMPANY_FIELDS,
            query = query,
            limit = PAGE_SIZE,
            offset = (page.unwrap_or(1) - 1) * PAGE_SIZE
        );
        let rsp = client
            .post(format!("{}/companies", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let details: Vec<IgdbCompany> = rsp.json().await.map_err(|e| anyhow!(e))?;
        let resp = details
            .into_iter()
            .map(|ic| {
                let image = ic.logo.map(|a| self.get_cover_image_url(a.image_id));
                PeopleSearchItem {
                    identifier: ic.id.to_string(),
                    name: ic.name,
                    image,
                    birth_year: None,
                }
            })
            .collect_vec();
        Ok(SearchResults {
            details: SearchDetails {
                total: resp.len().try_into().unwrap(),
                next_page: Some(page.unwrap_or(1) + 1),
            },
            items: resp,
        })
    }

    async fn person_details(
        &self,
        identity: &str,
        _source_specifics: &Option<PersonSourceSpecifics>,
    ) -> Result<MetadataPerson> {
        let client = self.get_client_config().await?;
        let req_body = format!(
            r#"
{fields}
where id = {id};
            "#,
            fields = INVOLVED_COMPANY_FIELDS,
            id = identity
        );
        let rsp = client
            .post(format!("{}/involved_companies", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        let mut details: Vec<IgdbInvolvedCompany> = rsp.json().await.map_err(|e| anyhow!(e))?;
        let detail = details
            .pop()
            .map(|ic| ic.company)
            .ok_or_else(|| anyhow!("No data"))?;
        let mut related = detail
            .published
            .unwrap_or_default()
            .into_iter()
            .map(|r| {
                let image = r.cover.map(|a| self.get_cover_image_url(a.image_id));
                MetadataPersonRelated {
                    character: None,
                    role: "Publishing".to_owned(),
                    metadata: PartialMetadataWithoutId {
                        image,
                        title: r.name.unwrap(),
                        is_recommendation: None,
                        lot: MediaLot::VideoGame,
                        source: MediaSource::Igdb,
                        identifier: r.id.to_string(),
                    },
                }
            })
            .collect_vec();
        related.extend(detail.developed.unwrap_or_default().into_iter().map(|r| {
            let image = r.cover.map(|a| self.get_cover_image_url(a.image_id));
            MetadataPersonRelated {
                character: None,
                role: "Development".to_owned(),
                metadata: PartialMetadataWithoutId {
                    image,
                    title: r.name.unwrap(),
                    is_recommendation: None,
                    lot: MediaLot::VideoGame,
                    source: MediaSource::Igdb,
                    identifier: r.id.to_string(),
                },
            }
        }));
        Ok(MetadataPerson {
            identifier: detail.id.to_string(),
            name: detail.name,
            images: Some(Vec::from_iter(
                detail.logo.map(|l| self.get_cover_image_url(l.image_id)),
            )),
            source: MediaSource::Igdb,
            description: detail.description,
            place: detail
                .country
                .and_then(from_numeric)
                .map(|c| c.name.to_owned()),
            website: detail
                .websites
                .unwrap_or_default()
                .first()
                .map(|i| i.url.clone()),
            related,
            birth_date: None,
            death_date: None,
            gender: None,
            source_specifics: None,
        })
    }

    async fn metadata_details(&self, identifier: &str) -> Result<MetadataDetails> {
        let client = self.get_client_config().await?;
        let req_body = format!(
            r#"{field} where id = {id};"#,
            field = GAME_FIELDS,
            id = identifier
        );
        ryot_log!(debug, "Body = {}", req_body);
        let rsp = client
            .post(format!("{}/games", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;
        ryot_log!(debug, "Response = {:?}", rsp);
        let mut details: Vec<IgdbItemResponse> = rsp.json().await.map_err(|e| anyhow!(e))?;
        let detail = details.pop().ok_or_else(|| anyhow!("No details found"))?;
        let groups = match detail.collection.as_ref() {
            Some(c) => vec![c.id.to_string()],
            None => vec![],
        };
        let mut game_details = self.igdb_response_to_search_response(detail);
        game_details.group_identifiers = groups;
        Ok(game_details)
    }

    async fn metadata_search(
        &self,
        query: &str,
        page: Option<i32>,
        _display_nsfw: bool,
    ) -> Result<SearchResults<MetadataSearchItem>> {
        let page = page.unwrap_or(1);
        let client = self.get_client_config().await?;
        let count_req_body =
            format!(r#"fields id; where version_parent = null; search "{query}"; limit: 500;"#);
        let rsp = client
            .post(format!("{}/games", URL))
            .body(count_req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        let search_count_resp: Vec<IgdbItemResponse> = rsp.json().await.map_err(|e| anyhow!(e))?;

        let total = search_count_resp.len().try_into().unwrap();

        let req_body = format!(
            r#"
{field}
search "{query}";
limit {limit};
offset: {offset};
            "#,
            field = GAME_FIELDS,
            limit = PAGE_SIZE,
            offset = (page - 1) * PAGE_SIZE
        );
        let rsp = client
            .post(format!("{}/games", URL))
            .body(req_body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        let search: Vec<IgdbItemResponse> = rsp.json().await.map_err(|e| anyhow!(e))?;

        let resp = search
            .into_iter()
            .map(|r| {
                let a = self.igdb_response_to_search_response(r);
                MetadataSearchItem {
                    identifier: a.identifier,
                    title: a.title,
                    image: a.url_images.first().map(|i| i.image.clone()),
                    publish_year: a.publish_year,
                }
            })
            .collect_vec();

        Ok(SearchResults {
            details: SearchDetails {
                total,
                next_page: Some(page + 1),
            },
            items: resp,
        })
    }
}

impl IgdbService {
    async fn get_access_token(&self) -> String {
        let client = Client::new();
        #[derive(Deserialize, Serialize, Default, Debug)]
        struct AccessResponse {
            access_token: String,
            token_type: String,
            expires_in: u128,
        }
        let access_res = client
            .post(AUTH_URL)
            .query(&json!({
                "grant_type": "client_credentials".to_owned(),
                "client_id": self.supporting_service.config.video_games.twitch.client_id.to_owned(),
                "client_secret": self.supporting_service.config.video_games.twitch.client_secret.to_owned(),
            }))
            .send()
            .await
            .unwrap();
        let access = access_res
            .json::<AccessResponse>()
            .await
            .unwrap_or_default();
        format!("{} {}", access.token_type, access.access_token)
    }

    async fn get_client_config(&self) -> Result<Client> {
        let cc = &self.supporting_service.cache_service;
        let maybe_settings = cc.get_key(ApplicationCacheKey::IgdbSettings).await.ok();
        let access_token =
            if let Some(Some(ApplicationCacheValue::IgdbSettings(value))) = maybe_settings {
                value.access_token
            } else {
                let access_token = self.get_access_token().await;
                cc.set_with_expiry(
                    ApplicationCacheKey::IgdbSettings,
                    ApplicationCacheValue::IgdbSettings(IgdbSettings {
                        access_token: access_token.clone(),
                    }),
                )
                .await
                .ok();
                access_token
            };
        Ok(get_base_http_client(Some(vec![
            (
                HeaderName::from_static("client-id"),
                HeaderValue::from_str(&self.supporting_service.config.video_games.twitch.client_id)
                    .unwrap(),
            ),
            (AUTHORIZATION, HeaderValue::from_str(&access_token).unwrap()),
        ])))
    }

    fn igdb_response_to_search_response(&self, item: IgdbItemResponse) -> MetadataDetails {
        let mut images = Vec::from_iter(item.cover.map(|a| MetadataImageForMediaDetails {
            image: self.get_cover_image_url(a.image_id),
        }));
        let additional_images =
            item.artworks
                .unwrap_or_default()
                .into_iter()
                .map(|a| MetadataImageForMediaDetails {
                    image: self.get_cover_image_url(a.image_id),
                });
        images.extend(additional_images);
        let people = item
            .involved_companies
            .into_iter()
            .flatten()
            .map(|ic| {
                let role = if ic.developer {
                    "Development"
                } else if ic.publisher {
                    "Publishing"
                } else if ic.porting {
                    "Porting"
                } else if ic.supporting {
                    "Supporting"
                } else {
                    "Unknown"
                };
                PartialMetadataPerson {
                    identifier: ic.id.to_string(),
                    name: ic.company.name,
                    source: MediaSource::Igdb,
                    role: role.to_owned(),
                    character: None,
                    source_specifics: None,
                }
            })
            .unique()
            .collect();
        let videos = item
            .videos
            .unwrap_or_default()
            .into_iter()
            .map(|vid| MetadataVideo {
                identifier: StoredUrl::Url(vid.video_id),
                source: MetadataVideoSource::Youtube,
            })
            .collect_vec();
        MetadataDetails {
            identifier: item.id.to_string(),
            lot: MediaLot::VideoGame,
            source: MediaSource::Igdb,
            title: item.name.unwrap(),
            description: item.summary,
            people,
            url_images: images,
            videos,
            publish_date: item.first_release_date.map(|d| d.date_naive()),
            publish_year: item.first_release_date.map(|d| d.year()),
            genres: item
                .genres
                .unwrap_or_default()
                .into_iter()
                .map(|g| g.name)
                .unique()
                .collect(),
            video_game_specifics: Some(VideoGameSpecifics {
                platforms: item
                    .platforms
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| p.name)
                    .collect(),
            }),
            suggestions: item
                .similar_games
                .unwrap_or_default()
                .into_iter()
                .map(|g| PartialMetadataWithoutId {
                    title: g.name.unwrap(),
                    image: g.cover.map(|c| self.get_cover_image_url(c.image_id)),
                    identifier: g.id.to_string(),
                    lot: MediaLot::VideoGame,
                    source: MediaSource::Igdb,
                    is_recommendation: None,
                })
                .collect(),
            provider_rating: item.rating,
            ..Default::default()
        }
    }

    fn get_cover_image_url(&self, hash: String) -> String {
        format!("{}/{}/{}.jpg", self.image_url, self.image_size, hash)
    }
}
