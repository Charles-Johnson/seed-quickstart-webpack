use seed::*;
use seed::prelude::*;
use crate::js_calls;

// Model

pub struct Model {
    pub clicks: i32,
    pub random_number: i32
}

impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            random_number: js_calls::get_random_number(0,100)
        }
    }
}


// Update

#[derive(Clone)]
pub enum Msg {
    Increment,
    NewRandomNumber
}

pub fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = js_calls::get_random_number(0,100),
    }
    Render.into()
}


// View

pub fn view(model: &Model) -> El<Msg> {
    div![ 
        class!["h-screen", "w-screen", "flex", "flex-wrap", "justify-center", "content-center"],
        button![ 
            class!["mb-8", "mr-8", "p-4", "rounded", "shadow-md", "bg-green-lighter", "hover:bg-green-light"],
            simple_ev(Ev::Click, Msg::Increment), 
            format!("Clicks: {}", model.clicks) 
        ],
        button![ 
            class!["mb-8", "p-4", "rounded", "shadow-md", "bg-blue-lighter", "hover:bg-blue-light"],
            simple_ev(Ev::Click, Msg::NewRandomNumber), 
            format!("Random number from Typescript: {}", model.random_number) 
        ]
    ]
}