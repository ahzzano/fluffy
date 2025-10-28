use std::rc::Rc;

use glow::{Context, HasContext, COLOR_BUFFER_BIT};
use sdl2::{video::{GLContext, Window}, Sdl, VideoSubsystem};

pub struct Display {
    video: VideoSubsystem,
    window: Window,
    gl: Context,
    glContext: GLContext
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
            glContext: context
        }
    }

    fn set_background_color(&mut self) {
        unsafe {
            self.gl.clear_color(0.5, 1.0, 1.0, 1.0);
        }
    }

    pub fn render(&mut self, step: &dyn Fn(&mut Self)) {
        self.set_background_color();
        unsafe{ 
            self.gl.clear(COLOR_BUFFER_BIT);
            step(self);
            self.window.gl_swap_window();
        }
    }
}
