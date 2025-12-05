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
        let prev = self.value;
        let result = self.value - by;

        // println!(
        //     "before move L{by} value = {} zero_count = {}, result = {}",
        //     self.value, self.zero_count, result
        // );

        // Calculate new position
        self.value = result.rem_euclid(DIAL_SIZE);

        // Count zero crossings
        // How many complete counter-clockwise rotations?
        let zeros = by / DIAL_SIZE;
        self.zero_count += zeros;

        // Check if we cross zero in the partial rotation
        // We cross zero if we're not starting at zero AND
        // the remainder of the move is greater than our starting position
        let remainder = by % DIAL_SIZE;

        if prev > 0 && remainder >= prev {
            self.zero_count += 1;
        }
        //
        // println!(
        //     "\tafter move L{by} value = {}, zero_count = {}",
        //     self.value, self.zero_count
        // );
    }

    pub fn move_right(&mut self, by: i16) {
        let result = self.value + by;

        // println!(
        //     "before move R{by} value = {} zero_count = {}, result = {}",
        //     self.value, self.zero_count, result
        // );

        // Calculate new position
        self.value = result.rem_euclid(DIAL_SIZE);

        // Count zero crossings
        // Formula: (current_position + movement) / DIAL_SIZE
        // This counts each time we pass through or land on 0
        let zeros = result / DIAL_SIZE;
        self.zero_count += zeros;

        // println!(
        //     "\tafter move R{by} value = {}, zero_count = {}",
        //     self.value, self.zero_count
        // );
    }
}
