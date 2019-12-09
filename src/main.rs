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

use cgmath::{Deg, Matrix, Matrix4, SquareMatrix, PerspectiveFov, Point3, Vector3};

use rand::prelude::*;

use std::{
	cell::RefCell, 
	rc::Rc, 
	time::Duration,
	mem::{
		zeroed,
		size_of,
		transmute
	},
	ffi::c_void
};

mod gl;

// mod mesh;
// use mesh::*;
// mod light;
// use light::*;
// mod material;
// use material::*;
mod demo;
use demo::*;

fn print_program_data(program: u32) {
	unsafe {
		let mut count = zeroed();
		gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut count);
		println!("Active Attributes: {}", count);
		for i in 0..count {
			const BUFFER_SIZE: usize = 16;
			#[derive(Debug)]
			struct AttributeInfo {
				name: [i8; BUFFER_SIZE],
				length: i32,
				size: i32,
				kind: u32
			}
			let mut attr = AttributeInfo {
				name: zeroed(),
				length: 0,
				size: 0,
				kind: 0
			};
			gl::GetActiveAttrib(
				program, i as u32, BUFFER_SIZE as i32, 
				&mut attr.length, 
				&mut attr.size, 
				&mut attr.kind, 
				&mut attr.name[0] as *mut i8
			);

			println!("Attribute {:?}", attr);
			println!("Attribute Name: {}", std::str::from_utf8(
				transmute::<&[i8], &[u8]>(&attr.name)
			).unwrap());
			println!("Attribute Type: {}", match(attr.kind) {
				gl::FLOAT => "float",
				gl::FLOAT_VEC3 => "vec3 float",
				_ => "Unknown type"
			});
		}
		let mut count = zeroed();
		gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut count);
		println!("Active Uniforms: {}", count);
		for i in 0..count {
			const BUFFER_SIZE: usize = 16;
			#[derive(Debug)]
			struct UniformInfo {
				name: [i8; BUFFER_SIZE],
				length: i32,
				size: i32,
				kind: u32
			}
			let mut attr = UniformInfo {
				name: zeroed(),
				length: 0,
				size: 0,
				kind: 0
			};
			gl::GetActiveUniform(
				program, i as u32, BUFFER_SIZE as i32, 
				&mut attr.length, 
				&mut attr.size, 
				&mut attr.kind, 
				&mut attr.name[0] as *mut i8
			);

			println!("Uniform {:?}", attr);
			println!("Uniform Name: {}", std::str::from_utf8(
				transmute::<&[i8], &[u8]>(&attr.name)
			).unwrap());
			println!("Uniform Type: {}", match(attr.kind) {
				gl::FLOAT => "float",
				gl::FLOAT_MAT4 => "mat4 float",
				gl::FLOAT_VEC3 => "vec3 float",
				_ => "Unknown type"
			});
		}
	}
}

fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_title("OpenGL / GLUT Sample -- Evan Brass");

	let windowed_context = ContextBuilder::new()
		.with_gl(GlRequest::Latest)
		.with_vsync(true)
		.build_windowed(wb, &event_loop)
		.expect("Unable to build windowed Context");

	const CHUNK_WIDTH: f32 = 10.0;
	const CHUNK_HEIGHT: f32 = 10.0;
	const CHUNK_DEPTH: f32 = 10.0;
	const CHUNK_SIZE_WIDTH: usize = 20;
	const CHUNK_SIZE_HEIGHT: usize = 60;
	const CHUNK_SIZE_DEPTH: usize = 20;
	const NUM_ITEMS: usize = CHUNK_SIZE_WIDTH * CHUNK_SIZE_HEIGHT * CHUNK_SIZE_DEPTH;
	#[derive(Clone, Copy, Debug)]
	#[repr(packed)]
	struct ChunkItem {
		pub size: f32,
		pub color: (f32, f32, f32)
	}
	#[derive(Clone)]
	struct Chunk {
		pub program: u32,
		pub buffer_id: u32,
		pub vao: u32,
		pub x: f32,
		pub y: f32,
		pub z: f32
	}
	struct ChunkItemAccessor {
		pub index: usize,
		buffer_id: u32
	}
	impl ChunkItemAccessor {
		fn get(&self) -> ChunkItem {
			let mut ret: ChunkItem = unsafe { zeroed() };
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
				gl::GetBufferSubData(
					gl::ARRAY_BUFFER,
					(self.index * size_of::<ChunkItem>()) as isize,
					size_of::<ChunkItem>() as isize,
					transmute::<&mut ChunkItem, &mut c_void>(&mut ret)
				);
				gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			}
			ret
		}
		fn set(&self, new_value: ChunkItem) {
			unsafe {
				gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
				gl::BufferSubData(
					gl::ARRAY_BUFFER,
					(self.index * size_of::<ChunkItem>()) as isize,
					size_of::<ChunkItem>() as isize,
					transmute::<&ChunkItem, &c_void>(&new_value)
				);
				gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			}
		}
	}

	// TODO: Implement array access for the chunk that fetches / updates GPU memory:
	//  - Update: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml
	//  - Fetch: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml
	static mut chunk_shader: u32 = 0;
	impl Chunk {
		unsafe fn build_chunk_shader() {
			unsafe fn add_shader(program: u32, stage: u32, source: &[u8]) {
				let id = gl::CreateShader(stage);
				gl::ShaderSource(
					id,
					1,
					[source.as_ptr() as *const _].as_ptr(),
					[source.len() as i32].as_ptr(),
				);
				gl::CompileShader(id);
				gl::AttachShader(program, id);
			}
			// Create the Chunk Shader
			chunk_shader = gl::CreateProgram();

			// Vertex Shader:
			add_shader(chunk_shader, gl::VERTEX_SHADER, include_bytes!("shaders/voxel.glslv"));
			// Geometry Shader:
			add_shader(chunk_shader, gl::GEOMETRY_SHADER, include_bytes!("shaders/voxel.glslg"));
			// Fragment Shader:
			add_shader(chunk_shader, gl::FRAGMENT_SHADER, include_bytes!("shaders/voxel.glslf"));

			gl::LinkProgram(chunk_shader);

			// DEBUG:
			print_program_data(chunk_shader);
		}
		fn new(x: f32, y: f32, z: f32) -> Chunk {
			let mut ret = Chunk {
				x, y, z,
				program: unsafe{ zeroed() },
				buffer_id: unsafe { zeroed() },
				vao: unsafe{ zeroed() }
			};
			unsafe {
				// 1) Create the buffer: Used for storing chunk_items (read and write)
				gl::GenBuffers(1, &mut ret.buffer_id);
				// 1.a) Allocate the size of the buffer that we need without filling it with any data:
				gl::BindBuffer(gl::ARRAY_BUFFER, ret.buffer_id);
				gl::BufferData(
					gl::ARRAY_BUFFER, 
					(size_of::<ChunkItem>() * NUM_ITEMS) as isize, 
					0 as *const c_void, 
					gl::DYNAMIC_DRAW
				);

				// 2) Create the vao: Used for drawing.
				gl::GenVertexArrays(1, &mut ret.vao);
				gl::BindVertexArray(ret.vao);

				if (chunk_shader == 0) {
					unsafe { 
						Chunk::build_chunk_shader(); 
					}
				}

				// 3) Bind shader attributes to the VAO using a temporary ChunkItem to get the offsets
				let temp = ChunkItem { size: 1.0, color: (1.0, 1.0, 1.0) };
				
				// 3.a) Size
				let location = gl::GetAttribLocation(chunk_shader, b"size\0".as_ptr() as *const _);
				let offset = &temp.size as *const _ as usize - &temp as *const _ as usize;
				let size = size_of::<ChunkItem>();
				if (location != -1) {
					gl::VertexAttribPointer(
						location as u32, // Specified  in vert shader
						1,
						gl::FLOAT,
						gl::FALSE,
						size as i32,
						// Measure the distance to the start of the struct field and use that as the offset:
						offset as *const c_void,
					);
					gl::EnableVertexAttribArray(location as u32);
				} else {
					println!("Attribute size didn't appear in the shader program.");
				}
				// 3.b) Color
				let location = gl::GetAttribLocation(chunk_shader, "color\0".as_ptr() as *const _);
				let offset = &temp.color as *const _ as usize - &temp as *const _ as usize;
				let size = size_of::<ChunkItem>();
				if (location != -1) {
					gl::VertexAttribPointer(
						location as u32,
						3,
						gl::FLOAT,
						gl::FALSE,
						size as i32,
						// Measure the distance to the start of the struct field and use that as the offset:
						offset as *const c_void,
					);
					gl::EnableVertexAttribArray(location as u32);
				} else {
					println!("Attribute color didn't appear in the shader program.");
				}
			}
			ret
		}
		fn access(&self, x: usize, y: usize, z: usize) -> ChunkItemAccessor {
			ChunkItemAccessor {
				index: z + CHUNK_SIZE_DEPTH * y + CHUNK_SIZE_DEPTH * CHUNK_SIZE_HEIGHT * x,
				buffer_id: self.buffer_id
			}
		}
		fn access_index(&self, index: usize) -> ChunkItemAccessor {
			ChunkItemAccessor {
				index,
				buffer_id: self.buffer_id
			}
		}

		fn draw(&self) {
			unsafe {
				gl::Enable(gl::CULL_FACE);
				
				// Bind the program
				gl::UseProgram(chunk_shader);
				gl::BindVertexArray(self.vao);
				// Move the chunk in model space + TODO: apply the scale from 0-1 -> 0-CHUNK_SIZE
				let location = gl::GetUniformLocation(chunk_shader, b"chunk_transform\0".as_ptr() as *const _);
				if (location != -1) {
					let chunk_transform: Matrix4<f32> = Matrix4::from_translation(
						Vector3::new(self.x, self.y, self.z)
					);
					gl::UniformMatrix4fv(
						location,
						1,
						gl::FALSE,
						chunk_transform.as_ptr()
					);
				}
				gl::DrawArrays(gl::POINTS, 0, NUM_ITEMS as i32);

				// So... I like having the uniforms defined in demo and am too lazy to move them but the shader has to be active for it to assign the uniforms properly... and since I only have a single shader... I'm just going to constantly use the program and then it will always be active so I don't have to move it around.
			}
		}
	}
	
	let mut demo = Demo::new(
		unsafe { windowed_context.make_current().unwrap() },
		// All the scene items
		vec![&mut || {
			struct VoxelProject {
				chunks: Vec<Chunk>
			}
			impl SceneItem for VoxelProject {
				fn anim(&mut self, update: Duration) {
					// Provide updates
					for i in 0..NUM_ITEMS {
						let accessor = self.chunks[0].access_index(i);
						let mut rng = thread_rng();
						let mut item = accessor.get();
						let change_factor = 0.5 * update.as_secs_f32(); // 2.0 max change per second
						item.color.0 = (item.color.0 + rng.gen_range(-change_factor, change_factor)).max(0.0).min(1.0);
						item.color.1 = (item.color.1 + rng.gen_range(-change_factor, change_factor)).max(0.0).min(1.0);
						item.color.2 = (item.color.2 + rng.gen_range(-change_factor, change_factor)).max(0.0).min(1.0);
						item.size = (item.size + rng.gen_range(-change_factor, change_factor)).max(0.0).min(1.0);
						accessor.set(item);
					}
				}
				fn draw(&self) {
					for chunk in self.chunks.iter() {
						chunk.draw();
					}
				}
			}

			let mut chunk = Chunk::new(
				-(CHUNK_SIZE_WIDTH as f32 / 2.0), 
				-(CHUNK_SIZE_HEIGHT as f32 / 2.0), 
				-(CHUNK_SIZE_DEPTH as f32 / 2.0)
			);
			for i in 0..NUM_ITEMS {
				let accessor = chunk.access_index(i);
				let mut rng = thread_rng();
				accessor.set(ChunkItem {
					size: rng.gen_range(0.0, 1.0),
					color: (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
				});
			}

			Box::new(VoxelProject {
				chunks: vec![chunk]
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
