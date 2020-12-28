use std::cmp::Ordering;

use crate::utils::chain_ordering;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeckCategory {
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "BUDGET")]
    Budget,
    #[serde(rename = "FEATURED")]
    Featured,
}

impl Default for DeckCategory {
    fn default() -> Self {
        DeckCategory::Community
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeckSort {
    #[serde(rename = "recently_updated")]
    RecentlyUpdated,
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "popularity")]
    Popularity,
}

impl Default for DeckSort {
    fn default() -> Self {
        DeckSort::RecentlyUpdated
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckQuery {
    #[serde(rename(serialize = "sortBy"))]
    pub sort_by: DeckSort,
    pub from: u32,
    pub count: u32,
    pub category: DeckCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    uid: String,
    title: String,
    description: String,
    #[serde(rename = "exportUID")]
    export: String,
    mode: String,
    #[serde(rename = "playStyle")]
    playstyle: String,
    created_at: u64,
    changed_at: u64,
    rating: i32,
    is_private: bool,
    is_draft: bool,
    is_riot: bool,
}

impl Eq for Deck {}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        (self.uid == other.uid) & (self.export == other.export)
    }
}

impl Ord for Deck {
    fn cmp(&self, other: &Self) -> Ordering {
        chain_ordering(self.uid.cmp(&other.uid), self.export.cmp(&other.export))
    }
}

impl PartialOrd for Deck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeckData {
    has_next: bool,
    pub decks: Vec<Deck>,
}
