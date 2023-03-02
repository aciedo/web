use crate::api::fetch;
use interface::v1::profile::{GetProfileByIdReq, GetProfileByIdRes};
use leptos::*;
use leptos_router::*;

#[component]
pub fn User(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let user = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or("87a7add4-0988-4d81-ba8e-621af24f6c29".to_string()),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                fetch::<GetProfileByIdRes, GetProfileByIdReq, 24>(cx, "https://localhost:8080/v1/profile/id", GetProfileByIdReq {
                    id: id.parse().unwrap(),
                }).await.map(|res| res.profile).flatten()
            }
        },
    );
    view! { cx,
        <div class="user-view">
            <Suspense fallback=|| view! { cx, "Loading..." }>
                {move || user.read(cx).map(|user| match user {
                    None => view! { cx,  <h1>"User not found."</h1> }.into_any(),
                    Some(user) => view! { cx,
                        <div>
                            <h1>"User: " {&user.name}" (id "{&user.id.to_string()}")"</h1>
                            // <ul class="meta">
                            //     <li>
                            //         <span class="label">"Created: "</span> {user.created}
                            //     </li>
                            //     <li>
                            //     <span class="label">"Karma: "</span> {user.karma}
                            //     </li>
                            //     {user.about.as_ref().map(|about| view! { cx,  <li inner_html=about class="about"></li> })}
                            // </ul>
                            <p class="links">
                                <a href=format!("/{}", user.id)>"submissions"</a>
                                " | "
                                <a href=format!("https://news.ycombinator.com/threads?id={}", user.id)>"comments"</a>
                            </p>
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}
