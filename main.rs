use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SOURCE_FILE: &str = "bars.csv";
const TARGET_FILE: &str = "volume_bars.csv";
const OHLCV_COLUMNS: &str = "open,high,low,close,volume";
const BAR_SIZE: usize = 5;

#[derive(Debug)]
struct Bar {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: usize
}

impl Bar {
    fn new() -> Bar {
        Bar {
            open: -1.0,
            high: -1.0,
            low: -1.0,
            close: -1.0,
            volume: 0
        }
    }

    fn print(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
            self.volume.to_string(),
        )
    }
}

fn read_ohlcv(path: &str) -> Vec<Bar> {
    let f = File::open(path)
        .expect("cant read source file");
    let mut reader = BufReader::new(f);

    let mut columns = String::new();
    let len = reader.read_line(&mut columns).unwrap();
    columns.remove(len-1);
    assert_eq!(columns, OHLCV_COLUMNS);

    let mut bars: Vec<Bar> = Vec::new();
    for line in reader.lines() {
        let temp_line = line.unwrap();
        let line_cols: Vec<&str> = temp_line.split(",").collect();
        bars.push(Bar {
            open: line_cols[0].parse::<f32>().unwrap(),
            high: line_cols[1].parse::<f32>().unwrap(),
            low: line_cols[2].parse::<f32>().unwrap(),
            close: line_cols[3].parse::<f32>().unwrap(),
            volume: line_cols[4].parse::<usize>().unwrap()
        });
    }
    bars
}

fn write_ohlcv(path: &str, bars: Vec<Bar>) {
    let columns_row = format!("{}\n", OHLCV_COLUMNS);
    let mut contents = String::from(columns_row);
    for bar in bars.iter().rev() {
        contents.push_str(&bar.print());
    }

    fs::write(path, contents)
        .expect("Unable to write file");
}

fn main() {
    let mut bars = read_ohlcv(SOURCE_FILE);
    let mut volume_bars: Vec<Bar> = Vec::new();
    let mut temp_volume_bar = Bar::new();
    let mut i = bars.len()-1;
    let mut is_ready = false;

    while !is_ready {
        if bars[i].volume > 0 {
            temp_volume_bar.volume = temp_volume_bar.volume+bars[i].volume;
            if temp_volume_bar.volume >= BAR_SIZE {
                bars[i].volume = temp_volume_bar.volume-BAR_SIZE;
                temp_volume_bar.volume = BAR_SIZE;
                temp_volume_bar.close = bars[i].close;
            } else {
                bars[i].volume = 0;
            }
            if temp_volume_bar.open.is_sign_negative() {
                temp_volume_bar.open = bars[i].open;
            }
            if temp_volume_bar.high.is_sign_negative() || bars[i].high > temp_volume_bar.high {
                temp_volume_bar.high = bars[i].high;
            }
            if temp_volume_bar.low.is_sign_negative() || bars[i].low < temp_volume_bar.low {
                temp_volume_bar.low = bars[i].low;
            }
            if temp_volume_bar.volume == BAR_SIZE {
                volume_bars.push(temp_volume_bar);
                temp_volume_bar = Bar::new();
            }
        } else {
            temp_volume_bar.close = bars[i].close;
            if i == 0 {
                is_ready = true;
            } else {
                i = i-1;
            }
        } 
    }
    if temp_volume_bar.volume < BAR_SIZE {
        volume_bars.push(temp_volume_bar);
    }

    write_ohlcv(TARGET_FILE, volume_bars);
}
