use yew::prelude::*;
use web_sys::{DragEvent, FileReader, File};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use gloo::events::EventListener;

pub struct DragAndDrop {
    on_file_uploaded: Callback<String>,
}

pub enum Msg {
    FileUploaded(String),
    ReadFileContent(String),
}

impl Component for DragAndDrop {
    type Message = Msg;
    type Properties = ();

    fn create(_props: &Self::Properties, link: &ComponentLink<Self>) -> Self {
        Self {
            on_file_uploaded: link.callback(Msg::FileUploaded),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FileUploaded(content) => {
                self.on_file_uploaded.emit(content);
            }
            Msg::ReadFileContent(content) => {
                // Handle the read file content here (e.g., send it to the server)
                // This is where you can integrate with the POST endpoint
                log::info!("File content: {}", content);
            }
        }
        true
    }

    fn change(&mut self, _props: &Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_drop = self.link.callback(|e: DragEvent| {
            e.prevent_default();
            if let Some(files) = e.data_transfer().unwrap().files().get(0) {
                let file = files;
                let reader = FileReader::new().unwrap();
                let file_reader = reader.clone();
                let link = self.link.clone();
                let onloadend_cb = Closure::wrap(Box::new(move || {
                    if let Ok(result) = file_reader.result() {
                        if let Some(content) = result.as_string() {
                            link.send_message(Msg::ReadFileContent(content));
                        }
                    }
                }) as Box<dyn Fn()>);
                reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                reader.read_as_text(&file).unwrap();
                onloadend_cb.forget();
            }
            Msg::FileUploaded("".to_string())
        });

        let on_dragover = self.link.callback(|e: DragEvent| {
            e.prevent_default();
            Html::default()
        });

        html! {
        <div ondrop=on_drop ondragover=on_dragover>
            <p>{"Drag and drop a JSON file here"}</p>
        </div>
        }
    }
}
