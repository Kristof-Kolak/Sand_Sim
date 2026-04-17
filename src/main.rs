use macroquad::{miniquad::window::set_window_size, prelude::*, rand::ChooseRandom};

#[derive(Clone, Copy)]
enum Side{
    Left,
    Right
}


const WIDTH: i32 = 200;
const HEIGHT: i32 = 200;



#[macroquad::main("Sand_Sim")]
async fn main() {

    
    let mut screen_map: Vec<Vec<u8>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];


    set_window_size(WIDTH as u32, HEIGHT as u32);
    
    

    loop {


        clear_background(BLACK);

        let (x, y) = mouse_position();
        if is_mouse_button_down(MouseButton::Left){
            if (x < WIDTH as f32 && x >= 0.0) && (y < HEIGHT as f32 && y >= 0.0){
                if screen_map[y as usize][x as usize] != 1{
                    screen_map[y as usize][x as usize] = 1;
                }
            }
        }
        if is_mouse_button_down(MouseButton::Right){
            if (x < WIDTH as f32 && x >= 0.0) && (y < HEIGHT as f32 && y >= 0.0){
                if screen_map[y as usize][x as usize] == 1{
                    screen_map[y as usize][x as usize] = 0;
                }
            }
        }


        let mut new_map = screen_map.clone(); 
        for index_y in (0..HEIGHT).rev() {
            for index_x in 0..WIDTH{
                if index_y == HEIGHT-1{
                    //bottom line condition
                }else if screen_map[index_y as usize][index_x as usize] == 1 {
                    if index_y+1 < HEIGHT{                    
                        if screen_map[(index_y+1) as usize][index_x as usize] == 0{
                            //bottom simple movement
                            new_map[index_y as usize][index_x as usize] = 0;
                            new_map[(index_y+1) as usize][index_x as usize] = 1;
                        }else{
                            //diagonal movement
                            if (index_x - 1 >= 0) && (index_x + 1 < WIDTH){
                                let first_side = [Side::Left, Side::Right].choose().copied();
                                match first_side {
                                    Some(Side::Left) =>{
                                        if screen_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                            new_map[index_y as usize][index_x as usize] = 0;
                                            new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                        }else if screen_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                            new_map[index_y as usize][index_x as usize] = 0;
                                            new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                        }
                                    }
                                    Some(Side::Right) =>{
                                        if screen_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                            new_map[index_y as usize][index_x as usize] = 0;
                                            new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                        }else if screen_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                            new_map[index_y as usize][index_x as usize] = 0;
                                            new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                        } 
                                    }
                                    None =>{

                                    }
                                }

                            }else if index_x - 1 >= 0{ 
                                if screen_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                    new_map[index_y as usize][index_x as usize] = 0;
                                    new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                }
                            }else if index_x + 1 < WIDTH{
                                if screen_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                    new_map[index_y as usize][index_x as usize] = 0;
                                    new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                }
                            }
                        }
                    }
                }
            }
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