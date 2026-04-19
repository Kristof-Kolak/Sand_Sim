use macroquad::{miniquad::window::set_window_size, prelude::*, rand::ChooseRandom};

#[derive(Clone, Copy)]
enum Side{
    Left,
    Right
}

enum BrushMode{
    Paint,
    Clean
}

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

fn create_brush(brush_size: i32, screen_map: &mut Vec<Vec<u8>>, mouse_x: i32, mouse_y:i32, brush_mode: BrushMode){
    match brush_mode {
        BrushMode::Paint => {
            for dy in -brush_size..brush_size + 1{
                for dx in -brush_size..brush_size + 1{
                    if mouse_x + dx >= 0 && mouse_x + dx < WIDTH && mouse_y + dy >= 0 && mouse_y + dy< HEIGHT{
                        screen_map[(mouse_y + dy) as usize][(mouse_x + dx) as usize] = 1;
                    }    
                }
            } 
        }   
        BrushMode::Clean =>{
            for dy in -brush_size..brush_size + 1{
                for dx in -brush_size..brush_size + 1{
                    if mouse_x + dx >= 0 && mouse_x + dx < WIDTH && mouse_y + dy >= 0 && mouse_y + dy< HEIGHT{
                        screen_map[(mouse_y + dy) as usize][(mouse_x + dx) as usize] = 0;
                    }    
                }
            } 
        }
    }
}

#[macroquad::main("Sand_Sim")]
async fn main() {

    
    let mut screen_map: Vec<Vec<u8>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];

    let mut brush_size = 0;
    let sensitivity = 0.01;
    let mut scroll_acc:f32 =  0.0;
    set_window_size(WIDTH as u32, HEIGHT as u32);
    let mut left_to_right = true; 

    loop {


        clear_background(BLACK);

        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left){
            if (x < WIDTH as f32 && x >= 0.0) && (y < HEIGHT as f32 && y >= 0.0){
                    create_brush(brush_size, &mut screen_map, x as i32, y as i32, BrushMode::Paint);
                
            }
        }
        if is_mouse_button_down(MouseButton::Right){
            if (x < WIDTH as f32 && x >= 0.0) && (y < HEIGHT as f32 && y >= 0.0){
                    create_brush(brush_size, &mut screen_map, x as i32, y as i32, BrushMode::Clean);
            }
        }
        let (_mouse_x, mouse_y) = mouse_wheel();

        scroll_acc += mouse_y * sensitivity;

        if scroll_acc >= 1.0{
            brush_size += 1;
            scroll_acc -= 1.0;
        }
        if scroll_acc <= -1.0{
            brush_size -= 1;
            scroll_acc += 1.0;
        }
        
        if brush_size < 0{
            brush_size = 0;
        }else if brush_size > 200{
            brush_size = 200;
        }



        let mut new_map = screen_map.clone(); 
        for index_y in (0..HEIGHT).rev() {
            if left_to_right == true{
                for index_x in 0..WIDTH{
                    if index_y == HEIGHT-1{
                        //bottom line condition
                    }else if screen_map[index_y as usize][index_x as usize] == 1 {
                        if index_y+1 < HEIGHT{                    
                            if new_map[(index_y+1) as usize][index_x as usize] == 0{ 
                                //bottom simple movement
                                new_map[index_y as usize][index_x as usize] = 0;
                                new_map[(index_y+1) as usize][index_x as usize] = 1;
                            }else{
                                //diagonal movement
                                if (index_x - 1 >= 0) && (index_x + 1 < WIDTH){
                                    let first_side = [Side::Left, Side::Right].choose().copied();
                                    match first_side {
                                        Some(Side::Left) =>{
                                            if  new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                            }else if new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                            }
                                        }
                                        Some(Side::Right) =>{
                                            if  new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                            }else if  new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                            } 
                                        }
                                        None =>{

                                        }
                                    }

                                }else if index_x - 1 >= 0{ 
                                    if new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                        new_map[index_y as usize][index_x as usize] = 0;
                                        new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                    }
                                }else if index_x + 1 < WIDTH{
                                    if  new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                        new_map[index_y as usize][index_x as usize] = 0;
                                        new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }else if left_to_right==false{
                for index_x in (0..WIDTH).rev(){
                    if index_y == HEIGHT-1{
                        //bottom line condition
                    }else if screen_map[index_y as usize][index_x as usize] == 1 {
                        if index_y+1 < HEIGHT{                    
                            if  new_map[(index_y+1) as usize][index_x as usize] == 0{ 
                                //bottom simple movement
                                new_map[index_y as usize][index_x as usize] = 0;
                                new_map[(index_y+1) as usize][index_x as usize] = 1;
                            }else{
                                //diagonal movement
                                if (index_x - 1 >= 0) && (index_x + 1 < WIDTH){
                                    let first_side = [Side::Left, Side::Right].choose().copied();
                                    match first_side {
                                        Some(Side::Left) =>{
                                            if  new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                            }else if  new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                            }
                                        }
                                        Some(Side::Right) =>{
                                            if  new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                            }else if  new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                                new_map[index_y as usize][index_x as usize] = 0;
                                                new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                            } 
                                        }
                                        None =>{

                                        }
                                    }

                                }else if index_x - 1 >= 0{ 
                                    if new_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                        new_map[index_y as usize][index_x as usize] = 0;
                                        new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                    }
                                }else if index_x + 1 < WIDTH{
                                    if  new_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                        new_map[index_y as usize][index_x as usize] = 0;
                                        new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                    }
                                }
                            }
                        }
                    }
                }
            } 
        }
        

        if left_to_right == true{
            left_to_right = false;
        }else{
            left_to_right = true;
        }

        screen_map = new_map;
        for (index_y, row) in screen_map.iter().enumerate(){
            for (index_x, _) in row.iter().enumerate(){
                if screen_map[index_y][index_x] == 1{
                    draw_rectangle(index_x as f32, index_y as f32, 1.0, 1.0, YELLOW);

                }
            }
        }
        next_frame().await
    }
}