#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate conrod;

use conrod::{widget, Colorable, Positionable, Sizeable, Borderable, Labelable, Widget};
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

    let a_button1 = ui.add_widget(-1, gui::new_button(350, 40, 200, 40, "Ok"));
    let a_button2 = ui.add_widget(-1, gui::new_button(350, 100, 200, 40, "Ok"));
    let a_button3 = ui.add_widget(-1, gui::new_button(350, 150, 200, 40, "Ok"));
    let a_button4 = ui.add_widget(-1, gui::new_button(350, 200, 200, 40, "Ok"));
    let main_form = ui.add_widget(-1, gui::new_form(50, 50, 400, 300, "Main Menu"));
    let main_label = ui.add_widget(main_form, gui::new_label(10, 40, "Hello."));
    let main_button = ui.add_widget(main_form, gui::new_button(10, 40, 100, 40, "OK."));
    let a_label = ui.add_widget(main_form, gui::new_label(20, 100, "Hello again."));
    let another_form = ui.add_widget(-1, gui::new_form(300, 300, 300, 300,"Other form"));

	let mut running = true;
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    let mut widgets_collection: 
        HashMap<gui::WidgetHandle, conrod::widget::id::Id> = HashMap::new();
    	
    let mut half_screen_w = WIDTH as f64 / 2.0;
    let mut half_screen_h = HEIGHT as f64 / 2.0;

    use glium::glutin::{Event, ElementState, MouseButton};
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
                for i in 0 .. ui.num_widgets()  {
                    let handle = i as gui::WidgetHandle;
                    let id  = match widgets_collection.get(&handle) {
                        Some(_) => continue,
                        None => widget_generator.next()
                    };

                    widgets_collection.insert(handle, id);    
                }
            }

            let ui = &mut conrod_ui.set_widgets();

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

                        let mut framecolor =  conrod::color::grayscale(0.575);
                        if focus {framecolor = conrod::color::grayscale(0.650);}
    
                        let x = x as f64 - half_screen_w + (w as f64 / 2.0);
                        let y = half_screen_h - y as f64 - (h as f64/ 2.0);

                        widget::Toggle::new(true)
                            .x_y(x, y)
                            .w_h(w as f64, h as f64)
                            .color(framecolor)
                            .border(3.0)
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
