use yew::prelude::*;

use crate::api::user::api_hello;
// use api::user::api_hello;

#[function_component(Hello)]
pub fn hello() -> Html {
  let message = use_state(|| "Loading...".to_string());

  {
    let message = message.clone();
    use_effect_with((), move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let msg = match api_hello().await {
          Ok(msg) => msg,
          Err(_) => "Error fetching message".to_string(),
        };
        message.set(msg);
      });
      || ()
    });
  }
  
  html! {
    <div class="container">
      <div class="row min-vh-100 justify-content-center align-items-center">
        <div class="col-md-4">
        <p>{ (*message).clone() }</p>
        </div>
      </div>
    </div>
  }
}
