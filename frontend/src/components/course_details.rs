use crate::components::safe_html::SafeHtml;
use common::{CourseDetails, Stage};
use reqwasm::http::Method;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Properties,
};

use crate::request;
use crate::SERVER;

#[derive(Clone, Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetails,
}

#[function_component(CourseDetailsDisplay)]
pub fn course_details(CourseDetailsProps { course_details }: &CourseDetailsProps) -> Html {
    let stage_classes = ["stage"];
    let new_stage_visible = use_state(|| false);

    log::debug!("course_details {course_details:?}");
    let id = course_details.id().to_owned();
    let course = use_state(std::vec::Vec::new);
    {
        let course = course.clone();
        let new_stage_visible = new_stage_visible.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                let course = course.clone();
                let new_stage_visible = new_stage_visible.clone();
                let id = id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_stages: Vec<Stage> =
                        request!(&format!("{SERVER}/course/{id}"), Method::GET);
                    log::debug!("fetched course: {fetched_stages:?}");
                    new_stage_visible.set(fetched_stages.is_empty());
                    course.set(fetched_stages);
                });
                || ()
            },
            course_details.clone(),
        );
    }

    let update_stages = {
        let course = course.clone();
        let new_stage_visible = new_stage_visible.clone();
        let id = id.clone();
        Callback::from(move |stage| {
            let id = id.clone();
            let new_stage_visible = new_stage_visible.clone();
            let course = course.clone();
            log::debug!("New stage for {id}: {stage}");
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_stages: Vec<Stage> = request!(
                    &format!("{SERVER}/course/{id}"),
                    Method::POST,
                    serde_json::to_string(&stage).unwrap()
                );
                log::debug!("fetched stage: {fetched_stages:?}");
                new_stage_visible.set(fetched_stages.is_empty());
                course.set(fetched_stages);
            });
        })
    };

    let stages = course.iter().enumerate().map(|(id, stage)| {
        html! {
            <div class={classes!(stage_classes.as_ref())} style={if id%2==0 {"background: #bbb;"}else{"background: #ccc;"}}>
                <p class={"name"} style={"padding-left: 2.5rem;"}>{ stage.name() }</p>
                <p class={"duration"}><crate::components::icon::Alarm width=32 height=32 />{ stage.duration() }</p>
            </div>
        }
    });

    let onclick = {
        let new_stage_visible = new_stage_visible.clone();
        move |_| {
            new_stage_visible.set(!*new_stage_visible);
        }
    };
    html! {
        <div>
            <h2>{ course_details.name() }</h2>
            <h3><span style="cursor: pointer;" onclick={onclick}><crate::components::icon::Plus width=32 height=32 /></span><span style={"padding-left: 2.5rem; vertical-align: 8px;"}>{ "Stages" }</span></h3>
            if *(new_stage_visible.clone()) {
                <div>{ "Add to course " }{ course_details.id() }</div>
                <crate::components::stage_editor::StageEditor on_change={update_stages.clone()} />

                <SafeHtml style="font-size: 3em;" wrapper="div" html="&#x1F418; &#x1F427; &#x1F43C; &#x2665; &#x2605; &#x2139; &#x1F480; &#x1F44C; &#x1F37D; &#x1F384; &#x23F2;" />
            }
            { for stages }
        </div>
    }
}
