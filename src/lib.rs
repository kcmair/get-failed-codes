use gloo_file::{callbacks::read_as_text, File, FileReadError};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

enum Msg {
    FileLoaded(String),
    FetchSuccess(String),
    FetchError(String),
    DeleteSuccess(String),
    DeleteError(String),
    ReadFromDB,
    DeleteFromDB,
    DragOver,
}

struct Model {
    fetch_result: Option<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            fetch_result: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileLoaded(content) => {
                self.upload_file(ctx, content);
                true
            }
            Msg::FetchSuccess(result) => {
                self.fetch_result = Some(result);
                true
            }
            Msg::FetchError(error) => {
                self.fetch_result = Some(format!("Error: {}", error));
                true
            }
            Msg::DeleteSuccess(result) => {
                self.fetch_result = Some(result);
                true
            }
            Msg::DeleteError(error) => {
                self.fetch_result = Some(format!("Error: {}", error));
                true
            }
            Msg::ReadFromDB => {
                self.read_from_db(ctx);
                true
            }
            Msg::DeleteFromDB => {
                self.delete_from_db(ctx);
                true
            }
            Msg::DragOver => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        html! {
            <div>
                <div
                    ondrop={ctx.link().callback(move |e: DragEvent| {
                        e.prevent_default();
                        let file_list = e.data_transfer().unwrap().files().unwrap();
                        let file = file_list.get(0).unwrap();
                        let file = File::from(file);
                        let _task = read_as_text(&file, {
                            let link = link.clone();
                            move |text: Result<String, FileReadError>| {
                                match text {
                                    Ok(s) => link.send_message(Msg::FileLoaded(s)),
                                    Err(_) => link.send_message(Msg::FetchError("Unable to read file".to_string())),
                                }
                            }
                        });
                        Msg::FetchSuccess("File dropped".into())
                    })}
                    ondragover={ctx.link().callback(|e: DragEvent| {
                        e.prevent_default();
                        Msg::DragOver
                    })}
                >
                    <p>{ "Drag and drop a JSON file here" }</p>
                </div>
                <button onclick={ctx.link().callback(|_| Msg::ReadFromDB)}>{ "Read from DB" }</button>
                <button onclick={ctx.link().callback(|_| Msg::DeleteFromDB)}>{ "Delete from DB" }</button>
                <div>
                    { self.view_result() }
                </div>
            </div>
        }
    }
}

impl Model {
    fn upload_file(&self, ctx: &Context<Self>, content: String) {
        let request = Request::post("http://localhost:8080/write")
            .header("Content-Type", "application/json")
            .body(content);

        let link = ctx.link().clone();
        spawn_local(async move {
            let msg = match request.send().await {
                Ok(response) => {
                    let body = response.text().await.expect("Failed to get response text");
                    Msg::FetchSuccess(body)
                },
                Err(_) => Msg::FetchError("Failed to upload file".into()),
            };
            link.send_message(msg);
        });
    }

    fn read_from_db(&self, ctx: &Context<Self>) {
        let request = Request::get("http://localhost:8080/read")
            .send();

        let link = ctx.link().clone();
        spawn_local(async move {
            let msg = match request.await {
                Ok(response) => {
                    let body = response.text().await.expect("Failed to get response text");
                    Msg::FetchSuccess(body)
                },
                Err(_) => Msg::FetchError("Failed to read from DB".into()),
            };
            link.send_message(msg);
        });
    }

    fn delete_from_db(&self, ctx: &Context<Self>) {
        let request = Request::delete("http://localhost:8080/delete")
            .send();

        let link = ctx.link().clone();
        spawn_local(async move {
            let msg = match request.await {
                Ok(response) => {
                    let body = response.text().await.expect("Failed to get response text");
                    Msg::DeleteSuccess(body)
                },
                Err(_) => Msg::DeleteError("Failed to delete from DB".into()),
            };
            link.send_message(msg);
        });
    }

    fn view_result(&self) -> Html {
        if let Some(ref result) = self.fetch_result {
            html! { <p>{ result }</p> }
        } else {
            html! {}
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Model />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
