extern crate gl;
extern crate glfw;

pub mod shader;
pub mod program;
pub mod vbo;
pub mod vao;
pub mod mesh;

use glfw::{Action, Context, Key};
use mesh::Mesh;
use program::Program;
use shader::Shader;
use vao::Vao;
use vbo::Vbo;

use crate::shader::shader_from_source;

fn create_vert_shader() -> anyhow::Result<Shader> {
    shader_from_source! {
        gl::VERTEX_SHADER,
        r#"#version 330 core
        layout (location = 0) in vec3 vpos;
        layout (location = 1) in vec3 vcol;

        out vec4 vertexColor;
        void main() {
            gl_Position = vec4(vpos, 1.0);
            vertexColor = vec4(vcol, 1.0);
        }"#
    }
}

fn create_frag_shader() -> anyhow::Result<Shader> {
    shader_from_source! {
        gl::FRAGMENT_SHADER,
        r#"#version 330 core
        in vec4 vertexColor;
        out vec4 FragColor;
        void main() {
           FragColor = vertexColor;
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

fn create_mesh() -> anyhow::Result<Mesh> {
    let mut vao = Vao::create();
    vao.bind();

    {
        let vbo = Vbo::create();
        vbo.bind();
        let vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.0, // left
             0.5, -0.5, 0.0, // right
             0.0,  0.5, 0.0  // top
        ];
        vbo.upload_data(vertices);
        vbo.vertex_attrib(0, 3, gl::FLOAT);
        vbo.unbind();
        vao.add_vbo(vbo);
    }

    {
        let vbo = Vbo::create();
        vbo.bind();
        let vertices: Vec<f32> = vec![
            1.0, 0.0, 0.0, // left
            0.0, 1.0, 0.0, // right
            0.0, 0.0, 1.0  // top
        ];
        vbo.upload_data(vertices);
        vbo.vertex_attrib(1, 3, gl::FLOAT);
        vbo.unbind();
        vao.add_vbo(vbo);
    }

    vao.unbind();

    let program = create_program()?;

    Ok(Mesh::new(vao, program, 3))
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

    let mesh = create_mesh()?;

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        mesh.draw();

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
