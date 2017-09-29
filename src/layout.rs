
use cassowary;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;

type WidgetHandle = i32;

#[derive(Eq, PartialEq, Clone)]
struct LayoutPosition(u32, u32);

fn new_term (variable: cassowary::Variable, coefficient: f64) -> cassowary::Term {
    cassowary::Term {
        variable: variable,
        coefficient: coefficient
    }
}

impl LayoutPosition {
    pub fn new(icol: u32, irow: u32) -> LayoutPosition {
        LayoutPosition (icol, irow )
    }
}

impl Ord for LayoutPosition {
    fn cmp(&self, other: &LayoutPosition) -> Ordering {
        if self.1 < other.1  {
            return Ordering::Less;
        }
        if self.1 == other.1 {
            if self.0 < other.0 {
                return Ordering::Less;
            }
            if self.0 == other.0 {
                return Ordering::Equal;
            }
        }
        return Ordering::Greater;
    }
}

impl PartialOrd for LayoutPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Component {
    leading: u32,
    width: u32,
    top: u32,
    height: u32,

    standard_width: u32,
    standard_height: u32,

    item: WidgetHandle,
}

struct Span {
    leading: u32,
    width: u32,
    top: u32,
    height: u32,

    start_position: LayoutPosition,
}

enum Cell {
    Component(Component),
    Span(Span)
}

fn new_component(item: WidgetHandle, standard_width: u32, standard_height: u32) -> Cell {
    Cell::Component(
        Component {
            item: item,
            standard_width: standard_width,
            standard_height: standard_height,
            leading: 0,
            width: 0,
            top: 0,
            height: 0,
        }
    )
}

fn new_span(x: u32, y: u32) -> Cell {
    Cell::Span( Span {
        leading: 0,
        width: 0,
        top: 0,
        height: 0,
        start_position: LayoutPosition::new(x, y)
    })
}

impl Cell {
    pub fn set_leading(&mut self, x: u32) {
        match *self {
            Cell::Component(ref mut c) => c.leading = x,
            Cell::Span(ref mut s) => s.leading = x
        }
    }

    pub fn set_width(&mut self, x: u32) {
        match *self {
            Cell::Component(ref mut c) => c.width = x,
            Cell::Span(ref mut s) => s.width = x
        }
    }

    pub fn set_top(&mut self, x: u32) {
        match *self {
            Cell::Component(ref mut c) => c.top = x,
            Cell::Span(ref mut s) => s.top = x
        }
    }

    pub fn set_height(&mut self, x: u32) {
        match *self {
            Cell::Component(ref mut c) => c.height = x,
            Cell::Span(ref mut s) => s.height = x
        }
    }

    pub fn get_preferred_height(&self) -> u32 {
        match *self {
            Cell::Component(ref c) => c.standard_height,
            Cell::Span(ref c) => 12
        }
    }
}

pub struct GridLayout {
    wrap: u32,
    current_x: u32,
    current_y: u32,
    grid: BTreeMap<LayoutPosition, Cell>
}


enum Action {
    Add{item: WidgetHandle},
    Wrap,
    Span(u32, u32)
}

pub struct AccessLayout {
    actions: Vec<Action>,
}

pub struct WrapOnlyAccessLayout<'a> {
    access_layout: &'a mut AccessLayout
}

impl<'a> WrapOnlyAccessLayout<'a> {
    pub fn new(al: &'a mut AccessLayout) -> WrapOnlyAccessLayout {
        WrapOnlyAccessLayout {
            access_layout: al
        }
    }

    pub fn wrap(&mut self) {
        self.access_layout.wrap();
    }
}

impl AccessLayout {
    pub fn add(&mut self, item: WidgetHandle) -> &mut AccessLayout {
        self.actions.push(Action::Add{item});
        self
    }

    pub fn span(&mut self, x: u32, y: u32) -> &mut AccessLayout {
        self.actions.push(Action::Span(x, y));
        self
    }

    pub fn wrap(&mut self) -> WrapOnlyAccessLayout {
        self.actions.push(Action::Wrap);

        WrapOnlyAccessLayout::new(self)
    }
}

impl GridLayout {
    pub fn new() -> GridLayout {
        GridLayout {
            wrap: 0,
            current_x: 0,
            current_y: 0,
            grid: BTreeMap::new()
        }
    }

    pub fn set_wrap(mut self, wrap: u32) -> GridLayout {
        self.wrap = wrap;

        self
    }
    
