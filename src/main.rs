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

use std::time::Instant;
use std::ffi::CStr;

mod gl {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use gl::types::*;

struct DisplayLists {
	cube: GLuint,
	axis: GLuint
}
struct ButtonStates {
	left: ElementState,
	middle: ElementState,
	right: ElementState
}
struct Demo {
	wrapped_context: WindowedContext<PossiblyCurrent>,
	last_animate: Option<Instant>,
	yrot: f32,
	xrot: f32,
	scale: f32,
	display_lists: Option<DisplayLists>,
	// fog_on: bool
	button_states: ButtonStates
}

impl Demo {
	fn new(wrapped_context: WindowedContext<PossiblyCurrent>) -> Demo {
		println!(
			"Pixel format of the window's GL context: {:?}",
			wrapped_context.get_pixel_format()
		);

		gl::load_with(|ptr| wrapped_context.context().get_proc_address(ptr) as *const _);

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
			}
		}
	}
	fn build_display_lists(&mut self) -> DisplayLists {
		let cube = unsafe {
			gl::GenLists(1)
		};
		unsafe {
			const DX: f32 = 0.5;
			const DY: f32 = DX;
			const DZ: f32 = DY;
			gl::NewList( cube, gl::COMPILE );
				gl::Begin( gl::QUADS );
					gl::Color3f( 0., 0., 1. );
					gl::Normal3f( 0., 0.,  1. );
						gl::Vertex3f( -DX, -DY,  DZ );
						gl::Vertex3f(  DX, -DY,  DZ );
						gl::Vertex3f(  DX,  DY,  DZ );
						gl::Vertex3f( -DX,  DY,  DZ );
					gl::Normal3f( 0., 0., -1. );
						gl::TexCoord2f( 0., 0. );
						gl::Vertex3f( -DX, -DY, -DZ );
						gl::TexCoord2f( 0., 1. );
						gl::Vertex3f( -DX,  DY, -DZ );
						gl::TexCoord2f( 1., 1. );
						gl::Vertex3f(  DX,  DY, -DZ );
						gl::TexCoord2f( 1., 0. );
						gl::Vertex3f(  DX, -DY, -DZ );
					gl::Color3f( 1., 0., 0. );
					gl::Normal3f(  1., 0., 0. );
						gl::Vertex3f(  DX, -DY,  DZ );
						gl::Vertex3f(  DX, -DY, -DZ );
						gl::Vertex3f(  DX,  DY, -DZ );
						gl::Vertex3f(  DX,  DY,  DZ );
					gl::Normal3f( -1., 0., 0. );
						gl::Vertex3f( -DX, -DY,  DZ );
						gl::Vertex3f( -DX,  DY,  DZ );
						gl::Vertex3f( -DX,  DY, -DZ );
						gl::Vertex3f( -DX, -DY, -DZ );
					gl::Color3f( 0., 1., 0. );
					gl::Normal3f( 0.,  1., 0. );
						gl::Vertex3f( -DX,  DY,  DZ );
						gl::Vertex3f(  DX,  DY,  DZ );
						gl::Vertex3f(  DX,  DY, -DZ );
						gl::Vertex3f( -DX,  DY, -DZ );
					gl::Normal3f( 0., -1., 0. );
						gl::Vertex3f( -DX, -DY,  DZ );
						gl::Vertex3f( -DX, -DY, -DZ );
						gl::Vertex3f(  DX, -DY, -DZ );
						gl::Vertex3f(  DX, -DY,  DZ );
				gl::End( );
			gl::EndList( );
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
			cube,
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


			// set the fog parameters:
			// if self.fog_on {
			// 	gl::Fogi( gl::FOG_MODE, FOGMODE );
			// 	gl::Fogfv( gl::FOG_COLOR, FOGCOLOR );
			// 	gl::Fogf( gl::FOG_DENSITY, FOGDENSITY );
			// 	gl::Fogf( gl::FOG_START, FOGSTART );
			// 	gl::Fogf( gl::FOG_END, FOGEND );
			// 	gl::Enable( gl::FOG );
			// } else {
			// 	gl::Disable( gl::FOG );
			// }


			// possibly draw the axes:

			// if self.axis_on {
			// 	gl::Color3fv( &Colors[WhichColor][0] );
			// 	gl::CallList( AxesList );
			// }

			// draw the current object:
			if let Some(ref lists) = self.display_lists {
				gl::CallList( lists.cube );
				self.draw_axis();
			}

			/*if( DepthFightingOn != 0 )
			{
				glPushMatrix( );
					glRotatef( 90.,   0., 1., 0. );
					glCallList( BoxList );
				glPopMatrix( );
			}*/


			// draw some gratuitous text that just rotates on top of the scene:

			// glDisable( gl::DEPTH_TEST );
			// glColor3f( 0., 1., 1. );
			// DoRasterString( 0., 1., 0., "Text That Moves" );


			// draw some gratuitous text that is fixed on the screen:
			//
			// the projection matrix is reset to define a scene whose
			// world coordinate system goes from 0-100 in each axis
			//
			// this is called "percent units", and is just a convenience
			//
			// the modelview matrix is reset to identity as we don't
			// want to transform these coordinates

			// glDisable( gl::DEPTH_TEST );
			// glMatrixMode( gl::PROJECTION );
			// glLoadIdentity( );
			// gluOrtho2D( 0., 100.,     0., 100. );
			// glMatrixMode( gl::MODELVIEW );
			// glLoadIdentity( );
			// glColor3f( 1., 1., 1. );
			// DoRasterString( 5., 5., 0., "Text That Doesn't" );

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
				let _diff = now.duration_since(last_inst).as_millis();
				// self.yrot += diff as f32 / 50.0;
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
			self.yrot -= diff_x;
			self.xrot -= diff_y;
		}
	}
	fn scroll_delta(&mut self, diff_y: f32) {
		self.scale += diff_y / 100.0;
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
