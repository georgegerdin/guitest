#![allow(dead_code)]
#![allow(unused_variables)]
extern crate std;
extern crate glium;
extern crate glium_text;
extern crate cgmath;

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

pub enum Widget {
    Form {      position: Position, 
                size: Size,
                },
    Label {     position: Position,
                text: String
                },
    Button {    position: Position,
                size: Size,
                pressed: bool,
                },
    Textbox {   position: Position,
                size: Size,
                text: String
                },
}

pub enum RenderJob<'a> {
    Shape { vertices: Vec<Vertex>, 
            indices: glium::index::NoIndices,
            },
    Text {  text:   glium_text::TextDisplay<&'a glium_text::FontTexture>, 
            matrix: [[f32; 4]; 4], 
            color: (f32, f32, f32, f32) },
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

fn get_screen_x(i32 position, i32 screen_width) {
    let screen_x = (2.0 * position as f32) / screen_width as f32;
    screen_x - 1.0
}

fn get_screen_y(i32 position, i32 screen_height) {
    let screen_y = (2.0 * position as f32) / screen_height as f32;
    1.0 - screen_y
}

fn get_widget_rect(widget: &Widget) -> Option<Rect> {
    match *widget {
        Widget::Form {ref position, ref size} => Some(new_rect(position, size)),
        Widget::Label{..} => None,
        Widget::Button{ref position, ref size, ..} => Some(new_rect(position, size)),
        Widget::Textbox{ref position, ref size, ..} => Some(new_rect(position, size)),
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

pub struct UI<'a> {
    widgets: Vec<(i32, Widget)>,
    events: Vec<WidgetEvent>,
    screen_rect: Rect,
    font: glium_text::FontTexture,
    text_system: &'a glium_text::TextSystem,
}
use glium::backend::glutin_backend::GlutinFacade;
impl<'a> UI<'a> {
    pub fn new<'b> (screen_width: i32, screen_height: i32, display: &'b GlutinFacade, text_system: &'a glium_text::TextSystem) -> UI<'a> {
        let font_texture = glium_text::FontTexture::new(display, &include_bytes!("font.ttf")[..], 70).unwrap();

        UI {
            widgets: Vec::new(),
            events: Vec::new(),
            screen_rect: Rect {x: 0, y: 0, w: screen_width, h: screen_height},
            font: font_texture,
            text_system: text_system,
        }
    }
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn add_widget(&mut self, parent: i32, w: Widget) -> i32 {
        let index = self.widgets.len();
        let maxi32 = std::i32::MAX as usize;

        if index >= maxi32 {
            panic!("Too many widgets");
        }

        let safe_index = index as i32;

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
        let mut rects: Vec<Rect>;
        rects = Vec::new();
        rects.push(self.screen_rect);
        
        let mut render_jobs: Vec<RenderJob>;
        render_jobs = Vec::new();

        for widget in &self.widgets {
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

                    let vertex1 = Vertex { position: [rect.x as f32 / self.screen_rect.w as f32, rect.y as f32 / self.screen_rect.h as f32] };
                    let vertex2 = Vertex { position: [(rect.x as f32 + rect.w as f32) / self.screen_rect.w as f32,  rect.y as f32 / self.screen_rect.h as f32] };
                    let vertex3 = Vertex { position: [rect.x as f32 / self.screen_rect.w as f32, (rect.y as f32 + rect.h as f32) / self.screen_rect.h as f32] };
                    let vertex4 = Vertex { position: [(rect.x as f32 + rect.w as f32) / self.screen_rect.w as f32, (rect.y as f32 + rect.h as f32) / self.screen_rect.h as f32] };

                    let shape = vec![vertex1, vertex2, vertex3, vertex2, vertex3, vertex4];

                    println!("{}, {}", vertex1.position[0], vertex1.position[1]);
                    println!("{}, {}", vertex2.position[0], vertex2.position[1]);
                    println!("{}, {}", vertex3.position[0], vertex3.position[1]);
                    println!("{}, {}", vertex4.position[0], vertex4.position[1]);

                    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let newJob = RenderJob::Shape {vertices: shape, indices: index_buffer};

                    render_jobs.push( newJob );
                }
                _ => {
                    match widget.1 {
                        Widget::Label{ref position, ref text} => {
                            let parent_rect = rects.last().unwrap().clone();
                            let rendered_text = glium_text::TextDisplay::new(
                                self.text_system,
                                &self.font,
                                text.as_str());
                            let text_width = rendered_text.get_width();
                            let matrix:[[f32; 4]; 4] = cgmath::Matrix4::new(
                                0.2 / text_width, 0.0, 0.0, 0.0,
                                0.0, 0.2 * (800 as f32) / (600 as f32) / text_width, 0.0, 0.0,
                                0.0, 0.0, 1.0, 0.0,
                                get_screen_x(position.x + parent_rect.x, self.screen_rect.w), -1.0, 0.0, 1.0f32,
                            ).into();
                            let color = (1.0, 1.0, 1.0, 1.0);
                            let new_job = RenderJob::Text {text: rendered_text, 
                                                            matrix: matrix, 
                                                            color: color};
                            render_jobs.push(new_job);
                        }
                        _ => ()
                    }
                }
            }
        }
        render_jobs
    }
}