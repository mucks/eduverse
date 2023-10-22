use yew::prelude::*;

use crate::types::YewTeacher;

#[derive(Properties, PartialEq)]
pub struct TeacherCardProps {
    pub teacher: YewTeacher,
}

#[function_component(TeacherCard)]
pub fn teacher_profile(props: &TeacherCardProps) -> Html {
    html! {
        <div class="card">
            <div class="card-body">
                <div class="col-md-4">
                    <h5 class="card-title">
                    <p>{props.teacher.title.clone()}</p>
                    </h5>
                </div>
                <div class="col-md-8">
                    <p>{format!("Website: {}", props.teacher.website)}</p>
                    <p>{format!("Telegram: {}", props.teacher.telegram)}</p>
                    <p>{format!("Twitter: {}", props.teacher.twitter)}</p>
                </div>
            </div>
        </div>
    }
}
