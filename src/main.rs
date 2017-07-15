#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate conrod;

use conrod::{widget, Colorable, Positionable, Sizeable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

extern crate cgmath;

mod gui;
use gui::UI;
use std::collections::HashMap;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .build_glium()
        .unwrap();
     
    
    let mut ui = UI::new(800, 600);
    let mut conrod_ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/Dogma/Dogma.ttf");
    conrod_ui.fonts.insert_from_file(FONT_PATH).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let main_form = ui.add_widget(-1, gui::new_form(50, 50, 400, 300, "Main Menu"));
    let main_label = ui.add_widget(main_form, gui::new_label(10, 40, "Hello."));
    let main_button = ui.add_widget(main_form, gui::new_button(10, 40, 100, 40, "OK."));
    let a_label = ui.add_widget(main_form, gui::new_label(20, 100, "Hello again."));

	let mut running = true;
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let mut widgets_collection: 
        HashMap<usize, conrod::widget::id::Id> = HashMap::new();
    	
    use glium::glutin::{Event, ElementState, MouseButton};

    // Generate the widget identifiers.
    widget_ids!(struct Ids { text });
    let ids = Ids::new(conrod_ui.widget_id_generator());

    while running {
        ui.clear_events();

        let mut target = display.draw();
        
        for ev in display.poll_events() {
            match ev {
                Event::Closed => running = false,
                Event::MouseMoved(x, y) => {
                    ui.mousemove(mouse_x, mouse_y, x, y);
                    mouse_x = x; mouse_y = y;
                    },
                Event::MouseInput(ElementState::Pressed, button) =>  {
                    match button {
                        MouseButton::Left => ui.mousedown(),
                        _ => ()    
                    }
                },
                Event::MouseInput(ElementState::Released, button) => {
                    match button {
                        MouseButton::Left => ui.mouseup(),
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        let render_jobs = ui.render();

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        
        {
            {
                let mut widget_generator = conrod_ui.widget_id_generator();
                for i in 0 .. ui.num_widgets() {
                    let id  = match widgets_collection.get(&i) {
                        Some(_) => continue,
                        None => widget_generator.next()
                    };

                    widgets_collection.insert(i, id);    
                }
            }

            let ui = &mut conrod_ui.set_widgets();

            /*let mpostext = format!("Form position: ({}, {})", x, y);

                        widget::Text::new(&mpostext)
                            .top_left_of(ui.window)
                            .color(conrod::color::WHITE)
                            .set(ids.text, ui);
*/
            for render_job in &render_jobs {
                let i: conrod::widget::id::Id;

                macro_rules! find_widget {
                    ($collection:ident, $index:ident, $dest:ident) => {
                        match $collection.get(&$index) {
                            Some(k) => $dest = *k,
                            None => unreachable!()
                        }
                    }
                }

                match *render_job {
                    gui::RenderJob::Nul => (),
                    gui::RenderJob::Form { index, focus, x, y, w, h, ref title} => {
                        find_widget!(widgets_collection, index, i);

                        let mut bgcolor = conrod::color::LIGHT_CHARCOAL;
                        if focus {bgcolor = conrod::color::GRAY;}
    
                        widget::Rectangle::outline([w as f64, h as f64])
                            .x_y(x as f64 - 400.0 + (w as f64 / 2.0), 300.0 - y as f64 - (h as f64/ 2.0))
                            .color(bgcolor)
                            .set(i, ui);

                    }
                    gui::RenderJob::Button {index, pressed, focus, x, y, w, h, ref text } => {
                        find_widget!(widgets_collection, index, i);

                        let mut label_color = conrod::color::CHARCOAL;;
                        let mut bgcolor = conrod::color::DARK_BLUE;
                        if pressed {label_color = conrod::color::LIGHT_CHARCOAL; }
                        else {
                            if focus {bgcolor = conrod::color::GRAY;}
                            else {bgcolor = conrod::color::LIGHT_CHARCOAL;}
                        }    
                    

                        widget::Toggle::new(!pressed)
                            .top_left_of(ui.window)
                            .x_y(x as f64 - 400.0 + (w as f64 / 2.0), 300.0 - y as f64 - (h as f64/ 2.0))
                            .w_h(w as f64, h as f64)
                            .color(bgcolor)
                            .label(&text)
                            .label_color(label_color)
                            .set(i, ui);
                    },
                    gui::RenderJob::Label {index, x, y, ref text} => {
                        find_widget!(widgets_collection, index, i);

                        let fx = x as f64;
                        let fy = y as f64;

                        widget::Text::new(&text)
                            .top_left_of(ui.window)
                            .x_y(fx - 400.0, 300.0 - fy)
                            .w_h(0.1, 0.1)
                            .no_line_wrap()
                            .font_size(18)
                            .color(conrod::color::WHITE)
                            .set(i, ui);    
                    }
                }
            }
        }
        
        renderer.fill(&display, conrod_ui.draw(), &image_map);
        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();

        
    }
}
