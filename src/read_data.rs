use yew::{html, Callback, ChangeData, Component, ComponentLink, Html};

pub struct ReadData {
    link: ComponentLink<Self>,
    on_read: Callback<()>,
    data: Option<String>, // Data received from the API
}

pub enum Msg {
    ReadData,
    DataReceived(String),
}

impl Component for ReadData {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ReadData {
            link,
            on_read: link.callback(|_| Msg::ReadData),
            data: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ReadData => {
                // Send request to API to get data
                // Update self.data with the received data
                // You'll need to implement this logic using the fetch API
            }
            Msg::DataReceived(ref data) => {
                self.data = Some(data.clone()); // Clone the data to avoid moving it
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
                <button onclick=self.on_read.clone()>
                    {"Read Data"}
                </button>
                { self.render_data() }
            </div>
        }
    }
}

impl ReadData {
    fn render_data(&self) -> Html {
        match &self.data {
            Some(data) => html! { <div>{ data }</div> },
            None => html! {},
        }
    }
}
