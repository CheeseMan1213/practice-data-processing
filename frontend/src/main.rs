use yew::prelude::*;

mod print_count_to_ten;

#[function_component(App)]
fn app() -> Html {
    html! {
        <p>{"Test"}</p>
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
    yew::Renderer::<App>::new().render();
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod test;
