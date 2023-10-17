use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>
            <div class="container-fluid mt-5">
                <div class="pt-5 pb-5 bg-secondary text-white rounded text-center">
                    <h1>{ "Educational Platform" }</h1>
                    <h3>{ "Gate to Infinite Knowledge" }</h3>
                </div>
                <div class="pt-5 pb-5 bg-default text-white rounded text-center">
                    <h1>{ "About Us" }</h1>
                    <h5>{ "Welcome to our educational platform,\n where the pursuit of knowledge knows no bounds" }</h5>
                </div>
                <div class="pt-5 pb-5 bg-secondary text-white rounded text-center " style="vertical-align: middle;">
                    <h1>{ "Would you like to continue?" }</h1>
                    <div class="mt-4">
                        <Link<Route> to={Route::TeacherIntro} classes="btn btn-outline-light btn-lg ms-4 me-4"> { "Teacher" } </Link<Route>>
                        <Link<Route> to={Route::TeacherIntro} classes="btn btn-outline-light btn-lg ms-4 me-4"> { "Student" } </Link<Route>>
                    </div>
                </div>
            </div>
        </main>
    }
}
