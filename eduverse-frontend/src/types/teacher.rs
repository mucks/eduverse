use borsh::BorshSerialize;
use eduverse_contract::state::Teacher;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use yew::Properties;

use super::Subject;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, Default, Clone)]
pub struct CreateTeacherInstruction {
    pub title: String,
    pub website: String,
    pub telegram: String,
    pub twitter: String,
}

#[derive(Debug, Serialize, Default, Clone, Properties, PartialEq)]
pub struct YewTeacher {
    pub title: String,
    pub website: String,
    pub telegram: String,
    pub twitter: String,
    pub subjects: Vec<Subject>,
}

impl From<Teacher> for YewTeacher {
    fn from(teacher: Teacher) -> Self {
        Self {
            title: teacher.title,
            website: teacher.website,
            telegram: teacher.telegram,
            twitter: teacher.twitter,
            subjects: teacher
                .subjects_registered
                .iter()
                .filter_map(|x| Subject::from_u32(*x))
                .collect(),
        }
    }
}
