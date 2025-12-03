const DIAL_END: i16 = 99;
const DIAL_START: i16 = 0;

static DIAL_SIZE: i16 = DIAL_END - DIAL_START + 1;

pub struct Dial {
    value: i16,
    zero_count: i16,
}

impl Dial {
    pub fn new(starting_point: i16) -> Self {
        Dial {
            value: starting_point,
            zero_count: 0,
        }
    }

    // retrieve sum of values
    pub fn value(&self) -> i16 {
        self.value
    }

    // retrieve number of zeros counted
    pub fn zero_count(&self) -> i16 {
        self.zero_count
    }

    pub fn move_left(&mut self, by: i16) {
        // println!("before move L{by} value = {}", self.value);
        self.value = (self.value - by).rem_euclid(DIAL_SIZE);
        // println!("after move L{by} value = {}", self.value);

        // check if dial lands on zero then increase zero count
        if self.value == 0 {
            self.zero_count += 1;
        }
    }

    pub fn move_right(&mut self, by: i16) {
        // println!("before move R{by} value = {}", self.value);
        self.value = (self.value + by).rem_euclid(DIAL_SIZE);

        // println!("after move R{by} value = {}", self.value);
        // check if dial lands on zero then increase zero count
        if self.value == 0 {
            self.zero_count += 1;
        }
    }
}
