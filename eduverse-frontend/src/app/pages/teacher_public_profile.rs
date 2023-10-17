use yew::prelude::*;

use crate::app::components::date_picker::DatePicker;

#[function_component(TeacherPublicProfile)]
pub fn teacher_profile() -> Html {
    html! {
        <div class="container">
            <section class="mt-5">
                <div class="row">
                    <div class="col-md-12">
                        <div class="row">
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 150px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-9">
                                <h1>{"Teacher's Name"}</h1>
                                <div class="mt-4">
                                    <h2>{"Qualifications: "}
                                    <span class="badge bg-secondary">{"BA degree"}</span>
                                    <span class="ms-2 badge bg-secondary">{"C2 Cambridge"}</span>
                                    </h2>
                                    <h2 class="mt-3">{"Courses: "}
                                    <span class="badge bg-secondary">{"General English"}</span>
                                    <span class="ms-2 badge bg-secondary">{"C2 Cambridge"}</span>
                                    </h2>
                                    <h2 class="mt-3">{"Price:"}</h2>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section style="margin-top: 75px;">
                <div class="bg-secondary rounded">
                    <div class="p-5">
                        <div>
                            <h2> {"Bio"} </h2>
                            <p>{"Lorem ipsum ..."}</p>
                        </div>
                        <div class="mt-5">
                            <h2> {"VideoLink"} </h2>
                        </div>
                    </div>
                </div>
            </section>
            <section style="margin-top: 75px;">
                <h2>{"Schedule"}</h2>
                <div class="row mt-4">
                    <div class="col-md-8">
                        <ul class="list-group">
                            <li class="list-group-item">{"10:00"}</li>
                            <li class="list-group-item">{"11:00"}</li>
                            <li class="list-group-item">{"12:00"}</li>
                            <li class="list-group-item">{"13:00"}</li>
                            <li class="list-group-item">{"14:00"}</li>
                            <li class="list-group-item">{"15:00"}</li>
                            <li class="list-group-item">{"16:00"}</li>
                            <li class="list-group-item">{"17:00"}</li>
                            <li class="list-group-item">{"18:00"}</li>
                        </ul>
                    </div>
                    <div class="col-md-4">
                        <DatePicker />
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-4">
                        <button class="btn btn-primary mt-4">{"Lesson Request"}</button>
                    </div>
                </div>
            </section>
            <section style="margin-top: 75px;">
                <h1> { "Reviews" } </h1>
                <div class="row mt-5">
                    <div class="bg-secondary col-md-5" style="height: 200px">
                    </div>
                    <div class="col-md-1">
                    </div>
                    <div class="bg-secondary col-md-5" style="height: 200px">
                    </div>
                </div>
                <div class="row mt-5">
                    <div class="bg-secondary col-md-5" style="height: 200px">
                    </div>
                    <div class="col-md-1">
                    </div>
                    <div class="bg-secondary col-md-5" style="height: 200px">
                    </div>
                </div>
            </section>
            <section style="margin-top: 75px;">
            </section>
        </div>
    }
}
