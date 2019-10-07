#![allow(dead_code)]
#![allow(unused_imports)]
/**
 * This code is based largely off of two examples / tutorials:
 * https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs
 * https://wiki.alopex.li/LearningGfx
 * 
 * I've just been trimming it down to what I actually need to then recreate the sample 
 * project that Dr. Bailey has given us.
 */
use glutin::event::{Event, WindowEvent, StartCause, ElementState};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{GlRequest, ContextBuilder, WindowedContext, PossiblyCurrent};
use glutin::dpi::{LogicalSize, LogicalPosition};
use glutin::Api::OpenGl;
use glutin::event::*;

use cgmath::{Matrix4, Deg, PerspectiveFov, Point3, Vector3};

use rand::prelude::*;

use std::time::Instant;
use std::ffi::CStr;

mod gl {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use gl::types::*;

struct DisplayLists {
	ship: GLuint,
	axis: GLuint
}
struct ButtonStates {
	left: ElementState,
	middle: ElementState,
	right: ElementState
}
const WATER_SIZE: usize = 20;
struct Demo {
	wrapped_context: WindowedContext<PossiblyCurrent>,
	last_animate: Option<Instant>,
	yrot: f32,
	xrot: f32,
	scale: f32,
	display_lists: Option<DisplayLists>,
	// fog_on: bool
	button_states: ButtonStates,
	water_offsets: [[f32; WATER_SIZE]; WATER_SIZE],
	water_time: f32
}

impl Demo {
	fn new(wrapped_context: WindowedContext<PossiblyCurrent>) -> Demo {
		println!(
			"Pixel format of the window's GL context: {:?}",
			wrapped_context.get_pixel_format()
		);
		gl::load_with(|ptr| wrapped_context.context().get_proc_address(ptr) as *const _);
		
		let mut water_offsets = [[0 as f32; WATER_SIZE]; WATER_SIZE];
		let mut rng = rand::thread_rng();
		for row in water_offsets.iter_mut() {
			for item in row.iter_mut() {
				*item = rng.gen_range(0.0, 1000.0);
			}
		}

		Demo { 
			wrapped_context,
			last_animate: None,
			yrot: 0.0, xrot: 0.0,
			display_lists: None,
			scale: 1.0,
			// fog_on: false
			button_states: ButtonStates {
				left: ElementState::Released,
				middle: ElementState::Released,
				right: ElementState::Released
			},
			water_offsets,
			water_time: 0.0
		}
	}
	fn build_display_lists(&mut self) -> DisplayLists {
		let ship = unsafe {
			gl::GenLists(1)
		};
		unsafe {
			gl::NewList(ship, gl::COMPILE);
				let width = 1.0;
				let height = 1.0;
				let bow_lead = 1.0;
				let length = 3.0;
				// Haul:
				gl::Color3f(43.0 / 255.0, 26.0 / 255.0, 31.0 / 255.0);
				gl::Begin(gl::TRIANGLE_STRIP);
					gl::Vertex3f(0.0, 0.0, length);
					gl::Vertex3f(0.0, height, length);
					gl::Vertex3f(width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(width / 2.0, height, length - bow_lead);
					gl::Vertex3f(width / 2.0, 0.0, 0.0);
					gl::Vertex3f(width / 2.0, height, 0.0);

					gl::Vertex3f(-width / 2.0, 0.0, 0.0);
					gl::Vertex3f(-width / 2.0, height, 0.0);

					gl::Vertex3f(-width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(-width / 2.0, height, length - bow_lead);
					gl::Vertex3f(0.0, height, length);
					gl::Vertex3f(0.0, 0.0, length);
					gl::Vertex3f(-width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(width / 2.0, 0.0, length - bow_lead);
					gl::Vertex3f(-width / 2.0, 0.0, 0.0);
					gl::Vertex3f(width / 2.0, 0.0, 0.0);
				gl::End( );
				// Sail
				gl::Color3f(1.0, 1.0, 1.0);
				gl::Begin(gl::QUADS);
					gl::Vertex3f(width / 2.0, height * 2.5, length / 2.0);
					gl::Vertex3f(-width / 2.0, height * 2.5, length / 2.0);
					gl::Vertex3f(-width / 2.0, height * 1.5, length / 2.0);
					gl::Vertex3f(width / 2.0, height * 1.5, length / 2.0);
				gl::End();
				// Mast
				gl::Color3f(99.0 / 255.0, 55.0 / 255.0, 44.0 / 255.0);
				gl::LineWidth(50.0);
				gl::Begin(gl::LINES);
					gl::Vertex3f(0.0, 0.0, length / 2.0);
					gl::Vertex3f(0.0, height * 2.5, length / 2.0);
				gl::End();
			gl::EndList();
		}
		let axis = unsafe {
			gl::GenLists( 1 )
		};
		unsafe {
			gl::NewList( axis, gl::COMPILE );
				gl::LineWidth( 1.0 );
				gl::Begin(gl::LINE_STRIP);
				let d = 0.2;
				let l = 2.0;
				gl::Vertex3f(0.0, 0.0, 0.0);
				gl::Vertex3f(0.0, l, 0.0);
				gl::Vertex3f(-d, l - d, 0.0);
				gl::Vertex3f(0.0, l, 0.0);
				gl::Vertex3f(d, l - d, 0.0);
				gl::End();
				gl::LineWidth( 1.0 );
			gl::EndList( );
		}
		DisplayLists {
			ship,
			axis
		}
	}
	fn init(&mut self) {
		let version = unsafe {
			let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
				.to_bytes()
				.to_vec();
			String::from_utf8(data).unwrap()
		};
		println!("OpenGL version {}", version);
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Enable(gl::DEPTH_TEST);
			// since we are using glScalef( ), be sure normals get unitized:
			gl::Enable( gl::NORMALIZE );
		}

		self.display_lists = Some(self.build_display_lists());

	}
	fn draw_axis(&mut self) {
		if let Some(ref lists) = self.display_lists {
			unsafe {
				// Y-axis label
				gl::Color3f(0.0, 1.0, 0.0);
				gl::CallList(lists.axis);
				gl::Color3f(1.0, 0.0, 0.0);
				// X-axis label
				gl::Color3f(1.0, 0.0, 0.0);
				gl::PushMatrix();
				gl::Rotatef(90.0, 0.0, 0.0, 1.0);
				gl::CallList(lists.axis);
				gl::PopMatrix();
				// Z-axis label
				gl::Color3f(0.0, 0.0, 1.0);
				gl::PushMatrix();
				gl::Rotatef(90.0, 1.0, 0.0, 0.0);
				gl::CallList(lists.axis);
				gl::PopMatrix();
			}
		}
	}
	fn draw_ocean(&mut self) {
		unsafe {
			gl::Begin(gl::QUADS);
				gl::PushMatrix();
				gl::LoadIdentity();
				let total_width = 15.0;
				let total_height = total_width;
				let width = total_width / self.water_offsets.len() as f32;
				let height = total_height / self.water_offsets.len() as f32;
				let wave_height = 0.2;
				let red_start = 17.0;
				let green_start = 29.0;
				let blue_start = 74.0;
				let red_cof = (103.0 - red_start) / 2.0;
				let green_cof = (122.0 - green_start) / 2.0;
				let blue_cof = (140.0 - blue_start) / 2.0;
				for r in 0..(self.water_offsets.len() - 1) {
					for c in 0..(self.water_offsets.len() - 1) {
						let tl = (self.water_time + self.water_offsets[r][c]).sin() + 1.0;
						
						gl::Color3f(
							(red_start + red_cof * tl) / 255.0, 
							(green_start + green_cof * tl) / 255.0, 
							(blue_start + blue_cof * tl) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * r as f32, 
							tl * wave_height, 
							-total_height / 2.0 + height * c as f32
						);
						let tr = (self.water_time + self.water_offsets[r][c + 1]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * tr) / 255.0, 
							(green_start + green_cof * tr) / 255.0, 
							(blue_start + blue_cof * tr) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * r as f32, 
							tr * wave_height, 
							-total_height / 2.0 + height * (c + 1) as f32
						);
						let br = (self.water_time + self.water_offsets[r + 1][c + 1]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * br) / 255.0, 
							(green_start + green_cof * br) / 255.0, 
							(blue_start + blue_cof * br) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * (r + 1) as f32, 
							br * wave_height, 
							-total_height / 2.0 + height * (c + 1) as f32
						);
						let bl = (self.water_time + self.water_offsets[r + 1][c]).sin() + 1.0;
						gl::Color3f(
							(red_start + red_cof * bl) / 255.0, 
							(green_start + green_cof * bl) / 255.0, 
							(blue_start + blue_cof * bl) / 255.0
						);
						gl::Vertex3f(
							-total_width / 2.0 + width * (r + 1) as f32, 
							bl * wave_height, 
							-total_height / 2.0 + height * c as f32
						);
					}
				}
				gl::Translatef(total_width / 2.0, 0.0, total_height / 2.0);
				gl::PopMatrix();
			gl::End();
		}
	}
	fn draw(&mut self) {
		println!("Drawing.");

		unsafe {
			gl::DrawBuffer( gl::BACK );
			gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );

			gl::MatrixMode( gl::PROJECTION );

			// This is essentially a gluPerspective call
			let projection: Matrix4<f32> = PerspectiveFov {
				fovy: Deg(45.0).into(), 
				aspect: 1.0, 
				near: 0.1, 
				far: 1000.0
			}.into();
			let arr: &[f32; 16] = projection.as_ref();
			gl::LoadMatrixf(arr.as_ptr());

			gl::MatrixMode( gl::MODELVIEW );

			// This is essentially a gluLookAt call
			let mat: Matrix4<f32> = Matrix4::look_at(
				Point3::new(-1.0, 2.0, 3.0), 
				Point3::new(0.0, 0.0, 0.0),
				Vector3::new(0.0, 1.0, 0.0)
			);
			let arr: &[f32; 16] = mat.as_ref();
			gl::LoadMatrixf(arr.as_ptr());

			gl::ShadeModel(gl::FLAT);

			// rotate the scene:
			gl::Rotatef(self.yrot, 0.0, 1.0, 0.0);
			gl::Rotatef(self.xrot, 1.0, 0.0, 1.0);

			// uniformly scale the scene:
			gl::Scalef( self.scale, self.scale, self.scale );

			// Draw the objects:
			if let Some(ref lists) = self.display_lists {
				gl::CallList( lists.ship );
				self.draw_axis();
			}
			self.draw_ocean();

			gl::Flush();
        }
		self.wrapped_context.swap_buffers().unwrap();
	}
	fn animate(&mut self) {
		let now = Instant::now();
		match self.last_animate {
			None => {
				// We haven't animated yet.
			},
			Some(last_inst) => {
				// Diff is the # of miliseconds since the last animate call.
				let diff = now.duration_since(last_inst).as_millis();
				self.yrot += diff as f32 / 50.0;
				self.water_time += diff as f32 / 500.0;
			}
		}
		self.last_animate = Some(now);

		self.wrapped_context.window().request_redraw();
	}
	fn resized(&mut self, logical_size: &LogicalSize) {
		let dpi_factor = self.wrapped_context.window().hidpi_factor();
		self.wrapped_context.resize(logical_size.to_physical(dpi_factor));
	}
	fn mouse_move(&mut self, diff_x: f32, diff_y: f32) {
		if self.button_states.left == ElementState::Pressed {
			self.yrot += diff_x;
			self.xrot += diff_y;
		}
	}
	fn scroll_delta(&mut self, diff_y: f32) {
		self.scale += diff_y / 80.0;
		if self.scale < 0.05 {
			self.scale = 0.05;
		}
	}
}

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

    let windowed_context =
        ContextBuilder::new()
		.with_gl(GlRequest::Specific(OpenGl, (2, 1)))
		.with_vsync(true)
		.build_windowed(wb, &event_loop).unwrap();
	
	let mut demo = Demo::new(unsafe { windowed_context.make_current().unwrap() });

	demo.init();
	demo.animate(); // Initialize variables and 

    event_loop.run(move |event, _, control_flow| {
		println!("{:?}", event);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
					demo.resized(logical_size);
                }
                WindowEvent::RedrawRequested => {
					demo.draw();
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
			Event::DeviceEvent{ ref event, .. } => match event {
				// Pass mouse movement to the demo:
				DeviceEvent::MouseMotion{ delta: (x, y) } => demo.mouse_move(*x as f32, *y as f32),
				// Pass the mouse scroll to the demo:
				DeviceEvent::MouseWheel{ delta } => match delta {
					MouseScrollDelta::LineDelta(_, forward) => {
						let y = -forward;
						demo.scroll_delta(y);
					},
					MouseScrollDelta::PixelDelta(LogicalPosition{ y, .. }) => {
						demo.scroll_delta(*y as f32);
					}
				},
				// Keep the button states up to date:
				DeviceEvent::Button{button: 1, state } => demo.button_states.left = *state,
				DeviceEvent::Button{button: 2, state } => demo.button_states.middle = *state,
				DeviceEvent::Button{button: 3, state } => demo.button_states.right = *state,

				_ => ()
			},
			Event::NewEvents(StartCause::Poll) => {
				demo.animate();
			},
            _ => (),
        }
    });
}
