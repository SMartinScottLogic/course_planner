use common::{CourseDetails, Stage};
use reqwasm::http::Request;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct CourseDetailsProps {
    pub course_details: CourseDetails,
}

#[function_component(CourseDetailsDisplay)]
pub fn course_details(CourseDetailsProps { course_details }: &CourseDetailsProps) -> Html {
    let stage_classes = ["stage"];

    log::debug!("course_details {course_details:?}");
    let course = use_state(std::vec::Vec::new);
    {
        let course = course.clone();
        let id = course_details.id().to_owned();
        use_effect_with_deps(
            move |_| {
                let course = course.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_course: Vec<Stage> =
                        Request::get(&format!("https://localhost:1111/course/{id}"))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    log::debug!("fetched course: {fetched_course:?}");
                    course.set(fetched_course);
                });
                || ()
            },
            course_details.clone(),
        );
    }

    let stages = course.iter().map(|stage| {
        html! {
            <div class={classes!(stage_classes.as_ref())}>
                <p class={"name"} style={"padding-left: 2.5rem;"}>{ stage.name() }</p>
                <p class={"duration"}><crate::components::icon::Alarm width=32 height=32 />{ stage.duration() }</p>
            </div>
        }
    });
    let onclick = {
        move |_| {
            log::debug!("click");
        }
    };
    html! {
        <div>
            <h2>{ course_details.name() }</h2>
            <h3><span style="cursor: pointer;" onclick={onclick}><crate::components::icon::Plus width=32 height=32 /></span><span style={"padding-left: 2.5rem; vertical-align: 8px;"}>{ "Stages" }</span></h3>
            <div style="display: none">{ course_details.id() }</div>
            { for stages }
        </div>
    }
}
