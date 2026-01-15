use crate::api::{HackerNewsClient, Story, StoryType};
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum AppState {
    Loading,
    LoadingMore,
    Ready,
    Error(String),
}

pub struct App {
    pub stories: Vec<Story>,
    pub selected_index: usize,
    pub story_type: StoryType,
    pub state: AppState,
    pub error_message: Option<String>,
    pub scroll_offset: usize,
    pub show_details: bool,
    pub client: Arc<Mutex<HackerNewsClient>>,
    pub all_story_ids: Vec<i64>,
    pub loaded_count: usize,
    pub batch_size: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            stories: Vec::new(),
            selected_index: 0,
            story_type: StoryType::Top,
            state: AppState::Loading,
            error_message: None,
            scroll_offset: 0,
            show_details: false,
            client: Arc::new(Mutex::new(HackerNewsClient::new())),
            all_story_ids: Vec::new(),
            loaded_count: 0,
            batch_size: 30,
        }
    }

    pub async fn load_stories(&mut self) {
        self.state = AppState::Loading;
        self.error_message = None;

        let client = self.client.lock().await;
        match client.get_story_ids(self.story_type).await {
            Ok(ids) => {
                self.all_story_ids = ids;
                self.loaded_count = 0;
                let new_stories = client
                    .get_stories_by_ids(&self.all_story_ids[self.loaded_count..self.loaded_count.saturating_add(self.batch_size).min(self.all_story_ids.len())])
                    .await;
                match new_stories {
                    Ok(mut stories) => {
                        self.stories.append(&mut stories);
                        self.loaded_count = self.stories.len();
                        self.selected_index = 0;
                        self.scroll_offset = 0;
                        self.state = AppState::Ready;
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        self.state = AppState::Error(e.to_string());
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.state = AppState::Error(e.to_string());
            }
        }
    }

    pub async fn load_more_stories(&mut self) {
        if self.loaded_count >= self.all_story_ids.len() {
            return;
        }

        self.state = AppState::LoadingMore;
        let next_batch = self.loaded_count.saturating_add(self.batch_size);
        let slice_end = next_batch.min(self.all_story_ids.len());

        let ids_to_load = &self.all_story_ids[self.loaded_count..slice_end];

        if ids_to_load.is_empty() {
            self.state = AppState::Ready;
            return;
        }

        let client = self.client.lock().await;
        match client.get_stories_by_ids(ids_to_load).await {
            Ok(mut stories) => {
                self.stories.append(&mut stories);
                self.loaded_count = slice_end;
                self.state = AppState::Ready;
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.state = AppState::Error(e.to_string());
            }
        }
    }

    pub fn can_load_more(&self) -> bool {
        self.loaded_count < self.all_story_ids.len()
    }

    pub fn next_story(&mut self) {
        if !self.stories.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.stories.len() - 1);
            self.update_scroll();
        }
    }

    pub fn prev_story(&mut self) {
        if !self.stories.is_empty() {
            self.selected_index = self.selected_index.saturating_sub(1);
            self.update_scroll();
        }
    }

    pub fn page_down(&mut self) {
        if !self.stories.is_empty() {
            let page_size = 10;
            self.selected_index = (self.selected_index + page_size).min(self.stories.len() - 1);
            self.update_scroll();
        }
    }

    pub fn page_up(&mut self) {
        if !self.stories.is_empty() {
            let page_size = 10;
            self.selected_index = self.selected_index.saturating_sub(page_size);
            self.update_scroll();
        }
    }

    pub fn update_scroll(&mut self) {
        let visible_rows = 20;
        if self.selected_index >= self.scroll_offset + visible_rows {
            self.scroll_offset = self.selected_index - visible_rows + 1;
        } else if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    pub fn next_story_type(&mut self) {
        self.story_type = match self.story_type {
            StoryType::Top => StoryType::New,
            StoryType::New => StoryType::Best,
            StoryType::Best => StoryType::Show,
            StoryType::Show => StoryType::Ask,
            StoryType::Ask => StoryType::Top,
        };
    }

    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    pub fn selected_story(&self) -> Option<&Story> {
        self.stories.get(self.selected_index)
    }

    pub fn selected_story_url(&self) -> Option<&String> {
        self.selected_story().and_then(|s| s.url.as_ref())
    }

    pub fn has_selected_story_url(&self) -> bool {
        self.selected_story_url().is_some()
    }

    pub fn story_type_name(&self) -> &str {
        match self.story_type {
            StoryType::Top => "Top",
            StoryType::New => "New",
            StoryType::Best => "Best",
            StoryType::Show => "Show",
            StoryType::Ask => "Ask",
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
