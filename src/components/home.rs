use yew::prelude::*;
use web_sys::{Element, Node};
use gloo_utils::document;
use gloo_timers::callback::Timeout;
use gloo_console::log;
use std::time::Duration;

pub struct Hamburger;

pub struct User;

pub struct ProjectInfo;

pub struct App;

impl Component for App {

    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{
        let onclick = |_| {
            if document().get_element_by_id("userdashc").is_some() {
                let c = document().get_element_by_id("userdashc").unwrap();
                c.set_id("userdasho");
            }else if document().get_element_by_id("userdasho").is_some() {
                let c = document().get_element_by_id("userdasho").unwrap();
                c.set_id("userdashc");
            }
        };
        // let timeout = Timeout::new(1_000, move || {
        //     log!("tiding ...");
        // });timeout.forget();
        // log!("rendering");
        html! {
            <>
            <div class="top-bar-outer">
                <Hamburger/>
                <User/>
                <div class="top-bar">
                    <div class="user" {onclick} ></div>
                    <div class="empty"></div>
                </div>
                <FlapNav/>
                <p id="get-below" class="box">{"Welcome to DC Network , the most new and amazing platform for decentralized chating. this website is still under construction."}</p>
                <div id="but-start" align="center">
                    <button>{" Start Exploring "}</button>
                </div>
                <div id="decchat" align="center"><img src="img/decchat.png"/></div>
            </div>
            <p>{"DC Chat is based on the multi server - client architecture, which looks something like this."}</p>
            <div id="but-start" align="center">
                <a href="/chat"><button>{" Start Chatting "}</button></a>
            </div>
            <ProjectInfo/>
            </>
        }
    }
}

impl Component for Hamburger {

    type Message = ();
    type Properties = ();

    fn create(_ctx:&Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{
        let onclick = |_| {
            if document().get_element_by_id("hambure").is_some() {
                let c = document().get_element_by_id("hambure").unwrap();
                let cx = document().get_element_by_id("flapi").unwrap();
                cx.set_id("flapo");
                c.set_id("hamburx");

            }else if document().get_element_by_id("hamburx").is_some() {
                let c = document().get_element_by_id("hamburx").unwrap();
                let cx = document().get_element_by_id("flapo").unwrap();
                cx.set_id("flapi");
                c.set_id("hambure");
            }
        };

        html! {
            <div id="hambure" {onclick} class="hamburger">
                <div id="ham-1"></div>
                <div id="ham-2"></div>
                <div id="ham-3"></div>
            </div>
        }
    }
}

impl Component for User {

    type Message = ();
    type Properties = ();

    fn create(_ctx:&Context<Self>)-> Self {
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{

        html! {
            <div id="userdashc" class="user-dash">
                <div class="empty"></div>

                <div class="user-info"></div>

                <div class="user-comp">
                    <button>{"Generate Address"}</button>
                </div>

                <div class="user-comp">
                    <input placeholder="your address"/>
                </div>

                <div class="user-comp">
                    <input type="file" />
                </div>
                <div class="user-comp">
                    <input placeholder="attach private key"/>
                </div>

                <div class="user-comp">
                    <input type="file" />
                </div>

                <div class="user-comp">
                    <input placeholder="attach public key"/>
                </div>

                <div class="user-comp">
                    <button>{"New Account"}</button>
                </div>
            </div>
        }
    }
}
impl Component for ProjectInfo {

    type Message = ();
    type Properties = ();

    fn create(_ctx:&Context<Self>)-> Self{
        Self
    }

    fn view(&self,_ctx:&Context<Self>)-> Html{

        html! {
            <div id="p-info">
                <p class="p-data">{"Contribution To This Project is Highly Appreciated"}</p>
                <img src="img/github.dark.min.svg" style="height:50px;width:50px;" class="p-data"/>
            </div>
        }
    }
}

#[function_component(FlapNav)]
pub fn flapnav() -> Html {
    html! {
        <div id="flapi" class="flapnavo">
            <div class="flap-comp">
                <h2>{"Network Status"}</h2>
            </div>
            <hr/>
            <div class="flap-comp">
                <h3>{"Number of Servers"}</h3>
            </div>
            <div class="flap-comp">
                <h3>{"0"}</h3>
            </div>
            <div class="flap-comp">
                <h3>{"Connections Per Minute"}</h3>
            </div>
            <div class="flap-comp">
                <h3>{"0"}</h3>
            </div>

        </div>
    }
}
