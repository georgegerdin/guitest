#![allow(dead_code)]
#![allow(unused_variables)]
extern crate std;
extern crate glium;
extern crate cgmath;

use text;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Size {
    w: i32,
    h: i32,
}

#[derive(Copy, Clone)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Copy, Clone)]
struct FloatRect {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, w: {}, h: {}", self. x, self.y, self.w, self.h)
    }
}

pub enum Widget {
    Form {      position: Position, 
                size: Size,
                },
    Label {     position: Position,
                text: String,
                rendered_text: i32,
                },
    Button {    position: Position,
                size: Size,
                pressed: bool,
                text: String,
                rendered_text: i32,
                },
    Textbox {   position: Position,
                size: Size,
                text: String,
                },
}

pub enum RenderJob {
    Shape { vertices: Vec<Vertex>, 
            indices: glium::index::NoIndices,
            color: (f32, f32, f32, f32),
            },
    Text {  text:   i32, 
            matrix: [[f32; 4]; 4], 
            color: (f32, f32, f32, f32) 
            },
}

pub enum WidgetEvent {
    ButtonClicked(i32),
}

pub fn new_form(ix: i32, iy: i32, iw: i32, ih: i32) -> Widget {
    Widget::Form {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
    }
}

pub fn new_label(ix: i32, iy: i32, itext: &str) -> Widget {
    Widget::Label {
        position: Position {x: ix, y: iy},
        text: itext.to_string(),
        rendered_text: -1,
    }
}

pub fn new_button(ix: i32, iy: i32, iw: i32, ih: i32, itext: &str) -> Widget {
    Widget::Button {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
        pressed: false,
        text: itext.to_string(),
        rendered_text: -1,
    }
}

fn new_rect(pos: &Position, size: &Size) -> Rect {
    Rect {
        x: pos.x,
        y: pos.y,
        w: size.w,
        h: size.h,
    }
} 

fn get_screen_x(position: i32, screen_width: i32) -> f32 {
    let screen_x = (2.0 * position as f32) / screen_width as f32;
    screen_x - 1.0
}

fn get_screen_y(position: i32, screen_height: i32) -> f32 {
    let screen_y = (2.0 * position as f32) / screen_height as f32;
    1.0 - screen_y
}

fn to_screen_rect(irect: Rect, screen_width: i32, screen_height: i32) -> FloatRect {
    FloatRect {
        x1: get_screen_x(irect.x, screen_width),
        y1: get_screen_y(irect.y, screen_height),
        x2: get_screen_x(irect.x + irect.w - 1, screen_width),
        y2: get_screen_y(irect.y + irect.h - 1, screen_height),
    }
}

fn get_widget_rect(widget: &Widget) -> Option<Rect> {
    match *widget {
        Widget::Form {ref position, ref size} => Some(new_rect(position, size)),
        Widget::Label{..} => None,
        Widget::Button{ref position, ref size, ..} => Some(new_rect(position, size)),
        Widget::Textbox{ref position, ref size, ..} => Some(new_rect(position, size)),
    }
}

fn get_widget_position(widget: &Widget) -> (i32, i32) {
    match *widget {
        Widget::Form {ref position, ..} => (position.x, position.y),
        Widget::Label{ref position, ..} => (position.x, position.y),
        Widget::Button{ref position, ..} => (position.x, position.y),
        Widget::Textbox{ref position, ..} => (position.x, position.y),
    }
}

fn widget_event_mousedown(widget: &mut Widget) {
    match *widget {
        Widget::Button{ref mut pressed, ..} => *pressed = true,
        _ => ()
    }

}

fn inside_rect(rect: Rect, x: i32, y: i32) -> bool {
    if x >= rect.x && x < rect.x + rect.w &&
       y >= rect.y && y < rect.y + rect.h {
           true
       }
    else {
        false
    }
}

fn render_form(rect: FloatRect) -> RenderJob {
    let vertex1 = Vertex { position: [rect.x1, rect.y1] };
    let vertex2 = Vertex { position: [rect.x2, rect.y1] };
    let vertex3 = Vertex { position: [rect.x1, rect.y2] };             
    let vertex4 = Vertex { position: [rect.x2, rect.y2] };

    let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex3, vertex4];

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    RenderJob::Shape{vertices: shape, indices: index_buffer, color: (0.0f32, 1.0f32, 1.0f32, 1.0f32) }
}

