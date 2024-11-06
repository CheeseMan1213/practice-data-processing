use api::user::api_hello;
use yew::prelude::*;
use yew_router::prelude::*;

mod print_count_to_ten;
mod api;
mod components;
mod pages;
mod contexts;

#[derive(Routable, PartialEq, Clone)]
enum Route {
  #[at("/")]
  Home,
  #[at("/rustaceans")]
  Rustaceans,
  #[at("/crates")]
  Crates,
  #[at("/login")]
  Login,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(route: Route) -> Html {
  match route {
    Route::Login => html! { <pages::login::Login /> },
    Route::NotFound => html! { <pages::not_found::NotFound /> },
    Route::Home => html! { <pages::home::Home /> },
    _ => html! { <pages::login::Login /> },
  }
}

#[function_component(App)]
fn app() -> Html {
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
    <p>{ (*message).clone() }</p>
    // <BrowserRouter>
    //   <contexts::CurrentUserProvider>
    //     <Switch<Route> render={switch}/>
    //   </contexts::CurrentUserProvider>
    // </BrowserRouter>
  }
}

fn main() {
    // println!("Hello, world from frontend.");
    // println!("{}", subtract(2, 3));
    // http://127.0.0.1:8082
    yew::Renderer::<App>::new().render();
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod test;
