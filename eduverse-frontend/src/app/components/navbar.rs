use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    app::{contexts::wallet_context::WalletContext, router::Route},
    util::DEBUG,
};

#[function_component(Navbar)]
pub fn app() -> Html {
    let wallet = use_context::<Rc<WalletContext>>().expect("could not get wallet context");

    let pk = wallet.pubkey();
    let trigger = use_force_update();

    let all_pages_link = match DEBUG {
        true => html! {
            <li class="nav-item">
                <Link<Route> to={Route::TestAll} classes="nav-link">{ "All" }</Link<Route>>
            </li>
        },
        false => html! {},
    };

    let mut connect_btn = html! {
        <button onclick={move |_| {
            let w = wallet.clone();
            let t = trigger.clone();
            wasm_bindgen_futures::spawn_local(async move {
                w.connect().await.expect("could not connect to wallet");
                t.force_update();
            });
        }} class="btn btn-primary">{"Connect"}</button>
    };

    if let Some(pk) = pk {
        let l = pk.to_string().len();
        let chars: Vec<char> = pk.to_string().chars().collect();
        let start: String = chars[..4].iter().collect();
        let end: String = chars[l - 4..l].iter().collect();
        let pk_short = format!("{start}...{end}");

        connect_btn = html! {
            <button disabled={true} class="btn btn-primary">{pk_short}</button>
        }
    }

    html! {
        <nav class="navbar bg-body-tertiary navbar-expand-md">
            <div class="container-fluid">
                <div class="d-flex">
                    <Link<Route> to={Route::Home} classes="navbar-brand">{ "Eduverse" } </Link<Route>>
                    <div class="input-group ms-2" style="width: 300px">
                        <input type="text" class="form-control" placeholder="Search" aria-label="Search" />
                        <span class="input-group-text" id="basic-addon2">
                            <i class="bi bi-search"></i>
                        </span>
                    </div>
                </div>
                <ul class="navbar-nav">
                    {all_pages_link}
                    <span class="p-2">
                        {"|"}
                    </span>
                    {connect_btn}
                </ul>
            </div>
        </nav>
    }
}
