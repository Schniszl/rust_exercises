extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::convert::From;
use core::iter::Iterator;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field : Vec<String> = Vec::new();
    if minefield.is_empty() {
        return field
    }

    let rows = minefield.len();
    let cols = if rows > 0 { minefield[0].len() } else { 0 };

    field = vec![String::from(" ".repeat(minefield[0].len()));minefield.len()];

    for (row_index, row) in minefield.iter().enumerate() {
        for(col_index, chr) in row.chars().enumerate(){
            if chr == '*' {
                field =mine_found(field, row_index, col_index,rows,cols);

            }
        }
    }
    field
}

fn mine_found(mut field: Vec<String>, row_index: usize,col_index: usize, rows: usize, cols: usize ) -> Vec<String> {
    // Leichter zugriff auf die Nachbarknoten
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    field[row_index].replace_range(col_index..col_index+1,"*");
    for (iter_row, iter_col) in directions {
        let ni = row_index as isize + iter_row;
        let nj = col_index as isize + iter_col;

        if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
            //// Konvertiere ni und nj in unsigned, da vektoren positiv sein müssen
            let (ni,nj) = (ni as usize, nj as usize);
            let c = field[ni].chars().nth(nj).unwrap();

            if c != '*' {
                if c.is_digit(10){
                    let n = c.to_digit(10).unwrap() + 1;
                    field[ni].replace_range(nj..nj+1,n.to_string().as_str());

                } else {
                    field[ni].replace_range(nj..nj+1,"1");
                }
            }
        }
    }
    field
}

