#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate glium_text;

mod gui;
use gui::UI;
use gui::Vertex;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();
     let text_system = glium_text::TextSystem::new(&display);


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	
	//Create user interface
    let mut ui = UI::new(800, 600, &display, &text_system);
    let main_form = ui.add_widget(-1, gui::new_form(50, 50, 400, 300));
    let main_label = ui.add_widget(main_form, gui::new_label(10, 10, "Hello."));

	let mut running = true;
    let mut mouse_x = 0;
    let mut mouse_y = 0;
	
    let font = glium_text::FontTexture::new(&display, &include_bytes!("font.ttf")[..], 70).unwrap();
    let text = glium_text::TextDisplay::new(&text_system, &font, "Hello world!");
    let text_width = text.get_width();
    println!("Text width: {:?}", text_width);

    use glium::glutin::{Event, ElementState, MouseButton};
    while running {
        ui.clear_events();

        let mut target = display.draw();
        
        for ev in display.poll_events() {
            match ev {
                Event::Closed => running = false,
                Event::MouseMoved(x, y) => {mouse_x = x; mouse_y = y;},
                Event::MouseInput(ElementState::Pressed, button) =>  {
                    match button {
                        MouseButton::Left => ui.mousedown(mouse_x, mouse_y),
                        _ => ()    
                    }
                },
                _ => ()
            }
        }

        let matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            2.0 / text_width, 0.0, 0.0, 0.0,
            0.0, 2.0 * (800 as f32) / (600 as f32) / text_width, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 1.0f32,
        ).into();

        let render_jobs = ui.render();

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        for render_job in &render_jobs {
            match *render_job {
                gui::RenderJob::Shape {ref vertices, ref indices } => {
                    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();       
                    target.draw(&vertex_buffer, indices, &program, &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
                },
                gui::RenderJob::Text {ref text, ref matrix, ref color} => {
                    glium_text::draw(&text, &text_system, &mut target, *matrix, *color);
                }
            }
        }

        glium_text::draw(&text, &text_system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));


        target.finish().unwrap();

        
    }
}
