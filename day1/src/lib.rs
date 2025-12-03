const DIAL_END: u32 = 99;
const DIAL_START: u32 = 0;

pub struct Dial {
    value: u32,
    zero_count: u32,
}

trait Ops {
    fn add(&mut self, rhs: u32);
    fn sub(&mut self, rhs: u32);
}

impl Dial {
    pub fn new(starting_point: u32) -> Self {
        Dial {
            value: starting_point,
            zero_count: 0,
        }
    }

    // retrieve sum of values
    pub fn value(&self) -> u32 {
        self.value
    }

    // retrieve number of zeros counted
    pub fn zero_count(&self) -> u32 {
        self.zero_count
    }

    pub fn move_left(&mut self, by: u32) {
        self.sub(by);

        // check if dial lands on zero then increase zero count
        if self.value == 0 {
            self.zero_count += 1;
        }
    }

    pub fn move_right(&mut self, by: u32) {
        self.add(by);

        // check if dial lands on zero then increase zero count
        if self.value == 0 {
            self.zero_count += 1;
        }
    }
}

impl Ops for Dial {
    fn add(&mut self, rhs: u32) {
        let mut result = self.value + rhs;
        //
        // println!(
        //     "Current Value: {}, rhs: {}, before add(result = {})",
        //     self.value, rhs, result
        // );

        while result > DIAL_END {
            // zero inclusize
            result = result - (DIAL_END + 1);
        }

        // println!("\tAfter add(result = {result})");
        self.value = result;
    }

    fn sub(&mut self, rhs: u32) {
        let mut result = self.value as i32 - rhs as i32;

        // println!(
        //     "Current Value: {}, rhs: {}, before sub(result = {})",
        //     self.value, rhs, result
        // );
        //
        while result < (DIAL_START as i32) {
            // result is alread negative
            // zero inclusize that is why there is + 1
            result = (DIAL_END as i32) + result + 1;
        }

        // println!("\tAfter sub(result = {result})");

        self.value = result as u32;
    }
}
