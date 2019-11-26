#![allow(bad_style)]
#![allow(unused)]
/**
 * This code is based largely off of two examples / tutorials:
 * https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs
 * https://wiki.alopex.li/LearningGfx
 *
 * I've just been trimming it down to what I actually need to then recreate the sample
 * project that Dr. Bailey has given us.
 */
use glutin::{
	dpi::LogicalPosition,
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
	Api::OpenGl,
	ContextBuilder, GlRequest,
};

use std::{cell::RefCell, rc::Rc, time::Duration};

mod gl;

mod mesh;
use mesh::*;
mod light;
use light::*;
mod material;
use material::*;
mod demo;
use demo::*;

#[derive(Clone, Copy)]
struct Point {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
impl Point {
	fn scale(&self, factor: f32) -> Point {
		Point {
			x: factor * self.x,
			y: factor * self.y,
			z: factor * self.z
		}
	}
}
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
			x: self.x + other.x, 
			y: self.y + other.y,
			z: self.z + other.z
		}
    }
}
impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
			x: self.x - other.x, 
			y: self.y - other.y,
			z: self.z - other.z
		}
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
    }
}

struct CubicCurve {
	pub control_1: Point,
	pub control_2: Point,
	pub control_3: Point,
	pub control_4: Point
}
impl CubicCurve {
	fn draw(&self, resolution: u32) {
		unsafe {
			gl::Begin(gl::LINE_STRIP);
		}
		for i in 0..(resolution + 1) {
			let t = i as f32 / resolution as f32;
			let tm = 1.0 - t;
			// float x = omt*omt*omt*p0.x + 3.f*t*omt*omt*p1.x + 3.f*t*t*omt*p2.x + t*t*t*p3.x;
			// float y = omt*omt*omt*p0.y + 3.f*t*omt*omt*p1.y + 3.f*t*t*omt*p2.y + t*t*t*p3.y;
			// float z = omt*omt*omt*p0.z + 3.f*t*omt*omt*p1.z + 3.f*t*t*omt*p2.z + t*t*t*p3.z;
			let p = self.control_1.scale(tm * tm * tm) + 
				self.control_2.scale(3.0 * t * tm * tm) +
				self.control_3.scale(3.0 * t * t * tm) +
				self.control_4.scale(t * t * t);
			unsafe {
				gl::Vertex3f(p.x, p.y, p.z);
			}
		}
		unsafe {
			gl::End();
		}
	}
}
fn make_curve(points: [f32; 8]) -> CubicCurve{
	unsafe {
		CubicCurve {
			control_1: Point{ x: points[0], y: points[1], z: 0.0 }, 
			control_2: Point{ x: points[2], y: points[3], z: 0.0 }, 
			control_3: Point{ x: points[4], y: points[5], z: 0.0 }, 
			control_4: Point{ x: points[6], y: points[7], z: 0.0 }
		}
	}
}

fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

	let windowed_context = ContextBuilder::new()
		.with_gl(GlRequest::Specific(OpenGl, (2, 1)))
		.with_vsync(true)
		.build_windowed(wb, &event_loop)
		.unwrap();

	let mut demo = Demo::new(
		unsafe { windowed_context.make_current().unwrap() },
		// All the scene items
		vec![&mut || {
			struct GenerativeProject {
				curves: Vec<CubicCurve>
			}
			impl SceneItem for GenerativeProject {
				fn anim(&mut self, update: Duration) {
					// self.curve_1.control_2
				}
				fn draw(&self) {
					for curve in self.curves.iter() {
						curve.draw(20);
					}
				}
			}
			move_to(Point {
				x: 0.302, y: 1.53, z: 0.0
			});
			Box::new(GenerativeProject {
				curves: vec![
					// Path 1
					make_curve([
						0.186, 0.114, 
						0.00759, 0.348, 
						-0.121, 0.323
					]),
					make_curve([
						-0.0968, -0.0191, 
						-0.188, -0.103, 
						-0.147, -0.239
					]),
					make_curve([
						0.0407, -0.137, 
						0.222, -0.202, 
						0.39, -0.0858
					]),
					make_curve([
						0.406, 0.281, 
						1.09, 0.0819, 
						1.3, 0.123
					]),
					make_curve([
						0.185, 0.0359, 
						0.108, 0.182, 
						0.0571, 0.2
					]),
					make_curve([
						-0.0507, 0.0178, 
						-0.17, -0.0334, 
						-0.0586, -0.128
					]),
					// Path 2
					{	move_to(Point {
							x: 1.93, y: 1.69, z: 0.0
						});
						make_curve([
							0.179, -0.0121,
							0.201, 0.154,
							0.1, 0.167
						])
					},
					make_curve([
						-0.0949, 0.0116,
						-0.112, -0.106,
						-0.0273, -0.106
					]),
					// Path 3
					{	move_to(Point {
							x: 2.1, y: 1.7, z: 0.0
						});
						make_curve([
							0.113, 0.033, 
							0.15, -0.0276, 
							0.24, 0.00114
						])
					},
					make_curve([
						0.111, 0.0355, 
						0.0806, 0.159,
						-0.00377, 0.156
					]),
					make_curve([
						-0.0891, -0.00303,
						-0.104, -0.103,
						-0.0333, -0.12
					]),
					// Path 4
					{	move_to(Point {
							x: 2.64, y: 0.948, z: 0.0
						});
						make_curve([
							0.0377, -0.203,
							-0.18, 0.0113,
							-0.268, 0.243
						])
					},
					make_curve([
						-0.0747, 0.197,
						-0.131, 0.592,
						0.0175, 0.43
					]),
					// Path 5
					{	move_to(Point {
							x: 2.64, y: 0.832, z: 0.0
						});
						make_curve([
							-0.547, 0.0792,
							-1.54, 0.1,
							-1.87, -0.00294
						])
					},
					// Path 6
					{	move_to(Point {
							x: 0.0000157, y: 0.169, z: 0.0
						});
						make_curve([
							0.883, 0.0396,
							0.782, -0.208,
							0.777, 0.639
						])
					},
					// Path 7
					{	move_to(Point {
							x: 0.00612, y: 0.19, z: 0.0
						});
						make_curve([
							0.06, 0.467,
							0.0667, 0.822,
							0.011, 1.38
						])
					},
					// Path 8
					{	move_to(Point {
							x: 0.167, y: 0.254, z: 0.0
						});
						make_curve([
							0.496, -0.0114,
							0.507, -0.0475,
							0.493, 0.291
						])
					},
					make_curve([
						-0.0119, 0.279,
						-0.524, 0.32,
						-0.518, 0.032
					]),
					// Path 9 - straight line
					{	move_to(Point {
							x: 1.98, y: 0.868, z: 0.0
						});
						make_curve([
							1.98, 0.868,
							0.0702, -0.829,
							0.0702, -0.829
						])
					},
					// Path 10 - straight line
					{	move_to(Point {
							x: 2.16, y: 0.864, z: 0.0
						});
						make_curve([
							2.16, 0.864,
							0.127, -0.797,
							0.127, -0.797
						])
					},
					// Path 11 - straight line
					{	move_to(Point {
							x: 2.32, y: 0.858, z: 0.0
						});
						make_curve([
							2.32, 0.858,
							0.0412, -0.794,
							0.0412, -0.794
						])
					},
					// Path 12 - straight line
					{	move_to(Point {
							x: 2.46, y: 0.842, z: 0.0
						});
						make_curve([
							2.46, 0.842,
							0.145, -0.781,
							0.145, -0.781
						])
					},
					// Path 13
					{	move_to(Point {
							x: 2.09, y: 0.0477, z: 0.0
						});
						make_curve([
							0.165, 0.0167,
							0.288, 0.00469,
							0.118, -0.0318
						])
					},
					make_curve([
						-0.174, -0.0373,
						-0.211, 0.0126,
						-0.0757, 0.0151
					]),
					// Path 14
					{	move_to(Point {
							x: 2.59, y: 0.072, z: 0.0
						});
						make_curve([
							-0.297, 0.0273,
							-0.264, -0.0686,
							-0.0894, -0.0636
						])
					},
					make_curve([
						0.185, 0.00527,
						0.109, 0.0504,
						0.0348, 0.053
					]),
				]
			})
		}],
	);
	demo.animate(); // Initialize variables and

	// Proxy used for sending commands from the context menu to the main loop
	event_loop.run(move |event, window_target, control_flow| {
		// println!("{:?}", event);
		*control_flow = ControlFlow::Poll;

		demo.handle_event(event, window_target, control_flow);
	});
}
