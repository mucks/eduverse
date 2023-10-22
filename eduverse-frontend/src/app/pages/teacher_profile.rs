use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use num_traits::{FromPrimitive, ToPrimitive};

use crate::{
    app::contexts::wallet_context::WalletContext,
    solana::contract_client::ContractClient,
    types::{Subject, YewTeacher},
};
use anyhow::{Context, Result};

async fn register_subject(wallet: Rc<WalletContext>, subject: Subject) -> Result<()> {
    let client = ContractClient::local();
    let pk = wallet.pubkey().context("Wallet not connected")?;
    let mut tx = client.register_subject(&pk, subject as u32).await?;
    tx.message.recent_blockhash = client.inner.get_latest_blockhash().await?;

    wallet.sign_and_send_transaction(tx).await?;
    Ok(())
}

async fn get_teacher(wallet: Rc<WalletContext>) -> Result<YewTeacher> {
    let client = ContractClient::local();
    let pk = wallet.pubkey().context("Wallet not connected")?;
    let teacher = client.get_teacher_by_pubkey(&pk).await?;
    Ok(teacher.into())
}

// This is a teachers private profile page, they can only access it when logged in

#[function_component(TeacherProfile)]
pub fn teacher_profile() -> Html {
    let teacher = use_state(|| YewTeacher::default());
    let subject = use_state(|| Subject::Math);
    let wallet = use_context::<Rc<WalletContext>>().expect("no wallet context found");

    let wallet_clone = wallet.clone();
    let teacher_clone = teacher.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            match get_teacher(wallet_clone.clone()).await {
                Ok(t) => {
                    log::info!("Teacher: {:?}", t);
                    teacher_clone.set(t);
                }
                Err(e) => {
                    log::error!("Error getting teacher: {:?}", e);
                }
            }
        });
    });

    let subject_clone = subject.clone();
    let add_subject = move |_| {
        let subject_clone = subject_clone.clone();
        let wallet = wallet.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = register_subject(wallet, *subject_clone).await {
                log::error!("add_subject: {}", e);
            }
        });
    };
    let subject_clone = subject.clone();
    let on_change = move |e: Event| {
        let el = e
            .target()
            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            .expect("could not get target");

        let val = el.value();
        let s = Subject::from_u32(val.parse::<u32>().expect("could not parse value"))
            .expect("could not convert to subject");
        subject_clone.set(s);
    };

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
                <div class="row">
                    <div class="col-md-8">
                        <div class="row">
                            <div class="col-md-3 mt-1">
                                <h4> {"Subjects:"} </h4>
                            </div>
                            {teacher.subjects.iter().map(|s| {
                                html! {
                                    <div class="col-md-2">
                                        <h2><span class="badge bg-secondary">{s.to_string()}</span></h2>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                        <div class="mt-2">
                            <select onchange={on_change} class="form-select" aria-label="Default select example">
                                {enum_iterator::all::<Subject>().map(|s| {
                                    html! {
                                        <option selected={*subject == s} value={s.to_u32().expect("panic: subject has an invalid number").to_string()}>{s.to_string()}</option>
                                    }
                                }).collect::<Html>()}
                            </select>
                            <button class="mt-2 btn btn-primary" onclick={add_subject}> {"Add Subject"} </button>
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
