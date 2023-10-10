use yew::prelude::*;

use super::{ContentSection, SidenavSection, TopSection};

#[function_component(Portal)]
pub fn portal() -> Html {
    html! {
        <div>
            < TopSection />
            <div class="container-fluid mt-3">
                <div class="row">
                    <SidenavSection />
                    <ContentSection />
                </div>
            </div>
        </div>
    }
}
