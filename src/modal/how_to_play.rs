use bevy::{
    asset::{AssetServer, Assets},
    camera::visibility::Visibility,
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::{UVec2, Vec2},
    picking::prelude::*,
    scene::prelude::*,
    text::{FontSize, Justify, TextColor, TextFont, TextLayout, TextSpan},
    ui::prelude::*,
    ui_widgets::{Button, ControlOrientation, Scrollbar, ScrollbarThumb},
};

use crate::{
    DEFAULT_BACKGROUND_COLOR, GREEN_COLOR, Modal, RED_COLOR, TEXT_OVER_COLOR, TEXT_PRESS_COLOR,
    on_handler_style_button_image,
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
            display: Display::Grid,
            left: percent(5),
            top: percent(5),
            height: percent(90),
            width: percent(85),
            border: px(5),
            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
        }

        BackgroundColor(DEFAULT_BACKGROUND_COLOR)
        BorderColor::all(GREEN_COLOR)
        Children [
            #Content
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::horizontal(percent(5)),
                align_content: AlignContent::Default,
                justify_content: JustifyContent::SpaceEvenly,
                overflow: Overflow::scroll_y(),
            }
            ScrollPosition::default()
            Children [
                htp_line_1(),
                htp_line_2(),
                htp_line_3(),
                { htp_example_set_4() },
                { htp_example_set_5() },
                htp_line_6(),
                // score_image(),
                htp_line_7(),

                // Close Button.
                Node {
                    border: UiRect::all(px(5)),
                    padding: UiRect::vertical(percent(2.)),
                    width: percent(50),
                    height: percent(20),
                    left: percent(25),
                }
                Button
                BorderColor::all(GREEN_COLOR)
                on(|event: On<Pointer<Click>>,
                    mut commands: Commands,
                    parent_q: Query<&ChildOf>,
                    mut scroll_position: Query<&mut ScrollPosition> | {
                    commands.entity(parent_q.root_ancestor(event.entity)).insert(Visibility::Hidden);
                    for mut scroll_pos in scroll_position.iter_mut() {
                        scroll_pos.0 = Vec2::ZERO;
                    }
                })
                on_handler_style_button_image::<Over>(TEXT_OVER_COLOR, 1)
                on_handler_style_button_image::<Press>(TEXT_PRESS_COLOR, 2)
                on_handler_style_button_image::<Release>(TEXT_OVER_COLOR, 1)
                on_handler_style_button_image::<Out>(GREEN_COLOR, 0)
                // Unsure how to do this by just having to modify the texture_atlas of the ImageNode
                template(move |context| {
                    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 16), 1, 3, None, None);
                    let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                    let texture_atlas = TextureAtlas {
                        layout: layout_handle,
                        index: 0,
                    };
                    Ok(ImageNode {
                        image: context.resource::<AssetServer>().load("menu/close.png"),
                        texture_atlas: Some(texture_atlas),
                        ..Default::default()
                    })
                })
            ],

            // Scrollbar
            Node {
                min_width: px(10),
            }
            Scrollbar {
                orientation: ControlOrientation::Vertical,
                target: #Content,
                min_thumb_length: 8.0,
            }
            Children [
                BorderColor::all(GREEN_COLOR)
                ScrollbarThumb {
                    border_radius: BorderRadius::all(px(4)),
                    border: UiRect::all(px(1)),
                }
            ],
        ]
    });
}

fn htp_line_1() -> impl Scene {
    bsn! {
        Node {
          padding: UiRect::top(percent(5)),
        }
        Children[
            Text::new("Goal")
            TextColor(TEXT_OVER_COLOR)
            TextFont {
              font_size: FontSize::Rem(1.5),
            }
            Children [
                  TextSpan::new(": To find the ")
                  TextColor(GREEN_COLOR)
                  TextFont {
                    font_size: FontSize::Rem(1.5),
                  },

                  TextSpan::new("six sets ")
                  TextColor(TEXT_OVER_COLOR)
                  TextFont {
                    font_size: FontSize::Rem(1.5),
                  },

                  TextSpan::new("among the ")
                  TextColor(GREEN_COLOR)
                  TextFont {
                    font_size: FontSize::Rem(1.5),
                  },

                  TextSpan::new("twelve cards.")
                  TextColor(TEXT_OVER_COLOR)
                  TextFont {
                    font_size: FontSize::Rem(1.5),
                  },
            ]
        ]
    }
}

