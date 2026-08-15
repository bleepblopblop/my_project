use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "hey, it works" }</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
