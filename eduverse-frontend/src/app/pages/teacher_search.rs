use yew::prelude::*;

#[function_component(TeacherSearch)]
pub fn teacher_search() -> Html {
    html! {
        <div class="container-fluid">
            <section>
                <div class="row justify-content-center mt-5">
                    <div class="col-md-6">
                        <h1> { "Choose your subject" } </h1>
                        <input type="text" class="form-control mt-4 ps-4" placeholder="Subject" aria-label="Subject" />
                        <div class="row mt-4">
                            <div class="col-md-3">
                                <button class="btn btn-outline-light rounded-corner">{"Lesson Time"}</button>
                            </div>
                            <div class="col-md-3">
                                <button class="btn btn-outline-light rounded-corner">{"Lesson Category"}</button>
                            </div>
                            <div class="col-md-3">
                                <button class="btn btn-outline-light rounded-corner">{"Teacher Location"}</button>
                            </div>
                            <div class="col-md-3">
                                <button class="btn btn-outline-light rounded-corner">{"More"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section>
                <div class="row justify-content-center mt-5">
                    <div class="col-md-6">
                        <h1> {"Top teachers"} </h1>
                        <div class="row mt-4">
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section>
                <div class="row justify-content-center" style="margin-top: 120px;">
                    <div class="col-md-6">
                        <h1> {"New talents"} </h1>
                        <div class="row mt-4">
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                            <div class="col-md-3">
                                <img class="rounded-circle" style="width: 100px;" src="https://placehold.co/150x150" />
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <section>
                <div class="row justify-content-center" style="margin-top: 120px;">
                    <div class="col-md-8">
                        <h1> {""} </h1>
                    </div>
                </div>
            </section>
        </div>
    }
}
