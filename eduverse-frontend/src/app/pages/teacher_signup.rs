use yew::prelude::*;

#[function_component(TeacherSignUp)]
pub fn teacher_signup() -> Html {
    html! {
        <div class="container fluid">
            <div class="row justify-content-center text-center mt-5">
                <div class="col-6">
                <h1> {"Teacher Sign Up"} </h1>
                <input  type="text" class="form-control mt-4" placeholder="Subject" aria-label="Subject" />
                <input  type="text" class="form-control mt-4" placeholder="Experience" aria-label="Experience" />
                <button class="btn btn-outline-light btn-lg mt-4"> {"Sign Up"} </button>
                </div>
            </div>
        </div>
    }
}
