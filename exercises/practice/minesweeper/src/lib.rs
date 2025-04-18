extern crate alloc;

use alloc::string::{String};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::Iterator;

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    if minefield.is_empty() {
        return vec![]
    }

    let rows = minefield.len();
    let cols = if rows > 0 { minefield[0].len() } else { 0 };

    // Erstelle eine liste über liste mit chars
    // let mut grid: Vec<Vec<char>> = minefield.iter().map(|s| s.chars().collect()).collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in minefield {
        let chars: Vec<char> = line.chars().collect(); // zerlege Zeile in Zeichen
        grid.push(chars); // füge die Zeichenzeile der Liste hinzu
    }

    // Leichter zugriff auf die Nachbarknoten
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            // Wenn Mine gefunden wurde
            if grid[i][j] == '*' {

                for (iter_row, iter_col) in directions {
                    // errechne den nachbar wert ->isize, da auch negative werte vorkommen
                    let ni = i as isize + iter_row;
                    let nj = j as isize + iter_col;
                    // prüfe ob es ein nachbarn gibt
                    if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                        // Konvertiere ni und nj in unsigned, da vektoren positiv sein müssen
                        let (ni, nj) = (ni as usize, nj as usize);

                        // Welche Nachbarn habe ich
                        if grid[ni][nj] != '*' {
                            let c = grid[ni][nj];
                            if c.is_ascii_digit() {
                                let n = c.to_digit(10).unwrap() + 1;
                                grid[ni][nj] = std::char::from_digit(n, 10).unwrap();
                            } else {
                                grid[ni][nj] = '1';
                            }
                        }
                    }
                }
            }
        }
    }
    //Konvertiere die Liste über Liste wieder in eine Listvon Strings
    let mut result = Vec::new();

    for row in grid {
        let mut row_string = String::new();
        for ch in row {
            row_string.push(ch); // füge jedes Zeichen zum String hinzu
        }
        result.push(row_string); // füge die fertige Zeile zum Ergebnis hinzu
    }
    result

    // Mach aus der Liste über Liste mit Zeichen eine Liste von Strings
    // grid.into_iter().map(|row| row.into_iter().collect()).collect()
}


