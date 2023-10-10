use yew::prelude::*;

use super::{
    CommentInfo, CommentProps, CommentsSection, VideoInfo, VideoListProps, VideoListSection,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}

#[function_component(ContentSection)]
pub fn content_section() -> Html {
    let video_list_props = yew::props!(VideoListProps {
        videos: vec![
            VideoInfo {
                name: "Variables".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Constructors".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Functions".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Threads".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Functions".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Variables".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Constructors".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Functions".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Threads".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
            VideoInfo {
                name: "Functions".to_string(),
                link: "".to_string(),
                description: "".to_string(),
                thumbnail: "".to_string()
            },
        ]
    });

    let comment_props = yew::props!(CommentProps {
        comments: vec![
            CommentInfo {
                user_name: "Evandro".to_string(),
                thumbnail: "img.png".to_string(),
                comment: "Wow, wonderful content".to_string(),
                answers: vec![]
            },
            CommentInfo {
                user_name: "Helo".to_string(),
                thumbnail: "helo.png".to_string(),
                comment: "Very good".to_string(),
                answers: vec![]
            },
            CommentInfo {
                user_name: "Helo".to_string(),
                thumbnail: "helo.png".to_string(),
                comment: "Very good".to_string(),
                answers: vec![]
            },
            CommentInfo {
                user_name: "Helo".to_string(),
                thumbnail: "helo.png".to_string(),
                comment: "Very good".to_string(),
                answers: vec![]
            },
            CommentInfo {
                user_name: "Helo".to_string(),
                thumbnail: "helo.png".to_string(),
                comment: "Very good".to_string(),
                answers: vec![]
            },
        ]
    });

    html! {
        <main role="main" class="col-md-9 ml-sm-auto col-lg-10 px-md-4">
            <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
                <h1 class="h2">{ " Video Title " }</h1>
            </div>
            <div class="row">
                <div class="col-md-8">
                    <video width="100%" controls=true>
                        <source src="path_to_video.mp4" type="video/mp4" />
                        { "Your browser does not support the video tag." }
                    </video>
                </div>
                <VideoListSection ..video_list_props />
            </div>
            <div class="row">
                <div class="mt-4">
                    <p>{ "Some text content here..." }</p>
                </div>
            </div>
            <CommentsSection ..comment_props />
        </main>
    }
}
