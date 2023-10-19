use std::rc::Rc;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::app::{
    components::navbar::Navbar, contexts::wallet_context::WalletContext, pages::home::Home,
    router::Route,
};

mod components;
mod contexts;
mod pages;
mod router;

#[function_component(App)]
pub fn app() -> Html {
    let wallet = use_memo((), |_| WalletContext::new());

    html! {
        <ContextProvider<Rc<WalletContext>> context={wallet}>
            <BrowserRouter>
                <Navbar />
                <main>
                    <Switch<Route> render={router::switch} />
                </main>
            </BrowserRouter>
        </ContextProvider<Rc<WalletContext>>>
    }
}
