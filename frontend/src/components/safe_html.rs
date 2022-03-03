use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
    #[prop_or("dev".to_string())]
    pub wrapper: String,
    #[prop_or("".to_string())]
    pub style: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(Props {html, wrapper, style}: &Props) -> Html {
    let element = gloo_utils::document().create_element(wrapper).unwrap();
    element.set_attribute("style", &style);
    element.set_inner_html(&html.clone());

    Html::VRef(element.into())
}