use yew::{html, Callback, Component, ComponentLink, Html};

pub struct DeleteData {
    link: ComponentLink<Self>,
    on_delete: Callback<()>,
    message: Option<String>, // Message received from the API after deletion
}

pub enum Msg {
    DeleteData,
    MessageReceived(String),
}

impl Component for DeleteData {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DeleteData {
            link,
            on_delete: link.callback(|_| Msg::DeleteData),
            message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DeleteData => {
                // Send request to API to delete data
                // Update self.message with the received message
                // You'll need to implement this logic using the fetch API
            }
            Msg::MessageReceived(message) => {
                self.message = Some(message);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.on_delete.clone()>
                    {"Delete Data"}
                </button>
                { self.render_message() }
            </div>
        }
    }
}

impl DeleteData {
    fn render_message(&self) -> Html {
        match &self.message {
            Some(message) => html! { <div>{ message }</div> },
            None => html! {},
        }
    }
}
