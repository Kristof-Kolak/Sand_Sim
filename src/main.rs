use macroquad::{miniquad::window::set_window_size, prelude::*};






const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;


#[macroquad::main("MyGame")]
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
                            //diagonall movement
                            if index_x - 1 >= 0{
                                if screen_map[(index_y+1) as usize][(index_x-1) as usize] == 0{
                                    new_map[index_y as usize][index_x as usize] = 0;
                                    new_map[(index_y+1) as usize][(index_x-1) as usize] = 1;
                                }else if index_x + 1 < WIDTH{
                                    if screen_map[(index_y+1) as usize][(index_x+1) as usize] == 0{
                                        new_map[index_y as usize][index_x as usize] = 0;
                                        new_map[(index_y+1) as usize][(index_x+1) as usize] = 1;
                                    }
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