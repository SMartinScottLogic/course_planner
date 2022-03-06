use common::{Stage, CourseDetails};
use reqwasm::http::Request;
use yew::{function_component, Properties, html, Callback, use_node_ref, use_effect_with_deps};
use web_sys::{HtmlInputElement, KeyboardEvent};

use crate::SERVER;

#[derive(Clone, Properties, PartialEq)]
pub struct StageEditorProps {
    pub on_change: Callback<Stage>,
}

fn add_new_stage(
    course: CourseDetails,
    input: HtmlInputElement,
    duration: HtmlInputElement,
    on_change: &Callback<Vec<Stage>>,
) {
    let on_change = on_change.clone();
    let name = input.value();
    let duration = duration.value();
    let stage = Stage::new(&name, &duration);
    let id = course.id().to_owned();
    log::info!("Add stage: {:?}", stage);

    wasm_bindgen_futures::spawn_local(async move {
        let stages: Vec<Stage> = Request::post(&format!("{SERVER}/course/{id}"))
            .body(serde_json::to_string(&stage).unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        on_change.emit(stages);
    });
}


#[function_component(StageEditor)]
pub fn stage_editor(StageEditorProps { on_change }: &StageEditorProps) -> Html {
    let stage_name_ref = use_node_ref();
    let stage_duration_ref = use_node_ref();

    {
        let stage_name_ref = stage_name_ref.clone();
        use_effect_with_deps(
            move |a| {
                if let Some(input) = a.cast::<HtmlInputElement>() {
                    input.focus();
                };
                || ()
            },
            stage_name_ref
       );
    }
    let notify = {
        let on_change = on_change.clone();
        let stage_name_ref = stage_name_ref.clone();
        let stage_duration_ref = stage_duration_ref.clone();
        move || {
        if let Some(name) = stage_name_ref.cast::<HtmlInputElement>() {
            if let Some(duration) = stage_duration_ref.cast::<HtmlInputElement>() {
                let name = name.value();
                let duration = duration.value();
            
                on_change.emit(Stage::new(&name, &duration))
            }
        }
        }
    };
    let onclick = {
        /*
        let stage_name_ref = stage_name_ref.clone();
        let stage_duration_ref = stage_duration_ref.clone();
        let on_change = on_change.clone();
        let id = course.id().to_owned();
        */
        let notify = notify.clone();
        move |_| {
            notify();
        }
    };
    let onkeyup = {
        /*
        let stage_name_ref = stage_name_ref.clone();
        let stage_duration_ref = stage_duration_ref.clone();
        let on_change = on_change.clone();
        let id = course.clone();
        */
        let notify = notify.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                notify();
            }
        }
    };

    html! {
        <div style="display: flex; flex-flow: row nowrap;">
        <input type="text" ref={stage_name_ref} onkeyup={onkeyup.clone()}
            name="stage_name_editor" placeholder="Stage name …" style="flex: 4 0px; padding-right: 1em"/>
        <input type="text" ref={stage_duration_ref} onkeyup={onkeyup.clone()}
            name="stage_len_editor" placeholder="duration" style="flex: 1 0px; padding-right: 1em"/>
        <button onclick={onclick} style="flex: 0">{ "Ok" }</button>
    </div>

    }
}
