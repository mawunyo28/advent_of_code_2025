DIAL_START = 0
DIAL_END = 99
DIAL_SIZE = DIAL_END - DIAL_START + 1


class Dial:
    def __init__(self, starting_point: int):
        self.value: int = starting_point
        self.zero_count: int = 0

    def move_left(self, by: int):
        self.value = (self.value - by) % DIAL_SIZE
        if self.value == 0:
            self.zero_count += 1
            # print(f"\t\tzero_count = {self.zero_count}")

    def move_right(self, by: int):
        self.value = (self.value + by) % DIAL_SIZE

        if self.value == 0:
            self.zero_count += 1
            # print(f"\t\tzero_count = {self.zero_count}")
