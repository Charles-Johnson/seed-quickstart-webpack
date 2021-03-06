use crate::{
    generated::css_classes::C, image_src, Msg, CHARLES_EMAIL, MAIL_TO_CHARLES,
    TWEET_TO_CHARLES,
};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    footer![
        class![
            C.h_16,
            C.shadow_2xl_above,
            C.flex,
            C.justify_center,
            C.sm__h_24,
        ],
        div![
            class![
                C.w_xs,
                C.h_full,
                C.px_5,
                C.flex,
                C.justify_between,
                C.items_center,
                C.sm__w_132
            ],
            h3![
                attrs! {
                    At::Href => MAIL_TO_CHARLES,
                },
                class![
                    C.font_display,
                    C.font_semibold,
                    C.text_16,
                    C.text_primary,
                    C.sm__text_26
                ],
                CHARLES_EMAIL
            ],
            div![
                class![C.cursor_pointer, C.h_full, C.flex, C.items_center,],
                simple_ev(Ev::Click, Msg::ScrollToTop),
                image_link("메일.svg", MAIL_TO_CHARLES),
                image_link("트위터.svg", TWEET_TO_CHARLES)
            ]
        ]
    ]
}

fn image_link(image: &str, href: &str) -> Node<Msg> {
    a![
        attrs! {
            At::Href => href,
        },
        img![
            class![
                C.mt_1, C.w_12, // sm__
                C.sm__w_16
            ],
            attrs! {
                At::Src => image_src(image)
            }
        ],
    ]
}
