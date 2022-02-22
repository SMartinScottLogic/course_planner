use reqwasm::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use common::CourseDetails;

#[derive(Clone, Properties, PartialEq)]
struct CourseDetailsProps {
    course_details: CourseDetails,
}

#[function_component(CourseDetailsDisplay)]
fn course_details(CourseDetailsProps { course_details }: &CourseDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ course_details.name() }</h3>
            <div>{ course_details.id() }</div>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
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
                <p onclick={on_course_select}>{course_detail.name().to_string()}</p>
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
                    //courses.set(fetched_courses);
                });
            }
        }
    };
    html! {
        <>
        <input type="text"
        ref={course_name_ref}
        name="course_name_editor"
        data-test-selector="nav-search-input"
        placeholder="Course name …"
        autocapitalize="none"
        spellcheck="false"
        autocomplete="off" />
        <button onclick={onclick}>{ "Ok" }</button>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    let courses = use_state(std::vec::Vec::new);
    {
        let courses = courses.clone();
        use_effect_with_deps(
            move |_| {
                let courses = courses.clone();
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
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <CourseNameEditor on_change={update_courses.clone()}/>
            <div>
                <h3>{"Known Courses"}</h3>
                <CoursesList course_details={(*courses).clone()} on_click={on_course_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
