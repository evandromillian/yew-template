use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{ "About" }</h1>
            <Link<Route> to={Route::Home}>{ "Go to Home" }</Link<Route>>
        </div>
    }
}
