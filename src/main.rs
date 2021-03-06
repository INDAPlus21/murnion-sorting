use ggez::event;
use ggez::event::{MouseButton};
use ggez::timer;
use ggez::graphics::{self, DrawParam, Color, DrawMode};
use ggez::{Context, GameResult};
use std::path;
use std::env;
use std::time::Duration;
use std::num;
use std::fs::File;
use std::io::BufReader;
use std::collections::BinaryHeap;
use chrono::DateTime;
use chrono::Utc;
use chrono::Timelike;
use rand::Rng;
use rand::rngs;
use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

static mut seed: u64 = 0;

const ARRAY_SIZE: usize = 1000;

const SCREEN_SIZE: (f32, f32) = (
    ((1000 / ARRAY_SIZE) * ARRAY_SIZE) as f32,
    720f32,
);

struct AppState {
    array: Vec<(usize, graphics::Mesh)>,
}

impl AppState { 
    //initialize the application here
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let cur = Utc::now().timestamp_millis();
        unsafe { seed = cur as u64; seed += 1; }
        let mut rng: rngs::StdRng = rand::SeedableRng::seed_from_u64(cur as u64);
        let mut array = vec!();
        for i in 0..ARRAY_SIZE {
            let color: Color = int_to_rgba(i, ARRAY_SIZE).into();
            let slice = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), 
                graphics::Rect::new_i32(0, 
                                        0, 
                                        (1000/ARRAY_SIZE) as i32, 
                                        720), 
                                        color)?;
            array.push((i, slice));
        }
        array.shuffle(&mut rng);

        let state = AppState {
            array: array,
        };

        Ok(state)
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //nothing here, i think
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //where the application draws the visualization
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        for i in 0..ARRAY_SIZE {
            graphics::draw(ctx, &self.array[i].1, (ggez::mint::Point2 { x: (i * (1000/ARRAY_SIZE)) as f32, y: 0.0 }, ));
        };
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        const VEC: Vec<usize> = vec!();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let args: Vec<String> = env::args().collect();
        match args[1].to_lowercase().as_str() {
            "radix sort" => {
                let mut third_digit_array = [VEC; 10];
                let mut second_digit_array = [VEC; 10];
                let mut first_digit_array = [VEC; 10];
                for i in 0..ARRAY_SIZE {
                    if i % 10 == 0 { self.draw(ctx); }
                    let mut digits: Vec<_> = self.array[i].0.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
                    while digits.len() < 3 { digits.insert(0, 0); }
                    macro_rules! digits_sort {
                        (digit: $digit:expr) => {
                            let mut idx = 0;
                            for j in 0..($digit as usize) {
                                idx += third_digit_array[j].len();
                            }
                            third_digit_array[$digit as usize].push(self.array[i].0);
                            let source = SineWave::new(self.array[i].0 as u32 + ((idx as i32 - self.array[i].0 as i32)).abs() as u32).take_duration(Duration::from_secs_f32(0.01)).amplify(0.1);
                            sink.append(source);
                            let x = self.array.remove(i);
                            self.array.insert(idx, x);
                        };
                    }
                    digits_sort!(digit: digits[2]);
                }
                for i in 0..10 {
                    for j in 0..third_digit_array[i].len() {
                        if j % 10 == 0 { self.draw(ctx); }
                        let mut idx = j;
                        if i != 0 { for k in 0..i { idx += third_digit_array[k].len(); }}
                        let mut digits: Vec<_> = self.array[idx].0.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
                        while digits.len() < 3 { digits.insert(0, 0); }
                        macro_rules! digits_sort {
                            (digit: $digit:expr) => {
                                let mut _idx = 0;
                                for j in 0..($digit as usize) {
                                    _idx += second_digit_array[j].len();
                                }
                                second_digit_array[$digit as usize].push(self.array[idx].0);
                                let source = SineWave::new(self.array[idx].0 as u32 + ((_idx as i32 - self.array[idx].0 as i32)).abs() as u32).take_duration(Duration::from_secs_f32(0.01)).amplify(0.1);
                                sink.append(source);
                                let x = self.array.remove(idx);
                                self.array.insert(_idx, x);
                            };
                        }
                    digits_sort!(digit: 9 - digits[1]);
                    }
                }
                sink.stop();
                let sink = Sink::try_new(&stream_handle).unwrap();
                for i in 0..10 {
                    for j in 0..second_digit_array[i].len() {
                        if j % 10 == 0 { self.draw(ctx); }
                        let mut idx = j;
                        if i != 0 { for k in 0..i { idx += second_digit_array[k].len(); }}
                        let mut digits: Vec<_> = self.array[idx].0.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
                        while digits.len() < 3 { digits.insert(0, 0); }
                        macro_rules! digits_sort {
                            (digit: $digit:expr) => {
                                let mut _idx = 0;
                                for j in 0..($digit as usize) {
                                    _idx += first_digit_array[j].len();
                                }
                                first_digit_array[$digit as usize].push(self.array[idx].0);

                                //practically speaking im just forcing the radix to sound nice
                                //but im so tired of scouring "sound of sorting" videos for how they did it
                                //their code SUCKS
                                //let source = SineWave::new(self.array[idx].0 as u32 + ((_idx as i32 - self.array[idx].0 as i32)).abs() as u32).take_duration(Duration::from_secs_f32(0.001)).amplify(0.1);
                                let source = SineWave::new((first_digit_array[$digit as usize].len() * 30) as u32).take_duration(Duration::from_secs_f32(0.001)).amplify(0.1);
                                sink.append(source);
                                let x = self.array.remove(idx);
                                self.array.insert(_idx, x);
                            };
                        }
                        digits_sort!(digit: digits[0]);
                    }
                }
            },
            "selection sort" => {
                for i in 0..ARRAY_SIZE {
                    let mut pos = i;
                    let mut smallest = self.array[i].0;
                    for j in i..ARRAY_SIZE {
                            let source = SineWave::new((300 + self.array[j].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            let source = SineWave::new((300 + self.array[pos].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                        if self.array[j].0 < smallest {
                            pos = j;
                            smallest = self.array[j].0;
                        }
                    }
                    let x = self.array.remove(pos);
                    self.array.insert(i, x);
                    if i % 10 == 0 { self.draw(ctx); }
                }
            },
            "insertion sort" => {
                let _seed;
                unsafe { _seed = seed; seed += 1; }
                let mut rng: rngs::StdRng = rand::SeedableRng::seed_from_u64(_seed);
                for i in 0..ARRAY_SIZE {
                    let mut j = i;
                    while j > 0 && self.array[j].0 < self.array[j-1].0 {
                            let source = SineWave::new((300 + self.array[j].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            let source = SineWave::new((300 + self.array[j - 1].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                        let x = self.array.remove(j);
                        self.array.insert(j - 1, x);
                        j -= 1;
                        if (rng.gen::<usize>()) % 600 == 0 {
                        self.draw(ctx);
                        }
                    }
                }
            },
            "merge sort" => {
                let x = merge(stream_handle, self, ctx, self.array.clone(), 0);

                fn merge(sink: rodio::OutputStreamHandle, drawer: &mut AppState, ctx: &mut Context, mut list: Vec<(usize, graphics::Mesh)>, index: usize) -> Vec<(usize, graphics::Mesh)> {
                    if list.len() == 1 { return list; }
            
                    let mut middle = list.len() / 2;
                    let mut left = list;
                    let mut right = left.split_off(middle);
                
                    left = merge(sink.clone(), drawer, ctx, left, index);
                    right = merge(sink.clone(), drawer, ctx, right, (index + middle));

                    let _sink = Sink::try_new(&sink).unwrap();
                
                    let mut new = vec!();
                    while left.len() > 0 && right.len() > 0 {
                        if left[0].0 < right[0].0 {
                            let x = left.remove(0);
                            let source = SineWave::new(300 + x.0 as u32).take_duration(Duration::from_secs_f32(0.01)).amplify(0.1);
                            _sink.append(source);
                            new.push(x);
                        } else {
                            let x = right.remove(0);
                            let source = SineWave::new(300 + x.0 as u32).take_duration(Duration::from_secs_f32(0.01)).amplify(0.1);
                            _sink.append(source);
                            new.push(x);
                        }
                    }
                    if left.len() > 0 {
                        new.append(&mut left);
                    }
                    if right.len() > 0 {
                        new.append(&mut right);
                    }
                    
                    for i in 0..new.len() {
                        drawer.array[i + index] = new[i].clone();
                    }
                    if index % 5 == 0 { drawer.draw(ctx); }
            
                    new
                }
                
                self.array = x;
            },
            "stalin sort" => {
                let mut i = 1;
                let mut j = 1;
                while i < ARRAY_SIZE {
                    if self.array[j-1].0 > self.array[j].0 {
                        let mut color = int_to_rgba(self.array[j].0, ARRAY_SIZE);
                        color[0] = std::cmp::max(((color[0]) * 255.0) as i32 - 77, 0) as f32 / 255.0;
                        color[1] = color[0];
                        color[2] = color[0];
                        let slice = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), 
                            graphics::Rect::new_i32(0, 
                                                    0, 
                                                    (1000/ARRAY_SIZE) as i32, 
                                                    720), 
                                                    color.into());
                        self.array[j].1 = slice.unwrap();
                        let x = self.array.remove(j);
                        self.array.push(x);
                        if i % 5 == 0 { 
                            let file = BufReader::new(File::open("resources/gunshot.wav").unwrap());
                            let source = Decoder::new(file).unwrap();
                            stream_handle.play_raw(source.convert_samples());
                            self.draw(ctx); 
                        }
                    } else {
                        j += 1;
                    }
                    i += 1;
                }
                sink.stop();
            },
            "bogosort" => {
                let _seed;
                unsafe { _seed = seed; seed += 1; }
                let mut rng: rngs::StdRng = rand::SeedableRng::seed_from_u64(_seed);
                for i in 0..ARRAY_SIZE * 10 {
                    for j in 1..ARRAY_SIZE {
                        let source = SineWave::new((300 + self.array[j].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                        sink.append(source);
                        let source = SineWave::new((300 + self.array[j - 1].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                        sink.append(source);
                        if self.array[j - 1].0 > self.array[j].0 {
                            break;
                        }
                        if i % 100 == 0 {
                            self.draw(ctx);
                        }
                    }
                    self.array.shuffle(&mut rng);
                }
            },
            "quantum bogosort" => {
                self.array.sort_by_key(|x| x.0);
                let mut sinks: Vec<rodio::Sink> = vec!();
                for i in 0..5 {
                    let x = Sink::try_new(&stream_handle).unwrap();
                    sinks.push(x);
                }
                let source1 = SineWave::new((200 + ARRAY_SIZE / 2) as u32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.1);
                sinks[0].append(source1);
                let source2 = SineWave::new((250 + ARRAY_SIZE / 2) as u32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.1);
                sinks[1].append(source2);
                let source3 = SineWave::new((300 + ARRAY_SIZE / 2) as u32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.1);
                sinks[2].append(source3);
                let source4 = SineWave::new((350 + ARRAY_SIZE / 2) as u32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.1);
                sinks[3].append(source4);
                let source5 = SineWave::new((400 + ARRAY_SIZE / 2) as u32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.1);
                sinks[4].append(source5);
                sinks[4].sleep_until_end();
            },
            "bubble sort" => {
                let mut i = 1;
                let mut sorted = true;
                while i < ARRAY_SIZE {
                    if self.array[i - 1].0 > self.array[i].0 {
                        let source = SineWave::new((300 + self.array[i].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                        sink.append(source);
                        let source = SineWave::new((300 + self.array[i - 1].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                        sink.append(source);
                        sorted = false;
                        let x = self.array.remove(i);
                        self.array.insert(i - 1, x);
                    }
                    if i % 911 == 0 {
                        self.draw(ctx);
                    }
                    i += 1;
                    if i == ARRAY_SIZE && !sorted {
                        i = 1;
                        sorted = true;
                    }
                }
            },
            "odd-even sort" => {
                let mut sorted = false;
                while (!sorted) {
                    sorted = true;
                    for i in 0..ARRAY_SIZE - 1 {
                        if self.array[i].0 > self.array[i + 1].0 && i % 2 == 0 {
                            let source = SineWave::new((300 + self.array[i].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            let source = SineWave::new((300 + self.array[i + 1].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            sorted = false;
                            let x = self.array.remove(i);
                            self.array.insert(i + 1, x);
                        }
                        if i % 991 == 0 {
                            self.draw(ctx);
                        }
                    }
                    let mut i = ARRAY_SIZE - 1;
                    while i > 0 {
                        i -= 1;
                        if self.array[i].0 > self.array[i + 1].0 && i % 2 == 1 {
                            let source = SineWave::new((300 + self.array[i].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            let source = SineWave::new((300 + self.array[i + 1].0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                            sink.append(source);
                            sorted = false;
                            let x = self.array.remove(i);
                            self.array.insert(i + 1, x);
                        }
                    }
                }
            },
            "heap sort" => {
                let mut heap: BinaryHeap<(usize, usize)> = BinaryHeap::new();
                for i in 0..ARRAY_SIZE {
                    heap.push((self.array[i].0, i))
                }
                for i in 0..ARRAY_SIZE {
                    let mut _heap = heap.clone();
                    let y = _heap.pop().unwrap();
                    let mut vec_heap = _heap.into_vec();
                    for i in 0..vec_heap.len() {
                        if vec_heap[i].1 > y.1 {
                            vec_heap[i].1 -= 1;
                        }
                    }
                    heap = BinaryHeap::from(vec_heap);
                    let x = self.array.remove(y.1);
                    let source = SineWave::new((x.0) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                    sink.append(source);
                    self.array.insert(y.0, x);
                    if i % 10 == 0 {
                        self.draw(ctx);
                    }
                }
            },
            "pancake sort" => {
                for i in 0..ARRAY_SIZE {
                    let mut smallest = 10000;
                    let mut pos = 0;
                    for j in i..ARRAY_SIZE {
                        if self.array[j].0 < smallest {
                            smallest = self.array[j].0;
                            pos = j;
                        }
                    }
                    let source = SineWave::new((400 + smallest * 2) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                    sink.append(source);
                    let mut new = self.array.split_off(pos);
                    new.reverse();
                    self.array.append(&mut new);
                    if i % 23 == 0 { self.draw(ctx); }
                    let mut new = self.array.split_off(i);
                    new.reverse();
                    self.array.append(&mut new);
                    if i % 23 == 0 { self.draw(ctx); }
                }
            },
            "counting sort" => {
                let mut ordered = vec!(vec!(); 1000);
                for i in self.array.clone() {
                    let source = SineWave::new(i.0 as u32).take_duration(Duration::from_secs_f32(0.01)).amplify(0.1);
                    sink.append(source);
                    if i.0 % 23 == 0 { sink.sleep_until_end(); }
                    ordered[i.0].push(i);
                }
                for i in 0..ordered.len() {
                    let x = ordered[i].pop().unwrap();
                    let source = SineWave::new((400 + x.0 * 2) as u32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.1);
                    sink.append(source);
                    if i % 10 == 0 { self.draw(ctx); }
                    self.array[i] = x;
                }
            }
            _ => unimplemented!()
        }
    }
}

fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ggez::ContextBuilder::new("visualiser", "felix")
        .add_resource_path(resource_dir)        // Import image files to GGEZ
        .window_setup(
            ggez::conf::WindowSetup::default()  
                .title("Visualiser")                // Set window title "Schack"
                .icon("/all-seeing-eye.ico")              // Set application icon
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                .resizable(false)               // Fixate window size
        );
    let (contex, event_loop) = &mut context_builder.build()?;

    let state = &mut AppState::new(contex)?;
    event::run(contex, event_loop, state)       // Run window event loop
}

fn int_to_rgba(i: usize, max: usize) -> [f32; 4] {
    let x = i as f64 / max as f64;
    let red = ((2.4 * x + 0.0).sin() * 127.0 + 128.0) / 255.0;
    let green = ((2.4 * x + 2.0).sin() * 127.0 + 128.0) / 255.0;
    let blue = ((2.4 * x + 4.0).sin() * 127.0 + 128.0) / 255.0;
    [red as f32, green as f32, blue as f32, 1.0]
}
