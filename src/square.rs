use chess::Square;
// use gloo_console::log;
// use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SquareCompProps {
    pub color: String,
    pub piece: Option<String>,
    pub can_move_to: bool,
    pub square: Square,
    pub set_selected: Callback<Option<Square>>,
    pub set_target: Callback<Option<Square>>,
}

#[function_component(SquareComp)]
pub fn square(props: &SquareCompProps) -> Html {
    let props_copy = props.clone();
    let click_handler = Callback::from(move |_| {
        if !props_copy.can_move_to && props_copy.piece.is_some() {
            props_copy.set_selected.emit(Some(props_copy.square));
        } else if props_copy.can_move_to {
            props_copy.set_target.emit(Some(props_copy.square));
        } else {
            props_copy.set_selected.emit(None);
        }
        // let object = JsValue::from("hello world");
        // log!("Hello", object);
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
    let move_circle = if props.can_move_to {
        html! { <div class="move-circle"></div> }
    } else {
        html! {}
    };

    html! {
        <div
            class="square"
            style={format!("background-color: {};", bg_color)}
            onclick={click_handler}
        >
        {move_circle}
        {image_element}
        </div>
    }
}
