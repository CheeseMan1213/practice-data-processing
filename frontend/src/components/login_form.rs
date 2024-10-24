use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;
use crate::contexts::CurrentUserDispatchActions;
use crate::Route;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::user::api_login;
use crate::api::user::api_me;
use crate::api::user::LoginResponse;
use crate::api::user::MeResponse;

async fn login(
  username: String,
  password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
  let login_repsonse = api_login(username, password).await?;
  let me_response = api_me(&login_repsonse.token).await?;
  Ok((login_repsonse, me_response))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
  let navigator = use_navigator();
  let current_user_ctx =
    use_context::<CurrentUserContext>().expect("Current user context is missing.");

  let username_handle = use_state(String::default);
  let username = (*username_handle).clone();
  let password_handle = use_state(String::default);
  let password = (*password_handle).clone();
  let errer_message_handle = use_state(String::default);
  let error_message = (*errer_message_handle).clone();

  let username_changed = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      username_handle.set(input.value());
    }
  });

  let password_changed = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      password_handle.set(input.value());
    }
  });

  let cloned_username = username.clone();
  let cloned_password = password.clone();
  let onsubmit = Callback::from(move |e: SubmitEvent| {
    e.prevent_default();

    let cloned_username = cloned_username.clone();
    let cloned_password = cloned_password.clone();
    let cloned_error_handle = errer_message_handle.clone();
    let cloned_navigator = navigator.clone();
    let cloned_user_ctx = current_user_ctx.clone();
    spawn_local(async move {
      match login(cloned_username.clone(), cloned_password.clone()).await {
        Ok(responses) => {
          cloned_user_ctx.dispatch(CurrentUserDispatchActions {
            action_type: crate::contexts::CurrentUserActions::LoginSuccess,
            login_response: Some(responses.0),
            me_response: Some(responses.1),
          });
          if let Some(nav) = cloned_navigator {
            nav.push(&Route::Home);
          }
        }
        Err(err) => cloned_error_handle.set(err.to_string()),
      }
    });
  });

  html! {
    <form onsubmit={onsubmit}>
    if !error_message.is_empty(){
      <Alert alert_type={"danger"} message={error_message} />
    }
      <div class="mb-3">
        <Input
          input_type="text"
          name="username"
          lable="Username"
          value={username}
          onchange={username_changed}
          />
      </div>
      <div class="mb-3">
        <Input
          input_type="password"
          name="password"
          lable="Password"
          value={password}
          onchange={password_changed}
          />
      </div>
      <button type="submit" class="btn btn-primary">{"Login"}</button>
    </form>
  }
}
