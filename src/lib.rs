extern crate url;
extern crate wasi;

use std::fmt;
use url::Url;
use wasm_bindgen::prelude::*;
use md5;
use wasm_bindgen::__rt::std::collections::HashMap;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Universe {
    run: u32,
    count: u32,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    digests_identical: u32,
    previous_digest: String,
    digest_history: Vec<String>,
    digest_history_count: HashMap<String, i32>,
    digest_history_retention: usize,
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self) -> String {
        let mut next = self.cells.clone();
        let mut total_alive: u32 = 0;
        let return_value: String;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                match cell {
                    Cell::Alive => {
                        total_alive += 1;
                    }
                    _ => (),
                }
                let live_neighbours = self.live_neighbour_count(row, col);
                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }

        if self.recurring_pattern_present(self.get_md5_sum().as_str()) {
            return_value = String::from("stop");
        } else {
            return_value = String::from("continue");
        }

        self.cells = next;

        self.run += 1;
        self.count = total_alive;
        rest_call_runcount(self.run, total_alive);

        return_value
    }
    pub fn new(
        width: u32,
        height: u32,
        m1: u32,
        m2: u32,
        digest_history_retention: usize,
    ) -> Universe {
        let run = 0;
        let count: u32 = 0;
        let width: u32 = width;
        let height: u32 = height;
        let digests_identical: u32 = 0;

        let cells = (0..width * height)
            .map(|i| {
                if i % m1 == 0 || i % m2 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            run,
            count,
            width,
            height,
            cells,
            digests_identical,
            previous_digest: String::from(""),
            digest_history_retention: digest_history_retention,
            digest_history: vec!(),
            digest_history_count: HashMap::with_capacity(30),
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }


    fn get_md5_sum(&self) -> String {
        let digest = md5::compute(self.to_string().as_bytes());
        format!("{:x}", digest).to_string()
    }

    fn recurring_pattern_present(&mut self, digest: &str) -> bool {

        self.digest_history.push(String::from(digest.clone()));
        if self.digest_history.len() > self.digest_history_retention {
            let keys_to_hold = self.digest_history.split_off(1);
            for key in &self.digest_history {
                self.digest_history_count.remove(key.as_str());
            }
            self.digest_history = keys_to_hold;
        }
        if let Some(x) = self.digest_history_count.get_mut(digest) {
            *x = *x + 1;
        } else {

            self.digest_history_count.insert(digest.to_string(), 1);
        }
        let mut digests_with_high_occurence: usize = 0;
        for (_key, value) in &self.digest_history_count {
            if value >= &20 {
                digests_with_high_occurence = digests_with_high_occurence + 1;
            }
        }
        log(&*format!("{:?}", self.digest_history_count));
        if digests_with_high_occurence >=2 || self.digest_history_count.values().max() >= Some(&40) {
            return true;
        }
        return false

    }

}

fn rest_call_runcount(run: u32, count: u32) {
    let result = Url::parse(format!("http://127.0.0.1:3000/runcount/{}/{}", run, count).as_str());
    match result {
        Ok(x) => {
            println!("{}", x);
        }
        _ => {
            println!("There was an error");
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
