use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Cell {
    x: f64,
    y: f64,
    state: bool
}

fn create_cell(ex: f64, why: f64, cell_size: f64) -> Cell {
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
    cell: Cell
}

fn model(_app: &App) -> Model {
    let cell = create_cell(0.0, 0.0, 1.0);
    Model {
        cell: cell
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, model: &Model, frame: Frame){
    if model.cell.state {
        frame.clear(PURPLE);
    } else {
        frame.clear(GREEN);
    }
}
