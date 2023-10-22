use yew::prelude::*;

use crate::{app::pages::teacher_card::TeacherCard, types::YewTeacher};

#[function_component(TeacherSearch)]
pub fn teacher_search() -> Html {
    let teachers = use_state(|| Vec::<YewTeacher>::new());
    let teachers_clone = teachers.clone();

    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = crate::solana::contract_client::ContractClient::local();
            match client.get_teachers().await {
                Ok(t) => {
                    log::info!("Teachers: {:?}", t);

                    let _teachers = t.into_iter().map(|t| t.into()).collect::<Vec<_>>();

                    teachers_clone.set(_teachers);
                }
                Err(e) => {
                    log::error!("Error getting teachers: {:?}", e);
                }
            }
        })
    });

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
                        {teachers.iter().map(|t| html! { <TeacherCard teacher={t.clone()}  /> }).collect::<Html>()}
                    </div>
                </div>
            </section>
        </div>
    }
}
