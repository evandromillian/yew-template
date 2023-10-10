use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(Table)]
pub fn table() -> Html {
    html! {
        <div>
            <h1>{ "Table" }</h1>
            <Link<Route> to={Route::Home}>{ "Go to Home" }</Link<Route>>
        </div>
    }
}
