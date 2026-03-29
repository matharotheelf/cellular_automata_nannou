use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Cell {
    x: f32,
    y: f32,
    state: bool
}

fn create_cell(ex: f32, why: f32, cell_size: f32) -> Cell {
    let x = ex*cell_size;
    let y = why*cell_size;
    let state = random();

    Cell {
        x: x,
        y: y,
        state: state
    }
}

struct Model {
    cells: Vec<Cell>,   
    cell_size: f32
}

fn model(app: &App) -> Model {
    let mut cells: Vec<Cell> = Vec::new(); 
    let cell_size = 10.0;

    let win = app.window_rect();
    let width = win.w();   // f32
    let height = win.h();  // f32

    let numX = (width/cell_size).floor() as i32;
    let numY = (height/cell_size).floor() as i32;

    for x_index in 0..numX {
        for y_index in 0..numY {
            println!("{}", x_index);
            println!("{}", y_index);
            let cell = create_cell(x_index as f32, y_index as f32, cell_size);
            cells.push(cell);
        }
    }

    Model {
        cells: cells,
        cell_size: cell_size
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();

    let draw = app.draw().x_y(win.left(), win.bottom());
    draw.background().color(BLACK);

    let cells = &model.cells;
    let cell_size = model.cell_size;

    for cell in cells {
        let color = if cell.state { BLACK } else { WHITE };

        draw.rect()
            .x_y(cell.x, cell.y)  // position
            .w_h(cell_size, cell_size)    // width, height (equal = square)
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}
