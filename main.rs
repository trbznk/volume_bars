use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SOURCE_FILE: &str = "bars.csv";
const TARGET_FILE: &str = "volume_bars.csv";
const BAR_SIZE: usize = 5;

#[derive(Debug)]
struct Bar {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: usize
}

fn read_ohlcv(path: &str) -> Vec<Bar> {
    let f = File::open(path)
        .expect("cant read source file");
    let mut reader = BufReader::new(f);

    let mut columns = String::new();
    let len = reader.read_line(&mut columns).unwrap();
    columns.remove(len-1);
    assert_eq!(columns, "open,high,low,close,volume");

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

fn main() {
    let mut bars = read_ohlcv(SOURCE_FILE);
    let mut volume_bars: Vec<Bar> = Vec::new();
    let (mut open, mut high, mut low, mut close): (f32, f32, f32 , f32) = (-1.0, -1.0, -1.0, -1.0);
    let mut volume: usize = 0;
    let mut i = 0;

    while i < bars.len() {
        if bars[i].volume > 0 {
            volume = volume+bars[i].volume;
            if volume >= BAR_SIZE {
                bars[i].volume = volume-BAR_SIZE;
                volume = BAR_SIZE;
                close = bars[i].close;
            } else {
                bars[i].volume = 0;
            }
            if open.is_sign_negative() {
                open = bars[i].open;
            }
            if high.is_sign_negative() || bars[i].high > high {
                high = bars[i].high;
            }
            if low.is_sign_negative() || bars[i].low < low {
                low = bars[i].low;
            }
    
            if volume == BAR_SIZE {
                let volume_bar = Bar {
                    open: open,
                    high: high,
                    low: low,
                    close: close,
                    volume: volume
                };
                volume_bars.push(volume_bar);
                open = -1.0;
                high = -1.0;
                low = -1.0;
                close = -1.0;
                volume = 0;
            }
        } else {
            close = bars[i].close;
            i = i+1;
        } 
    }
    if volume < BAR_SIZE {
        let volume_bar = Bar {
            open: open,
            high: high,
            low: low,
            close: close,
            volume: volume
        };
        volume_bars.push(volume_bar);
    }

    let mut contents = String::from("open,high,low,close,volume\n");
    for bar in volume_bars.iter() {
        let row = format!(
            "{},{},{},{},{}\n",
            bar.open.to_string(),
            bar.high.to_string(),
            bar.low.to_string(),
            bar.close.to_string(),
            bar.volume.to_string(),
        );
        contents.push_str(&row);
    }

    fs::write(TARGET_FILE, contents)
        .expect("Unable to write file");
}
