use macroquad::{miniquad::window::set_window_size, prelude::*};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
const SAND: u8 = 1;
const EMPTY: u8 = 0;

fn is_in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT
}

fn apply_brush(screen_map: &mut Vec<Vec<u8>>, brush_size: i32, mouse_x: i32, mouse_y: i32, value: u8) {
    for dy in -brush_size..=brush_size {
        for dx in -brush_size..=brush_size {
            let x = mouse_x + dx;
            let y = mouse_y + dy;
            if is_in_bounds(x, y) {
                screen_map[y as usize][x as usize] = value;
            }
        }
    }
}

fn try_move_sand(new_map: &mut Vec<Vec<u8>>, x: i32, y: i32, prefer_right: bool) {
    if y + 1 >= HEIGHT || new_map[(y + 1) as usize][x as usize] != EMPTY {
        // Bottom blocked, try diagonals
        let mut directions = vec![(x - 1, -1), (x + 1, 1)];
        if prefer_right {
            directions.reverse();
        }
        for (new_x, _) in directions {
            if is_in_bounds(new_x, y + 1) && new_map[(y + 1) as usize][new_x as usize] == EMPTY {
                new_map[y as usize][x as usize] = EMPTY;
                new_map[(y + 1) as usize][new_x as usize] = SAND;
                return;
            }
        }
    } else {
        // Bottom is free, move down
        new_map[y as usize][x as usize] = EMPTY;
        new_map[(y + 1) as usize][x as usize] = SAND;
    }
}

fn update_sand(screen_map: &mut Vec<Vec<u8>>, left_to_right: &mut bool) {
    let mut new_map = screen_map.clone();
    let prefer_right = *left_to_right;
    
    for y in (0..HEIGHT - 1).rev() {
        let x_range: Box<dyn Iterator<Item = i32>> = if *left_to_right {
            Box::new(0..WIDTH)
        } else {
            Box::new((0..WIDTH).rev())
        };
        
        for x in x_range {
            if screen_map[y as usize][x as usize] == SAND {
                try_move_sand(&mut new_map, x, y, prefer_right);
            }
        }
    }
    
    *screen_map = new_map;
    *left_to_right = !*left_to_right;
}

#[macroquad::main("Sand_Sim")]
async fn main() {
    let mut screen_map: Vec<Vec<u8>> = vec![vec![EMPTY; WIDTH as usize]; HEIGHT as usize];

    let mut brush_size = 0;
    let sensitivity: f32 = 0.01;
    let mut scroll_acc: f32 = 0.0;
    set_window_size(WIDTH as u32, HEIGHT as u32);
    let mut left_to_right = true;

    loop {
        clear_background(BLACK);

        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && is_in_bounds(x as i32, y as i32) {
            apply_brush(&mut screen_map, brush_size, x as i32, y as i32, SAND);
        }
        if is_mouse_button_down(MouseButton::Right) && is_in_bounds(x as i32, y as i32) {
            apply_brush(&mut screen_map, brush_size, x as i32, y as i32, EMPTY);
        }

        let (_mouse_x, mouse_y) = mouse_wheel();
        scroll_acc += mouse_y * sensitivity;

        if scroll_acc >= 1.0 {
            brush_size += 1;
            scroll_acc -= 1.0;
        }
        if scroll_acc <= -1.0 {
            brush_size -= 1;
            scroll_acc += 1.0;
        }

        brush_size = brush_size.max(0).min(200);

        update_sand(&mut screen_map, &mut left_to_right);

        for (y, row) in screen_map.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if screen_map[y][x] == SAND {
                    draw_rectangle(x as f32, y as f32, 1.0, 1.0, YELLOW);
                }
            }
        }
        next_frame().await
    }
}