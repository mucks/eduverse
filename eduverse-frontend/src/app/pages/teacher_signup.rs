use std::rc::Rc;

use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{
    app::contexts::wallet_context::WalletContext, solana::contract_client::ContractClient,
    types::CreateTeacherInstruction,
};

async fn create_teacher(
    wallet: Rc<WalletContext>,
    teacher: CreateTeacherInstruction,
) -> Result<()> {
    let client = ContractClient::local();

    let pk = wallet.pubkey().context("Wallet not connected")?;
    let mut tx = client.build_create_teacher_tx(teacher, &pk).await?;
    // phantom requires the blockhash to be set
    tx.message.recent_blockhash = client.inner.get_latest_blockhash().await?;

    wallet.sign_and_send_transaction(tx).await?;

    Ok(())
}

#[function_component(TeacherSignUp)]
pub fn teacher_signup() -> Html {
    let wallet = use_context::<Rc<WalletContext>>().expect("WalletContext not found");

    let teacher = use_state(|| CreateTeacherInstruction::default());
    let teacher_clone = teacher.clone();

    let onclick = move |_| {
        let w = wallet.clone();
        let t = (*teacher_clone).clone();
        wasm_bindgen_futures::spawn_local(async move {
            match create_teacher(w, t.clone()).await {
                Ok(_) => {
                    log::info!("Teacher created");
                }
                Err(e) => {
                    log::error!("Error creating teacher: {:?}", e);
                }
            }
        })
    };

    let teacher_clone = teacher.clone();
    let onchange = move |e: Event| {
        let mut t: CreateTeacherInstruction = (*teacher_clone).to_owned();

        // When events are created the target is undefined, it's only
        // when dispatched does the target get added.
        let target: Option<EventTarget> = e.target();
        // Events can bubble so this listener might catch events from child
        // elements which are not of type HtmlInputElement
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            match input.name().as_str() {
                "title" => t.title = input.value(),
                "website" => t.website = input.value(),
                "telegram" => t.telegram = input.value(),
                "twitter" => t.twitter = input.value(),
                _ => {}
            }

            teacher_clone.set(t);
        }
    };

    html! {
        <div class="container fluid">
            <div class="row justify-content-center text-center mt-5">
                <div class="col-6">
                    <h1> {"Teacher Sign Up"} </h1>
                    <input name="title" onchange={onchange.clone()} value={teacher.title.clone()}  type="text" class="form-control mt-4" placeholder="Title" aria-label="title" />
                    <input name="website" onchange={onchange.clone()} value={teacher.website.clone()}  type="text" class="form-control mt-4" placeholder="Website" aria-label="website" />
                    <input name="telegram" onchange={onchange.clone()} value={teacher.telegram.clone()}  type="text" class="form-control mt-4" placeholder="Twitter" aria-label="twitter" />
                    <input name="twitter"  onchange={onchange.clone()} value={teacher.twitter.clone()} type="text" class="form-control mt-4" placeholder="Telegram" aria-label="telegram" />
                    <button onclick={onclick} class="btn btn-outline-light btn-lg mt-4"> {"Sign Up"} </button>
                </div>
            </div>
        </div>
    }
}
