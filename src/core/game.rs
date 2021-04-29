use gl;

pub trait Game {
    const title: &'static str;
    const width: u32;
    const height: u32;

    pub fn run(&self) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(Self::title, Self::width, Self::height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        
        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        self.init();

        let mut event_pump = sdl.event_pump().unwrap();

        'main: loop {
            for event in event_pump.poll_iter() {
                // Handle user input here
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            self.render();

            window.gl_swap_window();
        }
    }

    pub fn init(&self);
    pub fn update(&self);
    pub fn render(&self);
}
