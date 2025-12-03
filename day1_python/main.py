from dial import Dial


def main():
    starting_point = 50
    dial = Dial(starting_point)

    filename = "input_codes.txt" 
    container = []
    with open(filename, 'r') as file:
        for line in file:
            line = line.strip()
            if not line:
                continue
            
            direction = line[0]

            move_amount = int(line[1:]) 
            if direction == "L":
                dial.move_left(move_amount)
            elif direction == "R":
                dial.move_right(move_amount)

    print(f"sum is {dial.value}")
    print(f"password is {dial.zero_count}")
            

    
if __name__ == '__main__':
    main()