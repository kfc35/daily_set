use bevy::{
    asset::{AssetServer, Assets},
    ecs::prelude::*,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    scene::prelude::*,
    time::{Timer, TimerMode},
    ui::prelude::*,
};

use crate::{AnimatedImageNode, AnimationTimer};

/// Struct that denotes an available ResultsBanner.
/// The asset itself should be laid out in a single column.
pub struct ResultsBanner {
    /// The name of the asset under assets/results_banner
    pub asset_name: &'static str,
    /// Contains:
    ///   - the [`TextureAtlasLayout`] size
    ///   - the number of rows in the asset,
    ///   - the maximum frame to possibly cycle to.
    ///   - the timer interval in seconds, default is 0.1.
    ///     If not provided, the results banner will not be animated and
    ///     will just use the first frame by default.
    ///
    /// If layout_information is not provided, the results banner is assumed to be a static image.
    pub animation_information: Option<(UVec2, u32, Option<u32>, f32)>,
}

impl ResultsBanner {
    // Static
    pub const CONGRATULATIONS: ResultsBanner = ResultsBanner::new_static("congratulations.png");
    pub const WELL_DONE: ResultsBanner = ResultsBanner::new_static("well_done.png");
    pub const YOURE_A_DIAMOND: ResultsBanner = ResultsBanner::new_static("youre_a_diamond.png");
    pub const STATIC_RESULTS_BANNER: [ResultsBanner; 3] = [
        ResultsBanner::CONGRATULATIONS,
        ResultsBanner::WELL_DONE,
        ResultsBanner::YOURE_A_DIAMOND,
    ];

    // Animations
    pub const GOAL: ResultsBanner =
        ResultsBanner::new_animation("goal.png", UVec2::new(128, 32), 4);
    pub const LUCKY_YOU: ResultsBanner =
        ResultsBanner::new_animation("lucky_you.png", UVec2::new(192, 96), 6);
    pub const NICE_WORK: ResultsBanner =
        ResultsBanner::new_animation("nice_work.png", UVec2::new(96, 64), 28);
    pub const NOODLE_TIME: ResultsBanner =
        ResultsBanner::new_animation("noodle_time.png", UVec2::new(96, 96), 12);
    pub const SWEET: ResultsBanner =
        ResultsBanner::new_animation("sweet.png", UVec2::new(96, 96), 19);
    pub const ANIMATIONS: [ResultsBanner; 5] = [
        ResultsBanner::GOAL,
        ResultsBanner::LUCKY_YOU,
        ResultsBanner::NICE_WORK,
        ResultsBanner::NOODLE_TIME,
        ResultsBanner::SWEET,
    ];

    // Context dependent
    pub const _HAPPY_CATURDAY: ResultsBanner =
        ResultsBanner::new_static("happy_caturday_perched.png");
    pub const _NICE_TRY: ResultsBanner = ResultsBanner::new_static("nice_try.png");

    pub const fn new_static(asset_name: &'static str) -> Self {
        ResultsBanner {
            asset_name,
            animation_information: None,
        }
    }

    pub const fn _new_static_from_animation(
        asset_name: &'static str,
        texture_layout_size: UVec2,
        max_layout_frame: u32,
    ) -> Self {
        ResultsBanner {
            asset_name,
            animation_information: Some((texture_layout_size, max_layout_frame, None, 0.1)),
        }
    }

    pub const fn new_animation(
        asset_name: &'static str,
        texture_layout_size: UVec2,
        max_layout_frame: u32,
    ) -> Self {
        ResultsBanner {
            asset_name,
            animation_information: Some((
                texture_layout_size,
                max_layout_frame,
                Some(max_layout_frame),
                0.1,
            )),
        }
    }

    pub const fn _new_animation_with_timer(
        asset_name: &'static str,
        texture_layout_size: UVec2,
        max_layout_frame: u32,
        timer_secs: f32,
    ) -> Self {
        ResultsBanner {
            asset_name,
            animation_information: Some((
                texture_layout_size,
                max_layout_frame,
                Some(max_layout_frame),
                timer_secs,
            )),
        }
    }

    pub fn scene(&self) -> Box<dyn Scene> {
        if let Some((texture_layout_size, num_rows, max_layout_frame, timer_secs)) =
            self.animation_information
        {
            if let Some(frame) = max_layout_frame {
                let asset_name = self.asset_name;
                Box::new(bsn! {
                    template(move |context| {
                        let layout = TextureAtlasLayout::from_grid(texture_layout_size, 1, num_rows, None, None);
                        let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                        let texture_atlas = TextureAtlas {
                            layout: layout_handle,
                            index: 0,
                        };
                        Ok(ImageNode {
                            image: context.resource::<AssetServer>().load(format!("results_banner/{}", asset_name)),
                            texture_atlas: Some(texture_atlas),
                            ..Default::default()
                        })
                        })
                    AnimatedImageNode({frame as usize})
                    AnimationTimer(Timer::from_seconds(timer_secs, TimerMode::Repeating))
                })
            } else {
                Box::new(bsn! {
                    template(move |context| {
                        let layout = TextureAtlasLayout::from_grid(texture_layout_size, 1, num_rows, None, None);
                        let layout_handle = context.resource_mut::<Assets<TextureAtlasLayout>>().add(layout);
                        let texture_atlas = TextureAtlas {
                            layout: layout_handle,
                            index: 0,
                        };
                        Ok(ImageNode {
                            image: context.resource::<AssetServer>().load("results_banner/noodle_time.png"),
                            texture_atlas: Some(texture_atlas),
                            ..Default::default()
                        })
                    })
                })
            }
        } else {
            Box::new(bsn! {
                ImageNode {
                    image: format!("results_banner/{}", self.asset_name)
                }
            })
        }
    }
}
