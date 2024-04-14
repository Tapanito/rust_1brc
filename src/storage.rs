use std::{collections::HashMap, fmt::Display};

// Min, Max, Acc, Counter
pub struct Entry(f64, f64, f64, i32);

impl Entry {
    fn mean(&self) -> f64 {
        self.2 / self.3 as f64
    }

    fn update(&mut self, temp: f64) {
        if temp < self.0 {
            self.0 = temp;
        }
        if temp > self.1 {
            self.1 = temp;
        }

        self.2 += temp;
        self.3 += 1;
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self(
            f64::INFINITY,
            f64::NEG_INFINITY,
            Default::default(),
            Default::default(),
        )
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{} ,{}", self.0, self.1, self.3, self.mean())
    }
}

pub struct Storage {
    map: HashMap<String, Entry>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, city: String, temp: f64) {
        self.map.entry(city).or_default().update(temp);
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec: Vec<_> = self.map.iter().collect();
        vec.sort_unstable_by(|x, y| x.0.cmp(y.0));

        let mut str = String::with_capacity(10 * 1024);
        vec.iter()
            .for_each(|pair| str.push_str(format!("{{{} {}}}\n", pair.0, pair.1).as_str()));

        write!(f, "{}", str)
    }
}
