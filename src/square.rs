use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SquareProps {
    pub color: String,
    pub piece: Option<String>,
}

#[function_component(Square)]
pub fn square(props: &SquareProps) -> Html {
    let click_handler = Callback::from(|_| {
        let object = JsValue::from("hello world");
        log!("Hello", object);
    });
    let bg_color = if props.color == "light" {
        "#f2e1c3"
    } else {
        "#c3a082"
    };

    let image_element = if props.piece.is_some() {
        html! { <img src={props.piece.clone().unwrap()} alt="Piece" /> }
    } else {
        html! {}
    };

    html! {
        <div
            class="square"
            style={format!("background-color: {};", bg_color)}
            onclick={click_handler}
        >
        {image_element}
        </div>
    }
}
