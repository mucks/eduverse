use crate::app::Home;
use enum_iterator::Sequence;
use yew::prelude::*;
use yew_router::Routable;

use super::{
    components::date_picker::DatePicker,
    pages::{
        teacher_intro::TeacherIntro, teacher_profile::TeacherProfile,
        teacher_public_profile::TeacherPublicProfile, teacher_search::TeacherSearch,
        teacher_signup::TeacherSignUp, test_all::TestAll,
    },
};

#[derive(Clone, Debug, Routable, PartialEq, Sequence)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/teacher-intro")]
    TeacherIntro,
    #[at("/teacher-sign-up")]
    TeacherSignUp,
    #[at("/teacher-profile")]
    TeacherProfile,
    #[at("/teacher-public-profile")]
    TeacherPublicProfile,
    #[at("/teacher-search")]
    TeacherSearch,

    #[at("/component/datepicker")]
    ComponentDatePicker,
    #[at("/test/all")]
    TestAll,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::ComponentDatePicker => html! { <DatePicker />},
        Route::TeacherIntro => html! { <TeacherIntro />},
        Route::TeacherSignUp => html! { <TeacherSignUp />},
        Route::TeacherSearch => html! { <TeacherSearch /> },
        Route::TeacherProfile => html! { <TeacherProfile /> },
        Route::TeacherPublicProfile => html! { <TeacherPublicProfile /> },
        Route::TestAll => html! { <TestAll /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
