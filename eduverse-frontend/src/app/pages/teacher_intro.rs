use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::router::Route;

#[function_component(TeacherIntro)]
pub fn teacher_intro() -> Html {
    html! {
        <div class="container-fluid">
            <section>
                <div class="row">
                    <div class="offset-md-1 col-md-5 text-center" style="margin-top: 120px">
                        <h4> {"Hello Teacher! Thanks for your interest"} </h4>
                    </div>
                    <div class="col-md-5 ms-2 mt-4">
                        <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/500x300" />
                    </div>
                </div>
            </section>
            <section class="container" style="margin-top: 75px">
                <div class="row text-center">
                    <h2> {"Why chose our Platform?"} </h2>
                    <div class="row mt-2">
                        <div class="col-4">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                        <div class="col-4">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                        <div class="col-4">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                    </div>
                </div>
            </section>
            <section class="container" style="margin-top: 75px">
                <div class="row text-center">
                    <h2> {"How does it work?"} </h2>
                    <div class="row mt-3">
                        <div class="col-3 ms-5">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                        <div class="col-1" style="margin-top: 40px">
                            <h1> {"→"} </h1>
                        </div>
                        <div class="col-3">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                        <div class="col-1" style="margin-top: 40px">
                            <h1> {"→"} </h1>
                        </div>
                        <div class="col-3">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/175x150" />
                        </div>
                    </div>
                </div>
            </section>
            <section class="text-center" style="margin-top: 50px">
                <Link<Route> to={Route::TeacherSignUp} classes="btn btn-outline-light btn-lg ms-4 me-4 ps-5 pe-5"> {"Sign up"} </Link<Route>>
            </section>
            <section style="margin-top: 75px">
            </section>

        </div>
    }
}
