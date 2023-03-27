extern crate sdl2;
extern crate gl;

// open gl docs locally -> cargo doc -p gl --no-deps --open
//

fn main()
{
    // println!("Hello, world!");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Scop", 900, 700).opengl().resizable().build().unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'main: loop
    {
        for event in event_pump.poll_iter()
        {
            //handle user input
            match event
            {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
        // render contents
    }
}
