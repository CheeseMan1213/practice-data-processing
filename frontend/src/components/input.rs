use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub lable: AttrValue,
    pub input_type: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
      <>
        <label for={html_id.clone()}>{props.lable.clone()}</label>
        <input
        class="form-control"
        id={html_id}
        type={props.input_type.clone()}
        name={props.name.clone()}
        value={props.value.clone()}
        onchange={props.onchange.clone()}
        />
      </>
    }
}
