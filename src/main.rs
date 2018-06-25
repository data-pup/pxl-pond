extern crate pxl;

use std::collections::HashSet;

use pxl::{Event, Pixel};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

const FINGER_X: usize = 100;
const FINGER_Y: usize = 100;
const FINGER_WIDTH: usize = 10;

const TOUCHED_TEMP: f32 = 0.0;
const DEF_TEMP: f32 = 1.0;

#[derive(Debug)]
struct Droplet {
    temp: f32,
}

#[derive(Debug)]
struct Pond {
    droplets: Vec<Droplet>,
}

impl Pond {
    /// Process a new event.
    fn process_event(&mut self, event: &Event) {
        match event {
            Event::Button {
                state: pxl::ButtonState::Pressed,
                button: pxl::Button::Action,
            } => self.touch(),
            Event::Button {
                state: pxl::ButtonState::Released,
                button: pxl::Button::Action,
            } => self.release(),
            _ => {}
        }
    }

    /// Place the finger in the pond.
    fn touch(&mut self) {
        self.update_epicenter(TOUCHED_TEMP);
    }

    /// Release the finger from the pond.
    fn release(&mut self) {
        self.update_epicenter(DEF_TEMP);
    }

    /// Generic function for touching or releasing the epicenter.
    fn update_epicenter(&mut self, new_val: f32) {
        let epicenter = Pond::get_ripple_epicenter();
        println!("{:?}", epicenter);
        self.droplets
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| epicenter.contains(i))
            .for_each(|(_, droplet)| droplet.temp = new_val);
    }

    /// Get a hash set of the indices affected by touching the pond.
    fn get_ripple_epicenter() -> HashSet<usize> {
        (FINGER_Y..FINGER_Y + FINGER_WIDTH)
            .flat_map(move |y| (FINGER_X..FINGER_X + FINGER_WIDTH).map(move |x| (x, y)))
            .map(|(x, y)| Pond::index(x, y))
            .collect::<HashSet<_>>()
    }

    /// Get the Vec indices for the given coordinates.
    fn index(x: usize, y: usize) -> usize {
        x + y * WIDTH
    }
}

impl pxl::Program for Pond {
    /// Initialize a new Program object.
    fn new() -> Pond {
        Pond {
            droplets: (0..WIDTH * HEIGHT)
                .into_iter()
                .map(|_| Droplet { temp: DEF_TEMP })
                .collect(),
        }
    }

    /// Return the desired width and height of pixel surface.
    fn dimensions() -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    /// Draw to the display
    ///
    /// Called by the runtime whenever the display is ready to present a new frame.
    fn render(&mut self, pixels: &mut [Pixel]) {
        assert_eq!(pixels.len(), self.droplets.len());
        for (pixel, droplet) in pixels.iter_mut().zip(&self.droplets) {
            *pixel = match droplet {
                Droplet { temp } => Pixel {
                    red: 0.0,
                    green: 0.0,
                    blue: *temp,
                    alpha: 1.0,
                },
            }
        }
    }

    /// Process events and update the state of the program.
    ///
    /// Called by the runtime 60 times per second.
    fn tick(&mut self, events: &[Event]) {
        events.iter().for_each(|e| self.process_event(e));
    }
}

fn main() {
    pxl::run::<Pond>();
}