fn render_button_background(rect: FloatRect) -> RenderJob {
    let vertex1 = Vertex { position: [rect.x1, rect.y1] };
    let vertex2 = Vertex { position: [rect.x2, rect.y1] };
    let vertex3 = Vertex { position: [rect.x1, rect.y2] };             
    let vertex4 = Vertex { position: [rect.x2, rect.y2] };

    let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex3, vertex4];

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    RenderJob::Shape{vertices: shape, indices: index_buffer, color: (1.0f32, 0.0f32, 1.0f32, 1.0f32) }
}

pub struct UI<'a> {
    widgets: Vec<(i32, Widget)>,
    events: Vec<WidgetEvent>,
    screen_rect: Rect,
    font: i32,
    text_system: &'a text::TextSystem,
}
use glium::backend::glutin_backend::GlutinFacade;
impl<'a> UI<'a> {
    pub fn new<'b> (screen_width: i32, screen_height: i32, display: &'b GlutinFacade, text_system: &'a text::TextSystem) -> UI<'a> {
        let font_texture = text_system.create_font("Standard", &include_bytes!("font.ttf")[..]).unwrap();

        UI {
            widgets: Vec::new(),
            events: Vec::new(),
            screen_rect: Rect {x: 0, y: 0, w: screen_width, h: screen_height},
            font: font_texture,
            text_system: text_system,
            rendered_texts: Vec::new(),
        }
    }
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn add_text_to_render(&mut self, text: &String) -> i32 {
        let index = self.rendered_texts.len();
        let maxi32 = std::i32::MAX as usize;

        if index >= maxi32 {
            panic!("Too many text renderings");
        }

        let safe_index = index as i32;

        self.rendered_texts.push(std::rc::Rc::new(text::TextDisplay::new(self.text_system, self.font.clone(), text.as_str())));
        
        safe_index
    }

    pub fn add_widget(&mut self, parent: i32, mut w: Widget) -> i32 {
        let index = self.widgets.len();
        let maxi32 = std::i32::MAX as usize;

        if index >= maxi32 {
            panic!("Too many widgets");
        }

        let safe_index = index as i32;

        match w {
            Widget::Label{ref position, ref text, ref mut rendered_text} => {
                *rendered_text = self.add_text_to_render(text);
            }
            Widget::Button{ref text, ref mut rendered_text, ..} => {
                *rendered_text = self.add_text_to_render(text);
            }
            _ => ()
        }

        self.widgets.push((parent, w));

        safe_index
    }

    pub fn mousedown(&mut self, mx: i32, my: i32) {
        let mut current_parent = -1;
        let mut rects: Vec<Rect>;
        rects = Vec::new();
        rects.push(self.screen_rect);
        
        for widget in &mut self.widgets {
            let mut rect = get_widget_rect(&widget.1);
                   
            match rect {
                Some(mut rect) => {
                    let parent_rect = rects.last().unwrap().clone();

                    rect.x+= parent_rect.x;
                    rect.y+= parent_rect.y;

                    if rect.x + rect.w > parent_rect.w {
                        rect.w = parent_rect.w - rect.x;
                    }

                    if rect.y + rect.h > parent_rect.h {
                        rect.h = parent_rect.h - rect.y;
                    }

                    if inside_rect(rect, mx, my) {
                        widget_event_mousedown(&mut widget.1);
                    }
                }
                None => ()    
            }
            
        }
    }



