use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetResponse<T> {
    pub found: bool,
    pub _id: String,
    pub _source: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchTotal {
    pub value: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SearchHitItem<T> {
    pub _id: String,
    pub _score: Option<f32>,
    pub _source: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceHits<T> {
    pub total: SearchTotal,
    pub max_score: Option<f32>,
    pub hits: Vec<SearchHitItem<T>>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse<T> {
    pub took: f32,
    pub timed_out: bool,
    pub hits: SourceHits<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationResponse<T> {
    pub aggregations: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationBucket<K, Sub> {
    pub buckets: Vec<AggregationBucketItem<K, Sub>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationBucketItem<K, Sub> {
    pub key: K,
    pub doc_count: i32,
    #[serde(flatten)]
    pub sub_aggs: Sub,
}
