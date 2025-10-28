mod display;

fn main() {
    println!("Hello, world!");
    let sdl = sdl2::init().unwrap();
    let mut _display = display::Display::init(&sdl);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut i = 0;


    'main_loop: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { .. } => {
                    break 'main_loop;
                }
                _ => {}
            }
        }
        _display.render();
    }
}
