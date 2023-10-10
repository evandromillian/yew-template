use yew::prelude::*;

#[function_component(SidenavSection)]
pub fn sidenav_section() -> Html {
    html! {
        <nav class="col-md-2 d-none d-md-block bg-light sidebar">
            <div class="position-sticky">
                <ul class="nav flex-column">
                    <li class="nav-item">
                        <a class="nav-link active" href="#">
                            <i class="fas fa-home"></i>
                            { " Home " }
                        </a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">
                            <i class="fas fa-folder"></i>
                            { " Section 1 " }
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
