use crate::{
    generated::css_classes::C, Msg,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view() -> impl View<Msg> {
    div![
        class![
            C.flex,
            C.flex_col,
            C.justify_center,
            C.h_600px,
        ],
        vec![
            form![
                input![
                    attrs!{At::Type => "text", At::Name => "input"}
                ]
            ],
            p![
                ""
            ]
        ]
    ]
}
