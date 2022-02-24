use reqwasm::http::Request;
use yew::prelude::*;

use common::CourseDetails;

mod components;

#[function_component(App)]
fn app() -> Html {
    let courses = use_state(std::vec::Vec::new);
    let selected_course = use_state(|| None);

    let on_course_select = {
        let selected_course = selected_course.clone();
        Callback::from(move |course: CourseDetails| selected_course.set(Some(course)))
    };

    {
        let courses = courses.clone();
        let on_course_select = on_course_select.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_courses: Vec<CourseDetails> =
                        Request::get("https://localhost:1111/courses/")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    log::debug!("fetched: {fetched_courses:?}");
                    fetched_courses.sort_by(|a, b| a.name().cmp(b.name()));
                    if !fetched_courses.is_empty() {
                        on_course_select.emit(fetched_courses.get(0).unwrap().clone());
                    }
                    courses.set(fetched_courses);
                });
                || ()
            },
            (),
        );
    }
    let update_courses = {
        let courses = courses.clone();
        log::debug!("update_courses");
        Callback::from(move |course_details| courses.set(course_details))
    };

    let details = selected_course.as_ref().map(|course_details| {
        html! {
            <components::course_details::CourseDetailsDisplay course_details={course_details.clone()} />
        }
    });

    html! {
        <div class={"wrapper"}>
            <div style={"flex: 1 100%"}>
                <h1>{ "Course Planner" }</h1>
            </div>
            <div style={"background: tomato"}>
                <h2>{"Known Courses"}</h2>
                <components::course_name_editor::CourseNameEditor on_change={update_courses} on_select={on_course_select.clone()} />
                <div class={"courses"}>
                    <components::course_list::CoursesList course_details={(*courses).clone()} on_click={on_course_select.clone()} />
                </div>
            </div>
            <div style={"background: lightgreen; flex: 2 0px"}>
                { for details }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