fn htp_line_2() -> impl Scene {
    bsn! {
        Node
        Children[
            Text::new("\nEvery ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Rem(1.0),
            }
            Children[
                TextSpan::new("card")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" can be identified by ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("four aspects")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(":  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("Shape")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(": Diamond, Oval, or Squiggle\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("Quantity")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(": One, Two, or Three\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("Fill")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(": Empty, Dashed, or Filled\
                                  \n  - ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("Color")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(": Blue, Pink, or Gold\n")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
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
              font_size: FontSize::Rem(1.0),
            }
            Children[
                TextSpan::new("set")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" is a group of ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("three cards")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" where, ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("for each individual aspect, either")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(":\n\n  A) The three cards are ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("all different")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" from each other.\
                    \n    For example, when considering shapes only, \
                    one card consists of diamond(s), \
                    another of oval(s), \
                    and the third of squiggle(s).")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("\n\n  B) The three cards are ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("all the same")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(".\
                    \n    For example, when considering shapes only, \
                    all three cards consists of diamond(s).\n")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },
            ]
        ]
    }
}

fn htp_example_set_4() -> impl SceneList {
    bsn_list! [
            Node {
                width: percent(80),
                border: px(5),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                height: px(100),
            }
            BorderColor::all(GREEN_COLOR)
            BackgroundColor(bevy::color::Color::WHITE)
            Children [
                Node {
                  width: percent(33),
                  height: percent(100),
                }
                ImageNode {
                    image: "card/diamond/diamond_1_D_oiblue.png"
                },

                Node {
                  width: percent(33)
                  height: percent(100),
                }
                ImageNode {
                    image: "card/oval/oval_2_E_oiblue.png"
                },

                Node {
                  width: percent(33)
                  height: percent(100),
                }
                ImageNode {
                    image: "card/squiggle/squiggle_3_F_oiblue.png"
                },
            ],

            Node {
              align_self: AlignSelf::Center,
              align_content: AlignContent::Center,
              width: percent(80),
            }
            Text::new("\nThis ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Rem(1.0),
            }
            TextLayout::justify(Justify::Center)
            Children [
                TextSpan::new("is a set")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" consisting of cards with ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("all different")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" shapes, quantities, and fills AND ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("all the same")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" color.\n")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },
            ],
    ]
}

fn htp_example_set_5() -> impl SceneList {
    bsn_list! [
            Node {
                width: percent(80),
                border: px(5),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                height: px(100),
            }
            BackgroundColor(bevy::color::Color::WHITE)
            BorderColor::all(RED_COLOR)
            Children [
                Node {
                  width: percent(33)
                }
                ImageNode {
                    image: "card/diamond/diamond_2_E_oipink.png"
                },

                Node {
                  width: percent(33)
                }
                ImageNode {
                    image: "card/diamond/diamond_2_E_oiblue.png"
                },

                Node {
                  width: percent(33)
                }
                ImageNode {
                    image: "card/diamond/diamond_3_E_oigold.png"
                },
            ],

            Node {
              align_self: AlignSelf::Center,
              align_content: AlignContent::Center,
              width: percent(80),
            }
            TextLayout::justify(Justify::Center)
            Text::new("\nThis is ")
            TextColor(GREEN_COLOR)
            TextFont {
              font_size: FontSize::Rem(1.0),
            }
            Children [
                TextSpan::new("NOT")
                TextColor(RED_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" a set. The cards all share the same shape and fill. \
                  They all differ in color. However, only ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("two of three cards")
                TextColor(RED_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" share the same quantity.\n")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },
            ],
    ]
}

fn htp_line_6() -> impl Scene {
    bsn! {
        Node
        Children[
            Text::new("Controls: \n")
            TextColor(TEXT_OVER_COLOR)
            TextFont {
              font_size: FontSize::Rem(1.5),
            }
            Children[
                TextSpan::new("To guess a set, ")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("click/press")
                TextColor(TEXT_PRESS_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },


                TextSpan::new(" on any three cards in succession. Selected cards have a green border and a tinted backing.\n\nTo ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("unselect a card, ")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("click/press")
                TextColor(TEXT_PRESS_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" on the green-bordered card ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("again")
                TextColor(TEXT_PRESS_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(". The border should disappear.\n\nIf you successfully ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("found a set")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(", your ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("score will increase")
                TextColor(TEXT_OVER_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(" and your found set will appear underneath the score.\n\n\
                              If your guess is ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("not a set")
                TextColor(RED_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(", the cards will blink with a ")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new("red border")
                TextColor(RED_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },

                TextSpan::new(".\n")
                TextColor(GREEN_COLOR)
                TextFont {
                  font_size: FontSize::Rem(1.0),
                },
            ]
        ]
    }
}

fn htp_line_7() -> impl Scene {
    bsn! {
        Node {
          justify_content: JustifyContent::Center,
          align_self: AlignSelf::Center,
        }
        Text::new("Good Luck and Have Fun!\n")
        TextColor(TEXT_PRESS_COLOR)
        TextFont {
          font_size: FontSize::Rem(1.5),
        }
        TextLayout::justify(Justify::Center)
    }
}
