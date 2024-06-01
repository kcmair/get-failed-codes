use serde::Deserialize;
use yew::{Component, ComponentLink, html, Html, ShouldRender};
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct ReadData {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    data: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub data: String,
}

pub enum Msg {
    ReadData,
    DataReceived(Result<ApiResponse, anyhow::Error>),
}

impl Component for ReadData {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            data: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReadData => {
                let request = Request::get("http://localhost:8080/read")
                    .body(Ok(()))
                    .expect("Failed to build request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<ApiResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::DataReceived(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
            }
            Msg::DataReceived(response) => {
                match response {
                    Ok(data) => {
                        self.data = Some(data.data);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string());
                    }
                }
                self.fetch_task = None;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::ReadData)>
                    {"Read Data"}
                </button>
                { self.render_data() }
            </div>
        }
    }
}

impl ReadData {
    fn render_data(&self) -> Html {
        if let Some(ref data) = self.data {
            html! { <div>{ data }</div> }
        } else if let Some(ref error) = self.error {
            html! { <div>{ error }</div> }
        } else {
            html! {}
        }
    }
}