    pub fn access(&mut self, access_closure: &Fn(&mut AccessLayout), standard_size_closure: &Fn(WidgetHandle) -> (u32, u32)) {
        let mut access_object = AccessLayout { actions: Vec::new() };  

        access_closure(&mut access_object);

        for a in access_object.actions {
            match a {
                Action::Add{item} => { 
                    let (width, height) = standard_size_closure(item);
                    self.add(item, width, height);
                },
                Action::Wrap => {self.current_x = 0; self.current_y = self.current_y + 1;},
                Action::Span(x, y) => self.span(x, y)
            }
        }

        self.internal_update();        
    }

    pub fn update(&self, result_closure: &mut FnMut(WidgetHandle, (u32, u32, u32, u32))) {
        for (ref position, ref cell) in &self.grid {
            match *cell {
                &Cell::Component(ref c) => result_closure(c.item, (c.leading, c.top, c.width, c.height)),
                &Cell::Span(_) => ()
            }
        }
    }

    fn move_position(&mut self) {
        while self.grid.contains_key( &LayoutPosition::new(self.current_x, self.current_y) ) {
            self.current_x = self.current_x + 1;
            if self.wrap > 0 {
                if self.current_x >= self.wrap {
                    self.current_x = 0;
                    self.current_y = self.current_y + 1;
                }
            }
        }
    }

    fn add(&mut self, item: WidgetHandle, standard_width: u32, standard_height: u32) {
        self.move_position();
        self.grid.insert(LayoutPosition::new(self.current_x, self.current_y), new_component(item, standard_width, standard_height));
    }

    fn span(&mut self, x: u32, y: u32) {
        if !self.grid.contains_key( &LayoutPosition::new(self.current_x, self.current_y) ) {
            panic!("Need grid cell to span from.");
        }

        for cell_y in 0..y {
            for cell_x in 0..x {
                if cell_x == 0 && cell_y == 0 {continue;}

                self.grid.insert(LayoutPosition::new(self.current_x + cell_x, self.current_y + cell_y), new_span(x, y));
            }
        }
    }

    fn internal_update(&mut self) {
        
        let mut num_columns = 0usize;

        if self.wrap == 0 {
            //Find the longest row since this grid doesn't have a fixed wrap
            let mut current_row: i32 = -1;
            let mut num_in_row = 1usize;
            for (ref position, _) in &self.grid {
                if position.1 as i32 > current_row {
                    //We have reached a new row
                    if num_in_row > num_columns { num_columns = num_in_row; } //We have found a new longest row
                    num_in_row = 0;
                    current_row = position.1 as i32;
                }
                num_in_row = num_in_row + 1;
            }
            if num_in_row > num_columns { num_columns = num_in_row; } //Last row was the longest row
        }
        else {
            num_columns = self.wrap as usize;
        }

        //Find the number of rows
        let position: LayoutPosition;
        { 
            position = self.grid.iter().next_back().unwrap().0.clone();
        }
        let num_rows = position.1 as usize + 1;

        self.calculate_row(num_rows, num_columns);
    }

    fn calculate_row(&mut self, num_rows: usize, num_columns: usize) {
        use cassowary::{Solver, Variable, Term, Expression};
        use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};
        use cassowary::WeightedRelation::*;

        let mut var_names: HashMap<Variable, (String, f64) > = HashMap::new();

        let mut solver = Solver::new();

        let widget_width: Variable = Variable::new();
        let widget_height: Variable = Variable::new();

        //One vector for every row in the grid
        let mut width_vars: Vec<Vec<Variable>> = Vec::new();
        let mut height_vars: Vec<Vec<Variable>> = Vec::new();
        let mut trailing_gap_vars: Vec<Vec<Variable>> = Vec::new();
        let mut inferior_gap_vars: Vec<Vec<Variable>> = Vec::new();
        let mut top_margin: Vec<Variable> = Vec::new();
        let mut bottom_margin: Vec<Variable> = Vec::new();
        let mut left_margin: Vec<Variable> = Vec::new();
        let mut right_margin: Vec<Variable> = Vec::new();
        let mut all_vars_equal_parent_width_expr: Vec<Expression> = Vec::new();
        let mut all_vars_equal_parent_height: Vec<Vec<Term>> = Vec::new();
        let mut all_vars_equal_parent_height_expr: Vec<Expression> = Vec::new();

