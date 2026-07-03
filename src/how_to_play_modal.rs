use bevy::{
    DefaultPlugins,
    app::{App, FixedUpdate, Startup, Update},
    asset::{AssetMetaCheck, AssetPlugin, AssetServer, Assets, RenderAssetUsages},
    ecs::prelude::*,
    image::{
        ImageLoaderSettings, ImagePlugin, ImageSamplerDescriptor, TextureAtlas, TextureAtlasLayout,
    },
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
struct HowToPlayModal;

const HTP_TEXT_2: &'static str = "Every card can be identified by four aspects:\
  \n  - Shape: Diamond, Oval, or Squiggle\
  \n  - Quantity: One, Two, or Three\
  \n  - Fill: Empty, Dashed, or Filled\
  \n  - Color: Blue, Pink, or Gold";
const HTP_TEXT_3: &'static str = "A set is a group of three cards where, for every aspect:\
  \n  - The three cards are all different from each other.\
  \n    For example, when considering shapes only, one card consists of diamond(s), another of oval(s), and the third of squiggle(s)
  \n  - The three cards are all the same.\
  \n    For example, when considering shapes only,all three cards consists of diamond(s)";

pub fn spawn(commands: &mut Commands) -> impl Scene {
    commands.spawn_scene(bsn! {
        Modal
        HowToPlayModal
        ZIndex(1)
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            left: percent(5),
            top: percent(5),
            height: percent(90),
            width: percent(90),
            border: px(5),
            padding: px(5),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
        }
        BorderColor::all(GREEN_COLOR)
        BackgroundColor(DEFAULT_BACKGROUND_COLOR)
        Children [
            Node {
                padding: UiRect::axes(percent(5), percent(2)),
            }
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
            ],

            Node {
              padding: UiRect::axes(percent(5), percent(0)),
            }
            Children[
              Text::new(HTP_TEXT_2)
              TextColor(GREEN_COLOR)
              TextFont {
                font_size: FontSize::Px(16.0),
              }
            ],

            Node {
                padding: UiRect::axes(percent(5), percent(0)),
            }
            Children [
              Text::new(HTP_TEXT_3)
              TextColor(GREEN_COLOR)
              TextFont {
                font_size: FontSize::Px(16.0),
              },
            ],

            Node {
                border: UiRect::all(px(5)),
                // TODO have to use px because percent results in a difference between left and right somehow.
                padding: UiRect::percent(25., 25., 2., 2.),
            }
            Button
            BorderColor::all(GREEN_COLOR)
            on_handler_style_button_text::<Over>(TEXT_OVER_COLOR)
            on_handler_style_button_text::<Press>(TEXT_PRESS_COLOR)
            on_handler_style_button_text::<Release>(TEXT_OVER_COLOR)
            on_handler_style_button_text::<Out>(GREEN_COLOR)
            on(|_: On<Pointer<Click>>,
                mut commands: Commands,
                modal_query: Query<Entity, With<HowToPlayModal>>| {
                commands.entity(modal_query.single().unwrap()).despawn();
            })
            Text::new("Close\nWHy")
            TextFont {
                font_size: FontSize::Px(30.0),
            }
            TextLayout::justify(Justify::Center)
            TextColor(GREEN_COLOR)
        ]
    });
}
