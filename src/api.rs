use anyhow::Result;
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use std::fmt;

const HN_API_BASE: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Story {
    pub id: i64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub score: i64,
    pub by: String,
    pub time: i64,
    pub descendant: Option<i64>,
    pub kids: Option<Vec<i64>>,
    #[serde(default)]
    pub r#type: String,
    pub text: Option<String>,
}

impl Story {
    pub fn domain(&self) -> String {
        match &self.url {
            Some(url) => {
                let domain = url
                    .replace("https://", "")
                    .replace("http://", "")
                    .split('/')
                    .next()
                    .unwrap_or("")
                    .to_string();
                if domain.is_empty() {
                    "news.ycombinator.com".to_string()
                } else {
                    domain
                }
            }
            None => "news.ycombinator.com".to_string(),
        }
    }

    pub fn time_ago(&self) -> String {
        let dt = Utc.timestamp_opt(self.time, 0).unwrap();
        let now = Utc::now();
        let duration = now.signed_duration_since(dt);

        let seconds = duration.num_seconds();
        if seconds < 60 {
            format!("{}s ago", seconds)
        } else if seconds < 3600 {
            format!("{}m ago", seconds / 60)
        } else if seconds < 86400 {
            format!("{}h ago", seconds / 3600)
        } else {
            format!("{}d ago", seconds / 86400)
        }
    }
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} points by {} {} | {} comments",
            self.score,
            self.by,
            self.time_ago(),
            self.descendant.unwrap_or(0)
        )
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum StoryType {
    Top,
    New,
    Best,
    Show,
    Ask,
}

impl StoryType {
    fn url(&self) -> String {
        match self {
            StoryType::Top => format!("{}/topstories.json", HN_API_BASE),
            StoryType::New => format!("{}/newstories.json", HN_API_BASE),
            StoryType::Best => format!("{}/beststories.json", HN_API_BASE),
            StoryType::Show => format!("{}/showstories.json", HN_API_BASE),
            StoryType::Ask => format!("{}/askstories.json", HN_API_BASE),
        }
    }
}

pub struct HackerNewsClient {
    client: reqwest::Client,
}

impl HackerNewsClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_story_ids(&self, story_type: StoryType) -> Result<Vec<i64>> {
        let url = story_type.url();
        let ids: Vec<i64> = self.client.get(&url).send().await?.json().await?;
        Ok(ids)
    }

    #[allow(dead_code)]
    pub async fn get_story(&self, id: i64) -> Result<Story> {
        let url = format!("{}/item/{}.json", HN_API_BASE, id);
        let story: Story = self.client.get(&url).send().await?.json().await?;
        Ok(story)
    }

    pub async fn get_stories_by_ids(&self, ids: &[i64]) -> Result<Vec<Story>> {
        let mut stories = Vec::with_capacity(ids.len());
        let client = self.client.clone();

        let chunk_size = 10;
        for chunk in ids.chunks(chunk_size) {
            let futures: Vec<_> = chunk.iter().map(|&id| {
                let url = format!("{}/item/{}.json", HN_API_BASE, id);
                let client = client.clone();
                async move {
                    let res = client.get(&url).send().await?;
                    res.json::<Story>().await.map_err(anyhow::Error::from)
                }
            }).collect();

            let results: Vec<Result<Story>> = futures::future::join_all(futures).await;
            for result in results {
                if let Ok(story) = result {
                    stories.push(story);
                }
            }
        }

        Ok(stories)
    }

    #[allow(dead_code)]
    pub async fn get_stories(&self, story_type: StoryType, limit: Option<usize>) -> Result<Vec<Story>> {
        let ids = self.get_story_ids(story_type).await?;
        let limit = limit.unwrap_or(ids.len());

        let mut stories = Vec::with_capacity(limit);
        let client = self.client.clone();

        let chunk_size = 10;
        for chunk in ids[..limit].chunks(chunk_size) {
            let futures: Vec<_> = chunk.iter().map(|&id| {
                let url = format!("{}/item/{}.json", HN_API_BASE, id);
                let client = client.clone();
                async move {
                    let res = client.get(&url).send().await?;
                    res.json::<Story>().await.map_err(anyhow::Error::from)
                }
            }).collect();

            let results: Vec<Result<Story>> = futures::future::join_all(futures).await;
            for result in results {
                if let Ok(story) = result {
                    stories.push(story);
                }
            }
        }

        Ok(stories)
    }
}

impl Default for HackerNewsClient {
    fn default() -> Self {
        Self::new()
    }
}