        for c in 0..num_columns {
            all_vars_equal_parent_height.push(Vec::new());
            top_margin.push(Variable::new());
            bottom_margin.push(Variable::new());
            solver.add_edit_variable(bottom_margin[c], 1.0).unwrap();
            solver.suggest_value(bottom_margin[c], 1000000.0).unwrap();
            var_names.insert(top_margin[c].clone(), ("Top Margin".to_owned(), 0.0));
            var_names.insert(bottom_margin[c].clone(), ("Bottom Margin".to_owned(), 0.0));

            all_vars_equal_parent_height[c].push(new_term(top_margin[c], 1.0));
            all_vars_equal_parent_height[c].push(new_term(bottom_margin[c], 1.0));
        }


        var_names.insert(widget_width.clone(), ("Widget width".to_owned(), 0.0));
        var_names.insert(widget_height.clone(), ("Widget height".to_owned(), 0.0));

        for row in 0..num_rows {
            width_vars.push(Vec::new());
            height_vars.push(Vec::new());
            trailing_gap_vars.push(Vec::new());
            inferior_gap_vars.push(Vec::new());
            left_margin.push(Variable::new());
            right_margin.push(Variable::new());
           
            var_names.insert(left_margin[row].clone(), ("Left Margin".to_owned(), 0.0));
            var_names.insert(right_margin[row].clone(), ("Right Margin".to_owned(), 0.0));
            
            let mut all_vars_equal_parent_width: Vec<Term> = Vec::new();
            all_vars_equal_parent_width.push(new_term(left_margin[row], 1.0));
            all_vars_equal_parent_width.push(new_term(right_margin[row], 1.0));

            for c in 0usize..num_columns {
                width_vars[row].push(Variable::new());
                var_names.insert(width_vars[row][c].clone(), (format!("w{}", c), 0.0));
                all_vars_equal_parent_width.push(new_term(width_vars[row][c], 1.0));

                let mut height = 0.0;
                match self.grid.get(&LayoutPosition::new(c as u32, row as u32)) {
                    Some(ref cell) => {
                        height = cell.get_preferred_height() as f64;
                    }
                    None =>()
                }

                height_vars[row].push(Variable::new());
                var_names.insert(height_vars[row][c], (format!("h{}", c), 0.0));
                all_vars_equal_parent_height[c].push(new_term(height_vars[row][c], 1.0));
                solver.add_edit_variable(height_vars[row][c], 1.0).unwrap();
                solver.suggest_value(height_vars[row][c], height).unwrap();
                if row + 1 != num_rows {
                    inferior_gap_vars[row].push(Variable::new());
                    var_names.insert(inferior_gap_vars[row][c], (format!("i{}", c), 0.0));
                    all_vars_equal_parent_height[c].push(new_term(inferior_gap_vars[row][c], 1.0));
                }

                if c + 1 != num_columns {
                    trailing_gap_vars[row].push(Variable::new());
                    var_names.insert(trailing_gap_vars[row][c], (format!("g{}", c), 0.0));
                    all_vars_equal_parent_width.push(new_term(trailing_gap_vars[row][c], 1.0));
                    solver.add_edit_variable(trailing_gap_vars[row][c], 1.0).unwrap();
                    solver.suggest_value(trailing_gap_vars[row][c], 8.0).unwrap();
                }
            }

            all_vars_equal_parent_width_expr.push(Expression::new(all_vars_equal_parent_width, 0.0));

            solver.add_constraints(&[all_vars_equal_parent_width_expr[row].clone() |EQ(REQUIRED)| widget_width]).unwrap();
            solver.add_constraints(&[left_margin[row]    |LE(STRONG)|    12.0, 
                                     right_margin[row]   |LE(STRONG)|    12.0,] ).unwrap();
        
            if num_columns > 1 {
                for i in 0 .. num_columns - 1 {
                    solver.add_constraint(width_vars[row][i] |EQ(STRONG)| width_vars[row][i+1]).unwrap()
                }
                for i in 0 .. num_columns - 2 {
                    solver.add_constraints(&[trailing_gap_vars[row][i] |EQ(STRONG)| trailing_gap_vars[row][i+1],
                                            trailing_gap_vars[row][i] |LE(STRONG)| 8.0]).unwrap()
                }
            }
        }

