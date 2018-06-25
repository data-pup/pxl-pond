extern crate pxl;

use pxl::{Event, Pixel};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const CLICK_X: usize = 100;
const CLICK_Y: usize = 100;

struct Droplet {
    temp: f32,
}

struct Pond {
    droplets: Vec<Droplet>,
}

impl Pond {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * WIDTH
    }
}

impl pxl::Program for Pond {
    /// Initialize a new Program object.
    fn new() -> Pond {
        Pond {
            droplets: (0..WIDTH * HEIGHT)
                .into_iter()
                .map(|_| Droplet { temp: 1.0 })
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

    fn tick(&mut self, events: &[Event]) {
        for event in events {
            if let Event::Button {
                state: pxl::ButtonState::Pressed,
                button: pxl::Button::Action,
            } = event
            {
                let i = self.index(CLICK_X, CLICK_Y);
                self.droplets[i].temp = 0.0;
            }
        }
    }
}

fn main() {
    pxl::run::<Pond>();
}
