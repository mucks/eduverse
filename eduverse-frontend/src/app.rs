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
        <BrowserRouter>
        <ContextProvider<Rc<WalletContext>> context={wallet}>
            <Navbar />
            <main>
                <Switch<Route> render={router::switch} />
            </main>
        </ContextProvider<Rc<WalletContext>>>
        </BrowserRouter>
    }
}
