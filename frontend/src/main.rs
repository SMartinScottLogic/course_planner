use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use common::{CourseDetails, Stage};

#[derive(Clone, Properties, PartialEq)]
struct AlarmProps {
    width: usize,
    height: usize,
}

#[function_component(Alarm)]
fn alarm(AlarmProps { width, height }: &AlarmProps) -> Html {
    html! {
        <svg style="color: cornflowerblue; padding-right: 0.5rem" xmlns={"http://www.w3.org/2000/svg"} width={ width.to_string() } height={height.to_string()} fill={"currentColor"} class={"bi bi-alarm"} viewBox={"0 0 16 16"}>
            <path d={"M8.5 5.5a.5.5 0 0 0-1 0v3.362l-1.429 2.38a.5.5 0 1 0 .858.515l1.5-2.5A.5.5 0 0 0 8.5 9V5.5z"}/>
            <path d={"M6.5 0a.5.5 0 0 0 0 1H7v1.07a7.001 7.001 0 0 0-3.273 12.474l-.602.602a.5.5 0 0 0 .707.708l.746-.746A6.97 6.97 0 0 0 8 16a6.97 6.97 0 0 0 3.422-.892l.746.746a.5.5 0 0 0 .707-.708l-.601-.602A7.001 7.001 0 0 0 9 2.07V1h.5a.5.5 0 0 0 0-1h-3zm1.038 3.018a6.093 6.093 0 0 1 .924 0 6 6 0 1 1-.924 0zM0 3.5c0 .753.333 1.429.86 1.887A8.035 8.035 0 0 1 4.387 1.86 2.5 2.5 0 0 0 0 3.5zM13.5 1c-.753 0-1.429.333-1.887.86a8.035 8.035 0 0 1 3.527 3.527A2.5 2.5 0 0 0 13.5 1z"}/>
        </svg>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct CourseDetailsProps {
    course_details: CourseDetails,
}

#[function_component(CourseDetailsDisplay)]
fn course_details(CourseDetailsProps { course_details }: &CourseDetailsProps) -> Html {
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
                <p class={"duration"}><Alarm width=32 height=32 />{ stage.duration() }</p>
            </div>
        }
    });
    html! {
        <div>
            <h2>{ course_details.name() }</h2>
            <div style="display: none">{ course_details.id() }</div>
            { for stages }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CourseListProps {
    course_details: Vec<CourseDetails>,
    on_click: Callback<CourseDetails>,
}

#[function_component(CoursesList)]
fn courses_list(
    CourseListProps {
        course_details,
        on_click,
    }: &CourseListProps,
) -> Html {
    let course_detail_classes = ["course"];
    let on_click = on_click.clone();
    course_details
        .iter()
        .map(|course_detail| {
            let on_course_select = {
                let on_click = on_click.clone();
                let course_detail = course_detail.clone();
                Callback::from(move |_| on_click.emit(course_detail.clone()))
            };
            html! {
                <p class={classes!(course_detail_classes.as_ref())} onclick={on_course_select}>{course_detail.name().to_string()}</p>
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct CourseNameEditorProps {
    on_change: Callback<Vec<CourseDetails>>,
}

#[function_component(CourseNameEditor)]
fn course_name_editor(CourseNameEditorProps { on_change }: &CourseNameEditorProps) -> Html {
    let course_name_ref = use_node_ref();

    let onclick = {
        let course_name_ref = course_name_ref.clone();
        let on_change = on_change.clone();
        move |_| {
            if let Some(input) = course_name_ref.cast::<HtmlInputElement>() {
                let on_change = on_change.clone();
                let name = input.value();
                let course_details = CourseDetails::new("", &name);
                log::info!("Update: {:?}", course_details);
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_courses: Vec<CourseDetails> =
                        Request::put("https://localhost:1111/course")
                            .body(serde_json::to_string(&course_details).unwrap())
                            .header("Content-Type", "application/json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    on_change.emit(fetched_courses);
                });
            }
        }
    };
    let onkeyup = {
        let course_name_ref = course_name_ref.clone();
        let on_change = on_change.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                if let Some(input) = course_name_ref.cast::<HtmlInputElement>() {
                    let on_change = on_change.clone();
                    let name = input.value();
                    let course_details = CourseDetails::new("", &name);
                    log::info!("Update: {:?}", course_details);
                    wasm_bindgen_futures::spawn_local(async move {
                        let fetched_courses: Vec<CourseDetails> =
                            Request::put("https://localhost:1111/course")
                                .body(serde_json::to_string(&course_details).unwrap())
                                .header("Content-Type", "application/json")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                        on_change.emit(fetched_courses);
                    });
                }
            }
        }
    };
    html! {
        <div style="display: flex; flex-flow: row nowrap;">
            <input type="text" ref={course_name_ref} onkeyup={onkeyup}
                name="course_name_editor" data-test-selector="nav-search-input" placeholder="Course name …"
                autocapitalize="none" spellcheck="false" autocomplete="off"  style="flex: 4 0px; padding-right: 1em"/>
            <button onclick={onclick} style="flex: 0">{ "Ok" }</button>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let courses = use_state(std::vec::Vec::new);
    {
        let courses = courses.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_courses: Vec<CourseDetails> =
                        Request::get("https://localhost:1111/courses/")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    log::debug!("fetched: {fetched_courses:?}");
                    courses.set(fetched_courses);
                });
                || ()
            },
            (),
        );
    }
    let selected_course = use_state(|| None);

    let on_course_select = {
        let selected_course = selected_course.clone();
        Callback::from(move |course: CourseDetails| selected_course.set(Some(course)))
    };

    let update_courses = {
        let courses = courses.clone();
        log::debug!("update_courses");
        Callback::from(move |course_details| courses.set(course_details))
    };

    let details = selected_course.as_ref().map(|course_details| {
        html! {
            <CourseDetailsDisplay course_details={course_details.clone()} />
        }
    });

    html! {
        <div class={"wrapper"}>
            <div style={"flex: 1 100%"}>
                <h1>{ "Course Planner" }</h1>
            </div>
            <div style={"background: tomato"}>
                <h2>{"Known Courses"}</h2>
                <CourseNameEditor on_change={update_courses}/>
                <div class={"courses"}>
                    <CoursesList course_details={(*courses).clone()} on_click={on_course_select.clone()} />
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
