use yew::prelude::*;

use crate::components::Modal;

/// A single example entry for display in the picker.
#[derive(Clone, Debug, PartialEq)]
pub struct ExampleItem {
    pub name: String,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct ExamplePickerProps {
    pub id: String,
    pub title: String,
    pub examples: Vec<ExampleItem>,
    pub active: bool,
    pub on_close: Callback<()>,
    pub on_select: Callback<usize>,
}

#[function_component(ExamplePicker)]
pub fn example_picker(props: &ExamplePickerProps) -> Html {
    html! {
        <Modal id={props.id.clone()} title={props.title.clone()} active={props.active} on_close={props.on_close.clone()}>
            <div class="examples-list">
                {for props.examples.iter().enumerate().map(|(idx, item)| {
                    let on_select = props.on_select.clone();
                    let onclick = Callback::from(move |_: MouseEvent| {
                        on_select.emit(idx);
                    });
                    html! {
                        <div class="example-item" key={idx} onclick={onclick}>
                            <h4>{&item.name}</h4>
                            <p>{&item.description}</p>
                        </div>
                    }
                })}
            </div>
        </Modal>
    }
}
