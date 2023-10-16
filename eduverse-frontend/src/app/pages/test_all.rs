use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::router::Route;

fn all_routes() -> Vec<Route> {
    enum_iterator::all::<Route>().collect()
}

#[function_component(TestAll)]
pub fn test_all() -> Html {
    let routes_view = all_routes()
        .into_iter()
        .map(|r| {
            let route_str = format!("{:?}", r);

            html! {
                <Link<Route> to={r} classes="list-group-item">{ route_str } </Link<Route>>
            }
        })
        .collect::<Html>();

    html! {
        <div>
            <h1> {"Test All Pages"} </h1>
            <div class="mt-5">
                <ul class="list-group">
                    {routes_view}
                </ul>
            </div>
        </div>
    }
}
