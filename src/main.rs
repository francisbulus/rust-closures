use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::hash::Hash;
fn main() {
    fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive_result = Cacher::new(|&num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });
    
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
    generate_workout(10, 7);
}


#[derive()]
struct Cacher<F, K, V>
{
    calculation: F,
    cache: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(&K) -> V,
    K: Hash + Eq,
{
    fn new(calculation: F) -> Self {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        use std::collections::hash_map::Entry;

        match self.cache.entry(arg) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => {
                let value = (self.calculation)(vacant.key());
                vacant.insert(value)
            }
        }
    }
}