use std::fs;

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

fn main() {
    let contents = fs::read_to_string(SOURCE_FILE)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n").collect::<Vec<&str>>();
    let n_lines = lines.len();

    let mut bars: Vec<Bar> = Vec::new();
    let columns: Vec<&str> = lines[0].split(",").collect::<Vec<&str>>();
    assert_eq!(columns, ["open", "high", "low", "close", "volume"]);

    for i in 1..n_lines {
        let line = lines[i].split(",").collect::<Vec<&str>>();
        if line.len() == 5 {
            let bar = Bar {
                open: line[0].parse::<f32>().unwrap(),
                high: line[1].parse::<f32>().unwrap(),
                low: line[2].parse::<f32>().unwrap(),
                close: line[3].parse::<f32>().unwrap(),
                volume: line[4].parse::<usize>().unwrap()
            };
            bars.push(bar);
        }
    }

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
