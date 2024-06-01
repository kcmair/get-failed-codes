use yew::{html, Callback, Component, ComponentLink, Html, InputData};

pub struct DragAndDrop {
    link: ComponentLink<Self>,
    on_file_uploaded: Callback<InputData>,
}

pub enum Msg {
    FileUploaded(InputData),
}

impl Component for DragAndDrop {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DragAndDrop {
            link,
            on_file_uploaded: link.callback(Msg::FileUploaded),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FileUploaded(data) => {
                self.on_file_uploaded.emit(data);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div ondrop=self.link.callback(|e| Msg::FileUploaded(e.data_transfer().unwrap().files().get(0).unwrap()))
                 ondragover=self.link.callback(|e| e.prevent_default())>
                <p>{"Drag and drop a JSON file here"}</p>
            </div>
        }
    }
}
