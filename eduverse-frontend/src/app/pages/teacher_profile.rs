use yew::prelude::*;

#[function_component(TeacherProfile)]
pub fn teacher_profile() -> Html {
    html! {
        <div class="container">
            <section class="mt-5">
                <div class="row">
                    <div class="col-md-8">
                        <div class="row">
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 150px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-2"></div>
                            <div class="col-md-7">
                                <h3 style="margin-top: 50px"> { "Welcome, teacher"} </h3>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section class="mt-5">
                <h3> {"Upcoming Lessons:"} </h3>
                <div class="row">
                    <div class="col-md-6">
                    <div class="row mt-3">
                        <div class="col-4 text-center">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/150x150" />
                            <h5 class="mt-2"> {"Student 1"} </h5>
                        </div>
                        <div class="col-4 text-center">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/150x150" />
                            <h5 class="mt-2"> {"Student 2"} </h5>
                        </div>
                        <div class="col-4 text-center">
                            <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/150x150" />
                            <h5 class="mt-2"> {"Student 3"} </h5>
                        </div>
                    </div>
                    </div>
                </div>
            </section>
            <section class="mt-5">
                <div class="row">
                    <div class="col-md-5">
                        <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/600x400" />
                    </div>
                    <div class="col-md-2">
                    </div>
                    <div class="col-md-5">
                        <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/600x400" />
                    </div>
                </div>
                <div class="row mt-5">
                    <div class="col-md-5">
                        <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/600x400" />
                    </div>
                    <div class="col-md-2">
                    </div>
                    <div class="col-md-5">
                        <img class=".img-fluid" style="max-width: 100%; height: auto;" src="https://placehold.co/600x400" />
                    </div>
                </div>
            </section>
            <section class="mt-5">
            </section>
        </div>
    }
}
