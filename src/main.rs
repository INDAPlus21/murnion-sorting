use ggez::event;
use ggez::event::{MouseButton};
use ggez::timer;
use ggez::graphics::{self, DrawParam, Color, DrawMode};
use ggez::{Context, GameResult};
use std::path;
use std::env;
use std::time::Duration;
use std::num;
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
        let _seed: u64;
        unsafe { _seed = seed; }
        let mut rng: rngs::StdRng = rand::SeedableRng::seed_from_u64(_seed);
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
            println!("{:?}, {:?}", i, color);
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
            "radix" => {
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
            "selection" => {
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
            "insertion" => {
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
                        if (j + i) % 613 == 0 {
                        self.draw(ctx);
                        }
                    }
                }
            },
            "merge" => {
                let mut list1: Vec<Vec<(usize, usize)>> = vec!();
                let mut list2: Vec<Vec<(usize, usize)>> = vec!();

                let mut step = 1;

                let mut n = 0;
                for b in &self.array {
                    let x = vec!((b.0, n));
                    list1.push(x);
                    n += 1;
                }
                while step < ARRAY_SIZE / 2 {
                    let mut i = 0;
    
                    let mut j = 0;
                    let mut k = 0;
                    
                    while i < ARRAY_SIZE {
                        let mut new: Vec<(usize, usize)> = vec!();
                        while j < step {
                            if list1[i+1].len() == 0 {
                                new.append(&mut list1[i]);
                                break;
                            }
                            if list1[i][j].0 < list1[i+1][k].0 {
                                new.push(list1[i][j]);
                                j += 1
                            } else {
                                new.push(list1[i+1][k]);
                                k += 1;
                            }
                        }
                        if list1[i+1].len() != 0 {
                            new.append(&mut list1[i+1]);
                        }
                        list2.push(new);
                        i += step*2;
                    }
                    let mut n = 0;
                    for i in list1 {
                        let mut m = 0;
                        for j in i {
                            let x = self.array.remove(j.1);
                            self.array.insert((n*step) + m, x);
                            m += 1;
                        }
                        n += 1;
                    }
                    self.draw(ctx);
                    step *= 2;
                    list1 = list2;
                    list2 = vec!();
                }
            }
            _ => unimplemented!()
        }
    }
}

pub fn main() -> GameResult {

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
