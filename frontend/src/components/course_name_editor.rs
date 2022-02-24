use common::CourseDetails;
use reqwasm::http::Request;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{html, Properties, Callback, function_component, use_node_ref};

#[derive(Properties, PartialEq)]
pub struct CourseNameEditorProps {
    pub on_change: Callback<Vec<CourseDetails>>,
    pub on_select: Callback<CourseDetails>,
}

fn add_new_course(input: HtmlInputElement, on_change: &Callback<Vec<CourseDetails>>, on_select: &Callback<CourseDetails>) {
    let on_change = on_change.clone();
    let on_select = on_select.clone();
    let name = input.value();
    let course_details = CourseDetails::new("", &name);
    log::info!("Update: {:?}", course_details);
    wasm_bindgen_futures::spawn_local(async move {
        let mut fetched_courses: Vec<CourseDetails> =
            Request::put("https://localhost:1111/course")
                .body(serde_json::to_string(&course_details).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
        fetched_courses.sort_by(|a, b| a.name().cmp(b.name()));
        if !fetched_courses.is_empty() {
            on_select.emit(fetched_courses.get(0).unwrap().clone());
        }
        on_change.emit(fetched_courses);
    });
}

#[function_component(CourseNameEditor)]
pub fn course_name_editor(CourseNameEditorProps { on_change, on_select }: &CourseNameEditorProps) -> Html {
    let course_name_ref = use_node_ref();

    let onclick = {
        let course_name_ref = course_name_ref.clone();
        let on_change = on_change.clone();
        let on_select = on_select.clone();
        move |_| {
            if let Some(input) = course_name_ref.cast::<HtmlInputElement>() {
                add_new_course(input, &on_change, &on_select)
            }
        }
    };
    let onkeyup = {
        let course_name_ref = course_name_ref.clone();
        let on_change = on_change.clone();
        let on_select = on_select.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                if let Some(input) = course_name_ref.cast::<HtmlInputElement>() {
                    add_new_course(input, &on_change, &on_select)
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

