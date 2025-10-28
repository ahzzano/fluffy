use std::rc::Rc;

use glow::Context;
use sdl2::{Sdl, VideoSubsystem, video::Window};

pub struct Display {
    video: VideoSubsystem,
    window: Window,
    gl: Context,
}

impl Display {
    pub fn init(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("fluffy", 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let context = window.gl_create_context().unwrap();
        window.gl_make_current(&context).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                video_subsystem.gl_get_proc_address(s) as *const _
            })
        };

        Display {
            video: video_subsystem,
            window,
            gl,
        }
    }
}

