use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <Link<Route> to={Route::About}>{ "Go to About" }</Link<Route>>
            <Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew v0.19 out now!" }</Link<Route>>
        </div>
    }
}
