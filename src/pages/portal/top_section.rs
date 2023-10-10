use yew::prelude::*;

#[function_component(TopSection)]
pub fn top_section() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <a class="navbar-brand" href="#">
                <img src="path_to_icon.png" width="30" height="30" class="d-inline-block align-top" alt="" />
                { " Company Name " }
            </a>
            <form class="form-inline ml-auto">
                <input class="form-control mr-sm-2" type="search" placeholder="Search" aria-label="Search" />
                <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{ " Search " }</button>
            </form>
            <div class="dropdown ml-3">
                <button class="btn btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                    { " Profile " }
                </button>
                <div class="dropdown-menu" aria-labelledby="dropdownMenuButton">
                    <a class="dropdown-item" href="#">{ " Profile " }</a>
                    <a class="dropdown-item" href="#">{ " Settings " }</a>
                    <a class="dropdown-item" href="#">{ " Logout " }</a>
                </div>
            </div>
        </nav>
    }
}
