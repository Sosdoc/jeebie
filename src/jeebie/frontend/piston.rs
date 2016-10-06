use piston_window::*;


pub struct PistonFrontend {
    window: PistonWindow,
    pub running: bool,
}

impl PistonFrontend {
    pub fn new_with_size(size: (u32, u32)) -> PistonFrontend {

        let window: PistonWindow =
            WindowSettings::new("Hello Piston!", [size.0, size.1])
            .build()
            .unwrap();

        PistonFrontend {
            window: window,
            running: true,
        }
    }

    pub fn draw(&mut self) {
        // TODO implement the trait for GPUFrontend
        if let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                        [0.0, 0.0, 100.0, 100.0],
                        c.transform, g);
            });
        }
    }

    pub fn update(&mut self) {
        // TODO handle events and closing the window
    }
}