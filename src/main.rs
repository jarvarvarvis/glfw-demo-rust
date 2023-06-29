extern crate gl;
extern crate glfw;

pub mod shader;
pub mod program;
pub mod vbo;
pub mod vao;
pub mod mesh;

use glfw::{Action, Context, Key};
use program::Program;
use shader::Shader;

use crate::shader::shader_from_source;

fn create_vert_shader() -> anyhow::Result<Shader> {
    shader_from_source! {
        gl::VERTEX_SHADER,
        r#"#version 330 core
        layout (location = 0) in vec3 vpos;
        void main() {
            gl_Position = vec4(vpos, 1.0);
        }"#
    }
}

fn create_frag_shader() -> anyhow::Result<Shader> {
    shader_from_source! {
        gl::FRAGMENT_SHADER,
        r#"#version 330 core
        out vec4 FragColor;
        void main() {
           FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }"#
    }
}

fn create_program() -> anyhow::Result<Program> {
    Program::from_shaders(
        vec![ 
            create_vert_shader()?, 
            create_frag_shader()? 
        ])
}

fn main() -> anyhow::Result<()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(300, 300, "GLFW Demo", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let program = create_program()?;

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
