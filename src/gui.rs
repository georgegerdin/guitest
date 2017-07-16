#![allow(dead_code)]
#![allow(unused_variables)]
extern crate std;
extern crate cgmath;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

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

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, w: {}, h: {}", self. x, self.y, self.w, self.h)
    }
}

pub enum Widget {
    Form {      position: Position, 
                size: Size,
                title: String
                },
    Label {     position: Position,
                text: String,
                },
    Button {    position: Position,
                size: Size,
                pressed: bool,
                text: String
                },
    Textbox {   position: Position,
                size: Size,
                text: String,
                },
}

pub enum RenderJob {
    Nul,

    Form {  index:  WidgetHandle,
            focus:  bool,
            x:      i32,
            y:      i32,
            w:      i32,
            h:      i32,
            title:  String
    },

    Button { index: WidgetHandle,
             pressed: bool,
             focus: bool,
             x:     i32,
             y:     i32,
             w:     i32,
             h:     i32,
             text:  String
    },

    Label { 
            index:  WidgetHandle, 
            x:      i32,
            y:      i32,
            text:   String, 
    },

}

pub enum WidgetEvent {
    ButtonClicked(i32),
}

pub fn new_form(ix: i32, iy: i32, iw: i32, ih: i32, title: &str) -> Widget {
    Widget::Form {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
        title: title.to_owned()
    }
}

pub fn new_label(ix: i32, iy: i32, itext: &str) -> Widget {
    Widget::Label {
        position: Position {x: ix, y: iy},
        text: itext.to_string()
    }
}

