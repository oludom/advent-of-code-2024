from .parser import AocArgParser
from .days import day3

def main():
    print("Welcome to the Advent of Code.")

    parser = AocArgParser()
    args = parser.parse_args()

    print(f"selected day {args.day}")

    with open(args.input, "r") as f:
        data = "".join(f.readlines())

        match args.day:
            case 3:
                day3.run(data)
            case _:
                print("the selected day has not been solved yet.")