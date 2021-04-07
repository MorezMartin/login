#![recursion_limit="256"]

use rocket_crud::users::users::{ Users };

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use serde_json::json;
use serde::{ Serialize, Deserialize };
use yew::format::{ Json, Nothing };
use yew::services::fetch::{ Request, Response, FetchService, FetchTask };
use anyhow;

pub struct Model {
    link: ComponentLink<Self>,
    value: Login,
    status: String,
    fetch_task: Option<FetchTask>
}

pub enum Msg {
    GotEmail(String),
    GotPassword(String),
    Clicked,
    GotAnswer(Result<Users, anyhow::Error>)
}

pub struct Login {
    email: String,
    password: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            value: Login { email: "".into(), password: "".into() },
            status: "".into(),
            fetch_task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotAnswer(user) => {
                let user = match user {
                    Ok(Users::UserResp { nickname, name, surname }) => {
                        format!("Hello {}", nickname)
                    },
                    _ => "Login failed, try again or sign in".to_string()
                };
                self.status = user
            }
            Msg::GotEmail(new_value) => {
                self.value.email = new_value;
            }
            Msg::GotPassword(new_value) => {
                self.value.password = new_value;
            }
            Msg::Clicked => {
                let user = Users::UserLogin { email: self.value.email.clone(), password: self.value.password.clone() };
                let request = Request::post("http://localhost:8000/user")
                    .header("Content-Type", "application/json")
                    .body(Json(&user))
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<anyhow::Result<Users>>>| {
                            let Json(data) = response.into_body();
                            Msg::GotAnswer(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.value = Login { email: self.value.email.clone(), password: "".to_string() };
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <form>
                <div>
                    <label for="email">{ "Enter your email:" }</label>
                    <input type="email" id="email" required=true value=self.value.email
                        oninput=self.link.callback(|p: InputData| Msg::GotEmail(p.value))
                    />
                </div>
                <div>
                    <label for="password">{ "Enter your password:" }</label>
                    <input type="password" id="password" required=true value=self.value.password
                        oninput=self.link.callback(|p: InputData| Msg::GotPassword(p.value))
                    />
                </div>
                <div>
                    <input type="submit" value="Sign In" onclick=self.link.callback(|_| Msg::Clicked) />
                </div>
                <div>{self.value.email.clone()}</div>
                <div>{self.value.password.clone()}</div>
                <div>{self.status.clone()}</div>
            </form>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
