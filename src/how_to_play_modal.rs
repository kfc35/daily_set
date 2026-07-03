use bevy::{
    camera::visibility::Visibility,
    ecs::prelude::*,
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, Justify, TextColor, TextFont, TextLayout, TextSpan},
    ui::prelude::*,
    ui_widgets::Button,
};

use crate::{
    DEFAULT_BACKGROUND_COLOR, GREEN_COLOR, Modal, TEXT_OVER_COLOR, TEXT_PRESS_COLOR,
    on_handler_style_button_text,
};

/// Marker component for the How to Play Modal
#[derive(Component, Clone, Default)]
pub struct HowToPlayModal;

/// Unhides the How to Play Modal
pub fn unhide(mut query: Query<&mut Visibility, With<HowToPlayModal>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Visible
    }
}

/// Spawns the How to Play Modal
pub fn spawn(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Modal
        HowToPlayModal
        Visibility::Hidden
        ZIndex(1)
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            left: percent(5),
            top: percent(5),
            height: percent(90),
            width: percent(90),
            padding: UiRect::horizontal(percent(2))
            border: px(5),
            align_content: AlignContent::Default,
            justify_content: JustifyContent::SpaceEvenly,
        }
        BorderColor::all(GREEN_COLOR)
        BackgroundColor(DEFAULT_BACKGROUND_COLOR)
        Children [
            htp_line_1(),
            htp_line_2(),
            htp_line_3(),
            htp_example_set_4(),
            htp_example_set_5(),

            // Close Button.
            Node {
                border: UiRect::all(px(5)),
                padding: UiRect::vertical(percent(2.)),
                width: percent(50),
                left: percent(25),
            }
            Button
            BorderColor::all(GREEN_COLOR)
            on_handler_style_button_text::<Over>(TEXT_OVER_COLOR)
            on_handler_style_button_text::<Press>(TEXT_PRESS_COLOR)
            on_handler_style_button_text::<Release>(TEXT_OVER_COLOR)
            on_handler_style_button_text::<Out>(GREEN_COLOR)
            on(|event: On<Pointer<Click>>,
                mut commands: Commands,
                parent_q: Query<&ChildOf>| {
                commands.entity(parent_q.root_ancestor(event.entity)).insert(Visibility::Hidden);
            })
            Text::new("Close")
            TextFont {
                font_size: FontSize::Px(30.0),
            }
            TextLayout::justify(Justify::Center)
            TextColor(GREEN_COLOR)
        ]
    });
}

fn htp_line_1() -> impl Scene {
    bsn! {
        Node
        Children[
            Text::new("Goal: To find the ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Px(32.0),
            }
            Children [
                  TextSpan::new("six sets ")
                  TextColor(TEXT_OVER_COLOR)
                  TextFont {
                    font_size: FontSize::Px(32.0),
                  },

                  TextSpan::new("among the ")
                  TextColor(GREEN_COLOR)
                  TextFont {
                    font_size: FontSize::Px(32.0),
                  },

                  TextSpan::new("twelve cards.")
                  TextColor(TEXT_OVER_COLOR)
                  TextFont {
                    font_size: FontSize::Px(32.0),
                  },
            ]
        ]
    }
}

fn htp_line_2() -> impl Scene {
    bsn! {
        Node
        Children[
            Text::new("Every ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Px(16.0),
            }
            Children[
                TextSpan::new("card")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" can be identified by ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("four aspects")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(":  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("Shape")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(": Diamond, Oval, or Squiggle\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("Quantity")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(": One, Two, or Three\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("Fill")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(": Empty, Dashed, or Filled\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("Color")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(": Blue, Pink, or Gold")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },
            ]
        ]
    }
}

fn htp_line_3() -> impl Scene {
    bsn! {
        Node
        Children[
            Text::new("A ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Px(16.0),
            }
            Children[
                TextSpan::new("set")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" is a group of ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("three cards")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" where, ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("for each individual aspect")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(":\n  - The three cards are ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("all different")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" from each other.\
                    \n    For example, when considering shapes only, \
                    one card consists of diamond(s), \
                    another of oval(s), \
                    and the third of squiggle(s).")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("\n  - The three cards are ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("all the same")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(".\
                    \n    For example, when considering shapes only, \
                    all three cards consists of diamond(s).")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },
            ]
        ]
    }
}

fn htp_example_set_4() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_content: AlignContent::Center,
        }
        Children [
            Node {
              padding: UiRect::right(percent(2)),
            }
            Text::new("This ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Px(16.0),
            }
            TextLayout::justify(Justify::Right)
            Children [
                TextSpan::new("is a set")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" consisting of cards with ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("all different")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" shapes, quantities, and fills AND ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("all the same")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" color.")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },
            ],

            Node {
                border: px(5),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                max_height: px(64),
            }
            BorderColor::all(GREEN_COLOR)
            Children [
                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/diamond/diamond_1_D_oiblue.png"
                },

                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/oval/oval_2_E_oiblue.png"
                },

                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/squiggle/squiggle_3_F_oiblue.png"
                },
            ],
        ]
    }
}

fn htp_example_set_5() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_content: AlignContent::Center,
        }
        Children [
            Node {
              padding: UiRect::right(percent(2)),
            }
            Text::new("This is ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Px(16.0),
            }
            TextLayout::justify(Justify::Right)
            Children [
                TextSpan::new("NOT")
                TextColor(TEXT_PRESS_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" a set. The cards all share the same shape and fill. \
                  They all differ in color. However, only ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new("two of three cards")
                TextColor(TEXT_PRESS_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },

                TextSpan::new(" share the same quantity.")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Px(16.0),
                },
            ],

            Node {
                border: px(5),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                max_height: px(64),
            }
            BorderColor::all(TEXT_PRESS_COLOR)
            Children [
                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/diamond/diamond_2_E_oipink.png"
                },

                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/diamond/diamond_2_E_oiblue.png"
                },

                Node {
                  width: percent(32)
                }
                BackgroundColor(bevy::color::Color::WHITE)
                ImageNode {
                    image: "card/diamond/diamond_3_E_oigold.png"
                },
            ],
        ]
    }
}
