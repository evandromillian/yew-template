use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VideoInfo {
    pub name: String,
    pub link: String,
    pub description: String,
    pub thumbnail: String,
}

#[derive(Properties, PartialEq)]
pub struct VideoListProps {
    #[prop_or_default]
    pub videos: Vec<VideoInfo>, // the field name `children` is important!
}

#[function_component(VideoListSection)]
pub fn video_list_section(props: &VideoListProps) -> Html {
    html! {
        <div>
            <div class="col-md-4">
                <div class="videos scrollable">
                    <ul class="list-unstyled">
                    {
                        props.videos.clone().into_iter().map(|video| {
                            html! {
                                <li>
                                    <div class="mt-4 mb-4">
                                        <a href="#">{ &video.name }</a>
                                    </div>
                                </li>
                            }
                        }).collect::<Html>()
                    }
                    </ul>
                </div>
            </div>
        </div>
    }
}
