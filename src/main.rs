use std::thread;
use std::time::Duration;

// fitness scene
// the "intensity" of customer preference fot fitness

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    //
    // 原本这里有一条固定的计算已经不见了，
    // 那么是到了下面具体分支才开始计算
    //
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 函数type 即是 Fn([type]) -> [type]
