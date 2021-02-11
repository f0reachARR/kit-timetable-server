use juniper::*;

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectClassPlan {
    pub topic: String,
    pub content: Option<String>,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectInstructor {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectGoalEvaluation {
    pub label: String,
    pub description: String,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectGoal {
    pub description: String,
    pub evaluations: Vec<GqlSubjectGoalEvaluation>,
}

#[derive(Debug, PartialEq, GraphQLObject)]
pub struct GqlSubjectFixedSchedule {
    pub date: i32,
    pub hour: i32,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum GqlSubjectScheduleType {
    Intensive,
    Fixed,
    Unknown,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectSchedule {
    #[graphql(name = "type")]
    pub schedule_type: GqlSubjectScheduleType,
    pub days: Vec<GqlSubjectFixedSchedule>,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectCategory {
    pub faculty: Option<String>,
    pub field: Option<String>,
    pub program: Option<String>,
    pub category: Option<String>,
    pub semester: String,
    pub available: bool,
    pub year: Vec<i32>,
    pub schedule: GqlSubjectSchedule,
}

#[derive(Debug, PartialEq, GraphQLEnum)]
pub enum GqlSubjectFlag {
    Internship,
    IGP,
    AL,
    PBL,
    PT,
    Univ3,
    Kyoto,
    Lottery,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectAttachment {
    pub key: String,
    pub name: String,
}

#[derive(Debug, GraphQLObject)]
pub struct GqlSubjectDto {
    pub id: i32,
    pub title: String,
    pub categories: Vec<GqlSubjectCategory>,
    pub instructors: Vec<GqlSubjectInstructor>,
    pub attachments: Vec<GqlSubjectAttachment>,
    pub flags: Vec<GqlSubjectFlag>,
    pub outline: String,
    pub purpose: String,
    pub plans: Vec<GqlSubjectClassPlan>,
    pub requirement: String,
    pub point: String,
    pub textbook: String,
    pub grading_policy: String,
    pub remark: String,
    pub research_plan: String,

    #[graphql(name = "timetableId")]
    pub timetable_id: Option<i32>,
    #[graphql(name = "courseId")]
    pub course_id: Option<i32>,
    pub credits: Option<i32>,
    #[graphql(name = "type")]
    pub subject_type: Option<String>,
    pub code: Option<String>,
    #[graphql(name = "className")]
    pub class_name: Option<String>,
    pub goal: Option<GqlSubjectGoal>,
}