        for c in 0..num_columns {
            all_vars_equal_parent_height_expr.push(Expression::new(all_vars_equal_parent_height[c].clone(), 0.0));
            solver.add_constraint(all_vars_equal_parent_height_expr[c].clone() |EQ(REQUIRED)| widget_height).unwrap();
            solver.add_constraints(&[top_margin[c]      |EQ(STRONG)|    12.0,
                                     bottom_margin[c]   |GE(STRONG)|     12.0,
            ] ).unwrap();

            if num_rows > 1 {
                for row in 0..num_rows - 1 {
                    if c < num_columns - 1 {
                        let num_height_vars = height_vars.len();
                        let num_inferior_gap_vars = inferior_gap_vars.len();
                        let num_height_var_row = height_vars[row].len();
                        let num_inferior_hap_var_row = inferior_gap_vars[row].len();
                        solver.add_constraint(height_vars[row][c] + inferior_gap_vars[row][c] |EQ(MEDIUM)| height_vars[row][c+1] + inferior_gap_vars[row][c+1]).unwrap();
                    }
                    solver.add_constraint(inferior_gap_vars[row][c] |GE(WEAK)| 8.0).unwrap();
                    if row < num_rows - 2 {
                        solver.add_constraint(inferior_gap_vars[row][c] |EQ(WEAK)| inferior_gap_vars[row+1][c]);
                    }
                }   
            }
        }


        solver.add_edit_variable(widget_width, 1.0).unwrap();
        solver.add_edit_variable(widget_height, 1.0).unwrap();
        solver.suggest_value(widget_width, 300.0).unwrap();
        solver.suggest_value(widget_height, 300.0).unwrap();

        macro_rules! get_value {
            ($x:expr) => { 
                match var_names.get($x) {
                    Some(c) => (c.0.clone(), c.1),
                    None => ("".to_owned(), 0.0)
                }
            }
        }

        macro_rules! set_value{
            ($var:ident, $value:ident) => {
                match var_names.get_mut($var) {
                    Some(ref mut c) => (*c).1 = $value,
                    None => ()
                }
            }
        }
        for c in solver.fetch_changes().iter() {
            let (ref var, value) = *c;
            set_value!(var, value);
        }        

        let mut y: Vec<f64> = Vec::new();
        for row in 0..num_rows {
            let mut x = get_value!(&left_margin[row]).1;
            for i in 0..num_columns{
                if row == 0 { y.push(get_value!(&top_margin[i]).1)}

                match self.grid.get_mut(&LayoutPosition::new(i as u32, row as u32)) {
                    Some( mut c) => {
                        (*c).set_leading(x as u32);
                        let width = get_value!(&width_vars[row][i]).1;
                        let height = get_value!(&height_vars[row][i]).1;
                        (*c).set_top(y[i] as u32);
                        (*c).set_width(width as u32);
                        x = x + width;
                        y[i] = y[i] + height;
                        (*c).set_height(height as u32);
                    },
                    None =>  x = x + get_value!(&width_vars[row][i]).1
                }

                if i < num_columns - 1 {
                    x = x + get_value!(&trailing_gap_vars[row][i]).1;
                }

                if row < num_rows - 1 {
                    y[i] = y[i] + get_value!(&inferior_gap_vars[row][i]).1;
                }
            }
        }
    }

    pub fn print(self) {
        let mut last_position = LayoutPosition::new(0,0);

        for (ref position, ref c) in &self.grid {
            if position.1 > last_position.1 {
                println!("");
            }

            match *c {
                &Cell::Component(ref c) => print!("\t{} ({}, {})", c.item, c.leading, c.top),
                &Cell::Span(_) => print!("\tSPAN\t")
            }

            last_position = (*position).clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_grid_layout() {
        use std::cmp::Ordering;
        
        assert_eq!(Ordering::Less,  LayoutPosition(0, 1).cmp(&LayoutPosition(0, 2)) );
        assert_eq!(Ordering::Less,  LayoutPosition(1, 2).cmp(&LayoutPosition(2, 2)) );
        assert_eq!(Ordering::Less,  LayoutPosition(1, 0).cmp(&LayoutPosition(0, 1)) );
        assert_ne!(Ordering::Less,  LayoutPosition(0, 1).cmp(&LayoutPosition(1, 0)) );
        assert_ne!(Ordering::Less,  LayoutPosition(0, 2).cmp(&LayoutPosition(0, 1)) );


        let mut layouter = GridLayout::new()
                        .set_wrap(4);


        let w: Vec<i32> = (0..10).collect();

        layouter.access(&|ref mut l| {
            l.add(w[0]);
            l.add(w[1]).span(2, 2);
            l.add(w[2]);
            l.add(w[3]);
            l.add(w[4]);
            l.add(w[5]);
            l.add(w[6]).wrap().wrap();
            l.add(w[7]).add(w[8]).add(w[9]);
            },

            &|l| -> (u32, u32) {
                (30, 12)
            }
        );

        layouter.update(&mut |index: WidgetHandle, rect: (u32, u32, u32, u32)| {
            println!("{}: ({}, {}, {}, {})", index, rect.0, rect.1, rect.2, rect.3);
        });

        layouter.print();

    }

}