use common::CourseDetails;
use yew::{classes, function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct CourseListProps {
    pub course_details: Vec<CourseDetails>,
    pub on_click: Callback<CourseDetails>,
}

#[function_component(CoursesList)]
pub fn courses_list(
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
