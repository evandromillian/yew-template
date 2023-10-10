use yew::prelude::*;
use yew_router::prelude::*;

use super::{About, Home, Portal, Table, UnitConverter, WebGLPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/table/:id")]
    Table { id: String },
    #[at("/gl")]
    GlRender,
    #[at("/convert")]
    Convert,
    #[at("/portal")]
    Portal,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <About /> },
        Route::Post { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
        Route::Table { id } => html! { <Table /> },
        Route::GlRender => html! { <WebGLPage /> },
        Route::Convert => html! { <UnitConverter /> },
        Route::Portal => html! { <Portal /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
