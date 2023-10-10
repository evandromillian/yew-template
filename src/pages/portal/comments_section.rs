use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CommentInfo {
    pub user_name: String,
    pub thumbnail: String,
    pub comment: String,
    pub answers: Vec<CommentInfo>,
}

#[derive(Properties, PartialEq)]
pub struct CommentProps {
    #[prop_or_default]
    pub comments: Vec<CommentInfo>, // the field name `children` is important!
}

#[function_component(CommentsSection)]
pub fn comments_section(props: &CommentProps) -> Html {
    html! {
        <div class="row">
            <div class="mt-4 w-100">
                <h3>{ " Comments " }</h3>
                <div class="scrollable-container">
                {
                    props.comments.clone().into_iter().map(|c| {
                        html! {
                            <div class="media mb-3">
                                <img src={ c.thumbnail } class="mr-3" width="50" />
                                <div class="media-body">
                                    <h5 class="mt-0">{ c.user_name }</h5>
                                    { c.comment }
                                </div>
                            </div>
                            }
                    }).collect::<Html>()
                }
                </div>
            </div>
        </div>
    }
}
