use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;

#[function_component(Home)]
pub fn home() -> Html {
  let current_user_ctx =
    use_context::<CurrentUserContext>().expect("Current user context is missing.");

  match &current_user_ctx.user {
    Some(user) => {
      html! {
        <div class="container">
          <div class="row">
            <div class="col-md-3">
              <Sidebar />
            </div>
            <div class="col-md-9">
              <h1>{"Home"}</h1>
            </div>
          </div>
        </div>
      }
    }
    None => {
      html! {
          <Redirect<Route> to={Route::Login} />
      }
    }
  }
}
