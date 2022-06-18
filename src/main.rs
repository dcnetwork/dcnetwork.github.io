#![recursion_limit = "256"]
use yew::prelude::*;

use web_sys::{Element, Node,WebSocket};
use gloo_utils::document;
use yew_router::prelude::*;
use decchatz::components::home::*;
use gloo_console::log;
use web_sys;
use js_sys::Function;
#[function_component(Sync)]
fn sync() -> Html {
    let socket = WebSocket::new("ws://223.184.174.109:4445").unwrap();
    socket.set_onopen(Some(&Function::new_no_args("{ alert('oppened') }")));
    socket.set_onmessage(Some(&Function::new_with_args("x","{ console.log(x) }")));
    socket.send_with_str("client here sir ");

    log!("tiding ... got it");
    html! {
        <div>

            <h1>{"# CHECKING NETWORK RECHABILITY ..."}</h1>
            <h1>{"Done"}</h1>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sync")]
    Sync
}
//
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <App/>
        },
        Route::Sync => html! {
            <Sync />
        }
    }
}
//
#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();

}
