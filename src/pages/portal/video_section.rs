use yew::prelude::*;

use super::CommentsSection;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}

#[function_component(VideoSection)]
pub fn video_section() -> Html {
    html! {
        <div class="row">
            <div class="col-md-8">
                <video width="100%" controls=true>
                    <source src="path_to_video.mp4" type="video/mp4" />
                    { "Your browser does not support the video tag." }
                </video>
                <div class="mt-4">
                    <p>{ "Some text content here..." }</p>
                </div>
                <CommentsSection />
            </div>
        </div>
    }
}
