fn modify_field(mut field: Vec<String>, x: usize, y: usize) -> Vec<String> {
    if field[y].chars().nth(x).unwrap().is_ascii_digit(){
        let mut num: u32 = field[y].chars().nth(x).unwrap().to_digit(10).unwrap();
        num = num + 1;
        let numstr = num.to_string();
        field[y].replace_range(x..x+1,numstr.as_str());
    } else if field[y].chars().nth(x).unwrap().is_whitespace(){
        field[y].replace_range(x..x+1,"1");
    }
    field
}

fn sourrounding_area(mut field: Vec<String>, x: usize, y: usize, height: usize, width: usize ) -> Vec<String> {
    //1D vertical field:
    if width == 1{
        if height == 1{
            return field;
        //top
        }else if y == 0{
            field[y+1].replace_range(x..x+1,"1");
            //field[1][0] = String::from("1");
        }else{
            field = modify_field(field, x, y-1);

            //not bottom
            if y != height-1{
                field = modify_field(field, x, y+1);
            }
        }
    //1D horizontal field:
    }else if height == 1{
        //left
        if x == 0{
            field[y].replace_range(x+1..x+2,"1");
            //field[0][1] = String::from("1");
        }else{
            field = modify_field(field, x-1, y);

            //not right
            if x != width-1{
                field[y].replace_range(x+1..x+2,"1");
                //field[0][x+1] = String::from("1");
            }
        }
    //2D field:
    }else{
        //left border
        if x == 0{
            //left-top corner
            if y == 0{
                field[y].replace_range(x+1..x+2,"1");
                field[y+1].replace_range(x..x+1,"1");
                field[y+1].replace_range(x+1..x+2,"1");

                //field[0][1] = String::from("1");
                //field[1][0] = String::from("1");
                //field[1][1] = String::from("1");

            //left-bottom corner
            }else if y == height-1{
                field = modify_field(field, x, y-1);
                field = modify_field(field, x+1, y-1);
                field = modify_field(field, x+1, y);

            //left-middle
            }else{
                field = modify_field(field, x, y-1);
                field = modify_field(field, x+1, y-1);
                field = modify_field(field, x+1, y);
                field[y+1].replace_range(x+1..x+2,"1");
                field[y+1].replace_range(x..x+1,"1");
                //field[y+1][1] = String::from("1");
                //field[y+1][0] = String::from("1");
            }
        //right border
        }else if x == width-1{
            //right-top corner
            if y == 0{
                field = modify_field(field, x, y+1);
                field = modify_field(field, x-1, y+1);
                field = modify_field(field, x-1, y);

            //right-bottom corner
            }else if y == height-1{
                field = modify_field(field, x-1, y);
                field = modify_field(field, x-1, y-1);
                field = modify_field(field, x, y-1);

            //right-middle
            }else{
                field = modify_field(field, x, y+1);
                field = modify_field(field, x-1, y+1);
                field = modify_field(field, x-1, y);
                field = modify_field(field, x-1, y-1);
                field = modify_field(field, x, y-1);

            }
        }else{
            //top-middle
            if y == 0{
                field[y].replace_range(x+1..x+2,"1");
                field[y+1].replace_range(x+1..x+2,"1");
                //field[y][x+1] = String::from("1");
                //field[y+1][x+1] = String::from("1");
                field = modify_field(field, x, y+1);
                field = modify_field(field, x-1, y+1);
                field = modify_field(field, x-1, y);

            //bottom-middle
            }else if y == height-1{
                field = modify_field(field, x-1, y);
                field = modify_field(field, x-1, y-1);
                field = modify_field(field, x, y-1);
                field = modify_field(field, x+1, y-1);
                field = modify_field(field, x+1, y);

            //no border
            }else{
                field = modify_field(field, x, y-1);
                field = modify_field(field, x+1, y-1);
                field = modify_field(field, x+1, y);
                field[y+1].replace_range(x+1..x+2,"1");
                //field[y+1][x+1] = String::from("1");
                field = modify_field(field, x, y+1);
                field = modify_field(field, x-1, y+1);
                field = modify_field(field, x-1, y);
                field = modify_field(field, x-1, y-1);

            }

        }

    }

    field
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field: Vec<String> = Vec::new();
    let mut height: usize = 0;
    let mut width: usize = 0;
    if !minefield.is_empty() {
        height = minefield.len() as usize;
        if !minefield[0].is_empty() {
            width = minefield[0].len() as usize;
            field = vec![String::from(" ".repeat(minefield[0].len())); minefield.len()];
        }else{
            field = vec![String::from("")];
        }
    }
    for (row_index, row) in minefield.iter().enumerate() {
        for(col_index, chr) in row.chars().enumerate(){
            if chr == '*'{
                //field[row_index][col_index] = String::from("*");
                field[row_index].replace_range(col_index..col_index+1,"*");
                field = sourrounding_area(field, col_index, row_index, height, width);

            }
        }
    }
    field
}
