#[macro_use]
extern crate itertools;
extern crate pxl;

use std::collections::HashSet;

use pxl::{Event, Pixel};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

const FINGER_X: usize = 100;
const FINGER_Y: usize = 100;
const FINGER_WIDTH: usize = 10;

const TOUCHED_TEMP: f32 = 0.0;
const DEF_TEMP: f32 = 0.5;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }

    fn distance(&self, other: Coordinate) -> f64 {
        let x_d = (self.x as f64 - other.x as f64).powi(2);
        let y_d = (self.y as f64 - other.y as f64).powi(2);
        (x_d + y_d).sqrt()
    }
}

#[derive(Debug)]
struct Droplet {
    height: f32,
}

#[derive(Debug)]
struct Ripple {
    age: f64,
    epicenter: Coordinate,
    epicenter_width: usize,
}

impl Ripple {
    const RIPPLE_MAX_AGE: f64 = 500.0;

    fn new() -> Ripple {
        Ripple {
            age: 0.0,
            epicenter: Coordinate {
                x: FINGER_X,
                y: FINGER_Y,
            },
            epicenter_width: FINGER_WIDTH,
        }
    }

    fn tick(&self) -> Option<Ripple> {
        let Ripple {
            age,
            epicenter,
            epicenter_width,
        } = self;
        if *age >= Ripple::RIPPLE_MAX_AGE {
            None
        } else {
            Some(Ripple {
                age: age + 1.0,
                epicenter: *epicenter,
                epicenter_width: *epicenter_width,
            })
        }
    }

    fn get_height(&self, distance: f64) -> f64 {
        self.age.sin() / (self.age * distance)
    }
}

#[derive(Debug)]
struct Pond {
    ripples: Vec<Ripple>,
    droplets: Vec<Droplet>,

    // Used for Iterator trait.
    curr_x: usize,
    curr_y: usize,
}

impl Pond {
    fn new() -> Pond {
        Pond {
            droplets: (0..WIDTH * HEIGHT)
                .into_iter()
                .map(|_| Droplet { height: DEF_TEMP })
                .collect(),
            ripples: vec![],
            curr_x: 0,
            curr_y: 0,
        }
    }

    /// Process a new event.
    fn process_event(&mut self, event: &Event) {
        match event {
            Event::Button {
                state: pxl::ButtonState::Pressed,
                button: pxl::Button::Action,
            } => self.touch(),
            // Event::Button {
            //     state: pxl::ButtonState::Released,
            //     button: pxl::Button::Action,
            // } => self.release(),
            _ => {}
        }
    }

    /// Place the finger in the pond.
    fn touch(&mut self) {
        self.ripples.push(Ripple::new());
    }

    /// Release the finger from the pond.
    fn release(&mut self) {
        // self.ripples.push(Ripple::new());
    }

    /// Generic function for touching or releasing the epicenter.
    fn update_epicenter(&mut self, new_val: f32) {
        let epicenter = Pond::get_ripple_epicenter().collect::<HashSet<_>>();
        self.droplets
            .iter_mut()
            .enumerate()
            .filter_map(
                |(i, d): (usize, &mut Droplet)| -> Option<(Coordinate, &mut Droplet)> {
                    if let Some(c) = Pond::coord(i) {
                        Some((c, d))
                    } else {
                        None
                    }
                },
            )
            .filter(|(c, _)| epicenter.contains(c))
            .for_each(|(_, mut droplet)| droplet.height = new_val);
    }

    /// Get a hash set of the indices affected by touching the pond.
    fn get_ripple_epicenter() -> impl Iterator<Item = Coordinate> {
        (FINGER_Y..FINGER_Y + FINGER_WIDTH)
            .flat_map(move |y| (FINGER_X..FINGER_X + FINGER_WIDTH).map(move |x| (x, y)))
            .filter(|(x, y)| Pond::in_bounds(*x, *y))
            .map(|(x, y)| Coordinate::new(x, y))
    }

    /// Get the Vec indices for the given coordinates.
    fn index(x: usize, y: usize) -> Option<usize> {
        if Pond::in_bounds(x, y) {
            Some(x + y * WIDTH)
        } else {
            None
        }
    }

    fn coord(i: usize) -> Option<Coordinate> {
        if i >= WIDTH * HEIGHT {
            None
        } else {
            let x = i % WIDTH;
            let y = i / WIDTH;
            Some(Coordinate::new(x, y))
        }
    }

    fn in_bounds(x: usize, y: usize) -> bool {
        x < WIDTH && y < HEIGHT
    }
}

impl pxl::Program for Pond {
    /// Initialize a new Program object.
    fn new() -> Pond {
        Pond::new()
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
        self.ripples = self.ripples.iter().filter_map(|r| r.tick()).collect();
        // for (pixel, droplet) in pixels.iter_mut().zip(&self.droplets) {
        //     *pixel = match droplet {
        //         Droplet { height } => Pixel {
        //             red: 0.0,
        //             green: 0.0,
        //             blue: *height,
        //             alpha: 1.0,
        //         },
        //     }
        // }
        let pixels_iter = pixels.iter_mut();
        let droplets_iter = self.droplets.iter();
        let coord_iter = (0..HEIGHT * WIDTH).filter_map(|i| Pond::coord(i));
        for (pixel, droplet, coord) in izip!(pixels_iter, droplets_iter, coord_iter) {
            self.ripples
                .iter()
                .map(|r| {
                    let d = r.epicenter.distance(coord);
                    r.get_height(d)
                })
                .fold(0.0, |res, curr| res + curr);
            unimplemented!();
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
