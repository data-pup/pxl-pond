extern crate pxl;

use pxl::Pixel;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

struct Droplet;

struct Pond {
    droplets: Vec<Droplet>,
}

impl Pond {
    fn _index(&self, x: usize, y: usize) -> usize {
        x + y * WIDTH
    }
}

impl pxl::Program for Pond {
    /// Initialize a new Program object.
    fn new() -> Pond {
        Pond {
            droplets: (0..WIDTH * HEIGHT).into_iter().map(|_| Droplet).collect(),
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
        for (pixel, cell) in pixels.iter_mut().zip(&self.droplets) {
            *pixel = match cell {
                _ => Pixel {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
            }
        }
    }
}

fn main() {
    pxl::run::<Pond>();
}
