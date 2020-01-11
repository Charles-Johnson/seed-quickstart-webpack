use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};
use zia::Context;

pub struct Model {
    context: Context,
    input: String,
    output: String,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            context: Context::new(),
            input: String::new(),
            output: String::new(),
        }
    }
}

#[derive(Clone)]
pub enum HomeMsg {
    Input(String),
    Submit,
    Nothing,
}

pub fn update(msg: HomeMsg, model: &mut Model) {
    match msg {
        HomeMsg::Input(s) => model.input = s,
        HomeMsg::Submit => {
            model.output = model.context.execute(&model.input);
            model.input = String::new();
        },
        HomeMsg::Nothing => {},
    };
}

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![C.flex, C.flex_col, C.justify_center, C.flex_1,],
        vec![
            input![
                class![C.border_primary, C.border_2],
                attrs! {At::Type => "text", At::Name => "input", At::Value => model.input},
                input_ev(Ev::Input, |s| Msg::Home(HomeMsg::Input(s))),
                keyboard_ev("keydown", |ev| if ev.key_code() == 13 {
                    Msg::Home(HomeMsg::Submit)
                } else {
                    Msg::Home(HomeMsg::Nothing)
                }),
            ],
            p![model.output]
        ]
    ]
}
