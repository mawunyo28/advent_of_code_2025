DIAL_START = 0
DIAL_END = 99


class Dial:

    def __init__(self, starting_point: int):
        self.value: int = starting_point
        self.zero_count: int = 0

    def move_left(self, by: int):
        self - by

        if self.value == 0:
            self.zero_count += 1
            # print(f"\t\tzero_count = {self.zero_count}")

    def move_right(self, by: int):
        self + by

        if self.value == 0:
            self.zero_count += 1
            # print(f"\t\tzero_count = {self.zero_count}")

    def __add__(self, other: int):
        result = self.value + other

        # print(f"Current Value: {self.value}, rhs: {other}, before add({result})")
        while result > DIAL_END:
            result = result - (DIAL_END + 1)

        # print(f"\tAfter add = {result}")
        self.value = result

    def __sub__(self, other: int):
        result = self.value - other
        
        # print(f"Current Value: {self.value}, rhs: {other}, before sub({result})")
        
        while result < DIAL_START:
            result = DIAL_END + result + 1
        
        # print(f"\tAfter sub = {result}")
        self.value = result
