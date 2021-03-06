use async_trait::async_trait;
use mockall::*;

use crate::domain::entities::{SubjectEntity, SubjectSearchTermsEntity};

#[derive(Debug, PartialEq)]
pub enum SubjectSearchScheduleOption {
    None,
    Fixed {
        date: Option<u32>,
        hour: Option<u32>,
    },
    Intensive,
}

#[derive(Debug, PartialEq)]
pub struct SubjectSearchInput<'a> {
    pub from: u32,
    pub count: u32,
    pub title: Option<&'a str>,
    pub available_only: bool,
    pub schedule: SubjectSearchScheduleOption,
    pub semester: Option<&'a str>,
    pub year: Option<u32>,
    pub category: Option<&'a str>,
    pub faculty: Option<&'a str>,
    pub program: Option<&'a str>,
    pub field: Option<&'a str>,
}

#[derive(Debug)]
pub struct SubjectSearchOutput {
    pub total_count: u32,
    pub items: Vec<SubjectEntity>,
}

#[automock]
#[async_trait]
pub trait SubjectRepository: Sync + Send {
    async fn get_by_id(&self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
    async fn search<'b>(
        &self,
        input: SubjectSearchInput<'b>,
    ) -> Result<SubjectSearchOutput, anyhow::Error>;
    async fn get_terms(&self) -> Result<SubjectSearchTermsEntity, anyhow::Error>;
}
