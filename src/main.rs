/**
 * This code is based largely off of two examples / tutorials:
 * https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs
 * https://wiki.alopex.li/LearningGfx
 * 
 * I've just been trimming it down to what I actually need to then recreate the sample 
 * project that Dr. Bailey has given us.
 */
use glutin::{
	event_loop::{ControlFlow, EventLoop},
	event::*,
	window::WindowBuilder,
	GlRequest, ContextBuilder,
	dpi::LogicalPosition,
	Api::OpenGl
};

use std::{
	time::{
		Duration
	},
	rc::Rc,
	cell::RefCell
};

mod gl;

mod mesh;
use mesh::*;
mod light;
use light::*;
mod material;
use material::*;
mod demo;
use demo::*;


fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

    let windowed_context =
        ContextBuilder::new()
		.with_gl(GlRequest::Specific(OpenGl, (2, 1)))
		.with_vsync(true)
		.build_windowed(wb, &event_loop).unwrap();
	
	/*
	struct LightingProject {
		torus: Torus,
		light_1: Light,
		mat_1: Material,
		light_motion: f32
	}
	impl LightingProject {
		fn new() -> Self {
			LightingProject {

			}
		}
	}
	impl SceneItem for LightingProject {
		fn init() {

		}
		fn anim(&mut self, diff: Duration) {
			self.light_motion += diff.as_secs_f32();
			self.light_1.place(0.0, 20.0 * self.light_motion.sin(), 0.0, 1.0);
		}
		fn draw(&mut self) {
			unsafe {
				self.light_1.call();
				gl::Enable(gl::LIGHTING);
				self.mat_1.call();
				gl::ShadeModel(gl::FLAT);
				self.torus.draw();
				gl::Disable(gl::LIGHTING);
			}
		}
	}
	let lighting_project = LightingProject {
		torus: {
			let mut torus = Torus::new();
			// torus.major_segments = 100;
			// torus.minor_segments = 50;
			torus
		},
		light_1: {
			let mut light = Light::new(0);
			light.attenuate(1.0, 0.0, 0.0);
			light.diffuse(244.0, 179.0, 147.0);
			light.specular(244.0, 179.0, 147.0);
			// Light::use_ambient(0.1, 0.1, 0.1);
			light
		},
		mat_1: {
			let mut material = Material::new(ActiveFace::Front);
			material.diffuse(122.0, 40.0, 203.0);
			material
		},
		light_motion: 0.0
	};
	*/

	let mut demo = Demo::new(
		unsafe { windowed_context.make_current().unwrap() },
		// All the scene items
		vec![
			Box::new(|| {
				// Initialize the scene
				let torus = Rc::new(RefCell::new(Torus::new()));
				{	let mut torus = torus.borrow_mut();
					torus.major_segments = 30;
					torus.minor_segments = 12;
				}
				let torus_ = torus.clone();
				let material = Rc::new(RefCell::new(Material::new(ActiveFace::Front)));
				material.borrow_mut().diffuse(122.0 / 255.0, 40.0 / 255.0, 203.0 / 255.0);
				let light = Rc::new(RefCell::new(Light::new(0)));
				let light_ = light.clone();
				let mut time = Box::new(0.0 as f32);

				SceneItem {
					anim: Box::new(move |duration: Duration| {
						*time += duration.as_secs_f32();
						torus_.borrow_mut().major_radius = (*time * 0.5).sin() * 2.0 + 3.0;
						light_.borrow_mut().place(0.0, time.sin() * 3.0, 0.0, 1.0);
					}),
					draw: Box::new(move || {
                        unsafe {
                            gl::Enable(gl::LIGHTING);
                            gl::ShadeModel(gl::FLAT);
                        }
						light.borrow().call();
                        material.borrow().call();
						torus.borrow().draw();
						unsafe { gl::Disable(gl::LIGHTING); }
					})
				}
			})
		]
	);
	demo.animate(); // Initialize variables and

	// Proxy used for sending commands from the context menu to the main loop
	event_loop.run(move |event, window_target, control_flow| {
		// println!("{:?}", event);
		*control_flow = ControlFlow::Poll;

		demo.handle_event(event, window_target, control_flow);
	});
}