pub fn new_button(ix: i32, iy: i32, iw: i32, ih: i32, itext: &str) -> Widget {
    Widget::Button {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
        pressed: false,
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

fn get_screen_x(position: i32, screen_width: i32) -> f32 {
    let screen_x = (2.0 * position as f32) / screen_width as f32;
    screen_x - 1.0
}

fn get_screen_y(position: i32, screen_height: i32) -> f32 {
    let screen_y = (2.0 * position as f32) / screen_height as f32;
    1.0 - screen_y
}

fn get_widget_rect(widget: &Widget) -> Option<Rect> {
    match *widget {
        Widget::Form {ref position, ref size, ..} => Some(new_rect(position, size)),
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
    if x >= rect.x && x < rect.x + rect.w - 1 &&
       y >= rect.y && y < rect.y + rect.h - 1 {
           true
       }
    else {
        false
    }
}

/***********************************************************************************
 *      render_form
 *      render_button
 *      render_text
 *    
 *      Auxiliary functions to create RenderJobs
 ***********************************************************************************/
fn render_form(index: WidgetHandle, focus: bool, rect: Rect, title: &str) -> RenderJob {
    RenderJob::Form {   index: index,
                        focus: focus,
                        x: rect.x, 
                        y: rect.y, 
                        w: rect.w, 
                        h: rect.h,
                        title: title.to_owned() 
    }
}

fn render_button(index: WidgetHandle, pressed: bool, focus: bool, rect: Rect, text: &str) -> RenderJob {
    RenderJob::Button{  index: index,
                        pressed: pressed,
                        focus: focus,
                        x: rect.x, 
                        y: rect.y, 
                        w: rect.w, 
                        h: rect.h,
                        text: text.to_owned() }
}

pub fn render_text(index: WidgetHandle, x: i32, y: i32, text: &str) -> RenderJob {
    RenderJob::Label {
        index: index,
        text: text.to_owned(), 
        x: x, 
        y: y
    }
}

pub type WidgetHandle = i32;

pub struct UI {
    widgets: Vec<(WidgetHandle, Widget)>,
    widget_indices: Vec<usize>,
    mouse_focused_widgets: Vec<WidgetHandle>,
    events: Vec<WidgetEvent>,
    screen_rect: Rect,
    dragged_window: WidgetHandle,
}

impl UI {
    pub fn new (screen_width: i32, screen_height: i32) -> UI {
        
        UI {
            widgets: Vec::new(),
            widget_indices: Vec::new(),
            mouse_focused_widgets: Vec::new(),
            events: Vec::new(),
            screen_rect: Rect {x: 0, y: 0, w: screen_width, h: screen_height},
            dragged_window: -1,
        }
    }
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    fn find_widget_index_by_handle(&self, handle: WidgetHandle) -> usize {
        debug_assert!(handle >= 0);
        return self.widget_indices[handle as usize];
    }

    fn find_widget_handle_by_index(&self, index: usize) -> WidgetHandle {
        for (handle, i) in self.widget_indices.iter().enumerate() {
            if *i == index {return handle as WidgetHandle;}
        }

        panic!("Widget handle not found");
    }

/***********************************************************************************
 *      UI::add_widget
 *      
 *      Adds a widget to the UI with the specified parent. If -1 is used as
 *      parent the widget will be a root window.
 *      Returns a handle to the created widget for manipulation.
 ***********************************************************************************/
    pub fn add_widget(&mut self, parent: WidgetHandle, w: Widget) -> WidgetHandle {
        let index = self.widget_indices.len();

        if index >= std::i32::MAX as usize {
            panic!("Too many widgets");
        }

        if parent != -1 {
            //We want to put the widget after the parent
            //in the vector to always keep the vector sorted
            let parent_index = self.find_widget_index_by_handle(parent);
            
            //Fix the widgets index map because of insertion
            //in the middle of the vector
            for i in parent_index + 1 .. self.widgets.len() {
                let tmp = self.find_widget_handle_by_index(i) as usize;
                self.widget_indices[tmp] = self.widget_indices[tmp] + 1;
            }

            self.widget_indices.push(parent_index + 1);
            self.widgets.insert(parent_index + 1, (parent, w));
        } else {
            //This is a root window just append it last to the vector
            self.widget_indices.push(self.widgets.len());
            self.widgets.push((parent, w));
        }

        index as WidgetHandle
    }

/***********************************************************************************
 *      UI::mousemove
 *      
 *      Handles a mouse move event
 *      Drags widgets and handles widgets losing mouse focus
 ***********************************************************************************/
    pub fn mousemove(&mut self, last_mx: i32, last_my: i32, mx: i32, my: i32) {
        if self.dragged_window >= 0 {
            match self.widgets[self.dragged_window as usize].1 {
                Widget::Form{ref mut position, ..} => {
                    position.x+= mx - last_mx;
                    position.y+= my - last_my;
                }
                _ => ()
            }

            return ;
        }


        #[derive(PartialEq)]
        enum Iteration {
            Child,
            Sibling
        }

        let mut focused_widgets: Vec<i32> = Vec::new();
        let mut next_iteration = Iteration::Child;

        let mut last_parent: WidgetHandle = -1;
        let mut last_rect = self.screen_rect.clone();

        let mut rects: Vec<(WidgetHandle, Rect)>;
        rects = Vec::new();
        rects.push( (last_parent, last_rect) );

        let mut index = 0; 
        'loop_widgets: for widget in &self.widgets {
            //If the new widget is a child widget of the last widget
            if last_parent < widget.0  {
                if next_iteration == Iteration::Sibling { //Don't compare to child widgets
                    continue 'loop_widgets;               //If the mouse cursor wasn't within parent
                }

                rects.push( (widget.0, last_rect) );
                last_parent = widget.0;
                next_iteration == Iteration::Sibling;
            }
            //If the new widget is higher in the tree
            else if last_parent > widget.0  {
                while last_parent != widget.0  {
                    rects.pop();
                    let (lp, lr) = rects.last().unwrap().clone();
                    last_parent = lp;
                    last_rect = lr;
                }

                next_iteration = Iteration::Sibling;

                for e in &focused_widgets {
                    if *e == widget.0 {
                        continue 'loop_widgets;  //A sibling widget on top of this one has received mouse focus
                    }
                }
            }
            //The widget has the same parent as the previous one
            else {
                let (lp, lr) = rects.last().unwrap().clone();
                last_parent = lp; 
                last_rect = lr;
            }

            let rect = get_widget_rect(&widget.1);
            
            let final_rect = match rect {
                Some(rect) => {
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

            if inside_rect(final_rect, mx, my) {
                focused_widgets.push(self.find_widget_handle_by_index(index));
                next_iteration = Iteration::Child; 
            }
            else {
                next_iteration = Iteration::Sibling;
            }

            last_rect = final_rect;
            index+= 1;
        }

        //Check if any widgets lost focus
        {
            let mut iter = focused_widgets.iter();
            for w in &self.mouse_focused_widgets {
                if iter.any(|x| *x == *w) { }
                else {
                    let i = self.find_widget_index_by_handle(*w);
                    match self.widgets[i].1 {
                        Widget::Button{ref mut pressed, ..} => *pressed = false,
                        _ => ()
                    }
                }
            }
        }


        self.mouse_focused_widgets = focused_widgets;
    }

/***********************************************************************************
 *      UI::mousedown
 *      
 *      Handles a mousedown event to the widgets in focus
 ***********************************************************************************/
    pub fn mousedown(&mut self) {
        for w in self.mouse_focused_widgets.iter().rev() {
            let i = self.find_widget_index_by_handle(*w);
            match self.widgets[i].1 {
                Widget::Form{..} => { self.dragged_window = *w; }
                Widget::Button{ref mut pressed, ..} => { *pressed = true; return; }
                _ => ()
            }
        }
    }

/***********************************************************************************
 *      UI::mouseup
 *      
 *      Handles a mouseup event to the widgets in focus
 ***********************************************************************************/
    pub fn mouseup(&mut self) {
        self.dragged_window = -1;

        for w in self.mouse_focused_widgets.iter().rev() {
            let i = self.find_widget_index_by_handle(*w);
            match self.widgets[i].1 {
                Widget::Button{ref mut pressed, ..} => { *pressed = false; return; }
                _ => ()
            }
        }
    }

/***********************************************************************************
 *      UI::render
 *      
 *      Creates a list of all widgets render information in the form of the
 *      RenderJob enum.
 ***********************************************************************************/
    pub fn render(&self) -> Vec<RenderJob> {
        let mut render_jobs: Vec<RenderJob>;
        render_jobs = Vec::new();

        let mut last_parent: WidgetHandle = -1;
        let mut last_rect = self.screen_rect.clone();
        last_rect.w = last_rect.w * 2;
        last_rect.h = last_rect.h * 2;

        let mut rects: Vec<(i32, Rect)>;
        rects = Vec::new();
        rects.push( (last_parent, last_rect) );

        let mut index = 0; 
        for widget in &self.widgets {
            //If the new widget is a child widget of the last widget
            if last_parent < widget.0  {
                rects.push( (widget.0, last_rect) );
                last_parent = widget.0;
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
            }
            //The widget has the same parent as the previous one
            else {
                let (lp, lr) = rects.last().unwrap().clone();
                last_parent = lp; 
                last_rect = lr;
            }

            let rect = get_widget_rect(&widget.1);
            
            let final_rect = match rect {
                Some(rect) => {
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

            let widget_handle = self.find_widget_handle_by_index(index);

            let mut focus = false;
            for i in &self.mouse_focused_widgets {
                if *i == widget_handle {focus = true; break; }
            }

            match widget.1 {
                Widget::Form{ref title, ..} => {
                    render_jobs.push(render_form(widget_handle, focus, final_rect, &title));
                }
                Widget::Label{ref text, ..} => {
                    render_jobs.push(render_text(widget_handle, final_rect.x, final_rect.y, text));
                }
                Widget::Button{ref text, pressed, ..} => {
                    render_jobs.push(render_button(widget_handle, pressed, focus, final_rect, text));
                }
                _ => ()
            }

            last_rect = final_rect;
            index+= 1;
        }

        render_jobs
    }

/***********************************************************************************
 *      UI::num_widgets
 *      
 *      Number of created widgets in the UI
 ***********************************************************************************/
    pub fn num_widgets(&self) -> usize {
        self.widgets.len()
    }
}

/***********************************************************************************
 *
 *    Unittest  
 *
 ***********************************************************************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_rendering() {
        use conrod::backend::glium::glium;
        use glium::{DisplayBuild};
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(800, 600)
            .build_glium()
            .unwrap();
       
        let mut ui = UI::new(800, 600);
        let main_form = ui.add_widget(-1, new_form(50, 50, 400, 300, "Test menu"));
        let second_form = ui.add_widget(-1, new_form(100, 100, 200, 200, "Second form"));
        let main_label = ui.add_widget(main_form, new_label(10, 10, "Hello."));
        let main_button = ui.add_widget(main_form, new_button(10, 40, 100, 40, "OK."));

        let jobs = ui.render();
        assert!(jobs.len() == 4);

        for job in jobs {
            match job {
                RenderJob::Form { index, focus, x, y, w, h, ref title} => {
                    println!("{}", index);
                    assert!(index == main_form || index == second_form);
                    
                    if index == main_form {
                        assert_eq!(x, 50);
                        assert_eq!(y, 50);
                        assert_eq!(w, 400);
                        assert_eq!(h, 300);
                    }
                    else if index == second_form {
                        assert_eq!(x, 100);
                        assert_eq!(y, 100);
                        assert_eq!(w, 200);
                        assert_eq!(h, 200);
                    }
                    
                }
                RenderJob::Button {index, focus, pressed, x, y, w, h, ref text} =>  {
                    assert_eq!(index, main_button);

                    assert_eq!(x, 60);
                    assert_eq!(y, 90);
                    assert_eq!(w, 100);
                    assert_eq!(h, 40);
                }

                _ => ()
            }
        }

        ui.mousemove(0, 0, 80, 110);
        ui.mousedown();
    }
}