    pub fn render(&self) -> Vec<RenderJob> {
        let mut render_jobs: Vec<RenderJob>;
        render_jobs = Vec::new();

        let mut last_parent = -1;
        let mut last_rect = self.screen_rect.clone();

        let mut rects: Vec<(i32, Rect)>;
        rects = Vec::new();
        rects.push( (last_parent, last_rect) );

        let mut index = 0; 
        for widget in &self.widgets {
            //println!("For for widget in index={}, last_parent={}, last_rect={}", index, last_parent, last_rect );

            //If the new widget is a child widget of the last widget
            if last_parent < widget.0  {
                rects.push( (widget.0, last_rect) );
                last_parent = widget.0;
            //    println!("This is a child widget last_parent={}, last_rect={}", last_parent, last_rect);
            }
            //If the new widget is higher in the tree
            else if last_parent > widget.0  {
                while last_parent != widget.0  {
                    println!("last_parent {}, widget.0 {}", last_parent, widget.0);
                    rects.pop();
                    let (lp, lr) = rects.last().unwrap().clone();
                    last_parent = lp;
                    last_rect = lr;
                }
            //    println!("Widget higher in the tree last_parent={}, last_rect={}", last_parent, last_rect);
            }
            //The widget has the same parent as the previous one
            else {
                let (last_parent, last_rect) = rects.last().unwrap().clone();
            //    println!("Widget is sibling last_parent={}, last_rect={}.", last_parent, last_rect);
            }

            let rect = get_widget_rect(&widget.1);
            
            let final_rect = match rect {
                Some(mut rect) => {
                    let parent_rect = last_rect;
                    
                    let final_x = rect.x + parent_rect.x;
                    let final_y = rect.y + parent_rect.y;
                    let mut final_w = rect.w;
                    let mut final_h = rect.h;

                    if rect.x + rect.w > parent_rect.w {
                        final_w = parent_rect.w - final_x;
                    }

                    if rect.y + rect.h > parent_rect.h {
                        final_h = parent_rect.h - final_y;
                    }
                    Rect {
                        x: final_x,
                        y: final_y,
                        w: final_w,
                        h: final_h
                    }
                }
                None => {
                    let parent_rect = last_rect;
                    let (ix, iy) = get_widget_position(&widget.1);

                    Rect {
                        x: ix + parent_rect.x, 
                        y: iy + parent_rect.y, 
                        w: parent_rect.w - ix, 
                        h: parent_rect.h - iy 
                    }
                }
            };

            let screen_rect = to_screen_rect(final_rect, 800, 600);

            match widget.1 {
                Widget::Form{..} => {
                    render_jobs.push(render_form(screen_rect));
                }
                Widget::Label{ref position, ref text, rendered_text} => {
                    
                    render_jobs.push(self.render_text(final_rect.x, final_rect.y, rendered_text));
                }
                Widget::Button{ref text, rendered_text, ..} => {
                    render_jobs.push(render_button_background(screen_rect));
                    render_jobs.push(self.render_text(final_rect.x, final_rect.y, rendered_text));
                }
                _ => ()
            }

            last_rect = final_rect;

            index+= 1;
        }
        render_jobs
    }

    pub fn render_text(&self, x: i32, y: i32, rendered_text: i32) -> RenderJob {
        if rendered_text < 0 {
            panic!("Text index can't be negative");
        }

        let rendered_text = self.rendered_texts[rendered_text as usize].clone();
        let text_width = rendered_text.get_width();
        //println!("Text width: {}", text_width);
        let matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
            2.0 / text_width, 0.0, 0.0, 0.0,
            0.0, 2.0 * (800.0) / (600.0) / text_width, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            get_screen_x(x, self.screen_rect.w), get_screen_y(y, self.screen_rect.h), 0.0, 1.0f32,
        ).into();
        let color = (1.0, 1.0, 1.0, 1.0);
        RenderJob::Text {
            text: rendered_text, 
            matrix: matrix, 
            color: color
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_rendering() {
        use glium::{DisplayBuild, Surface};
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(800, 600)
            .build_glium()
            .unwrap();
        let text_system = text::TextSystem::new(&display);

        let mut ui = UI::new(800, 600, &display, &text_system);
        let main_form = ui.add_widget(-1, new_form(50, 50, 400, 300));
        let main_label = ui.add_widget(main_form, new_label(10, 10, "Hello."));
        let main_button = ui.add_widget(main_form, new_button(10, 40, 100, 40, "OK."));
        let second_form = ui.add_widget(-1, new_form(100, 100, 200, 200));

        assert!(ui.render().len() == 4);
    }
}