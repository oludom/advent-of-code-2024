from argparse import ArgumentParser

class AocArgParser(ArgumentParser):
    def __init__(self) -> None:
        super().__init__()
        self.add_argument("day", type=str, help="day to execute, like '3' or '3+'")
        self.add_argument("-i", "--input", type=str, help="INPUT: path to file containing input", required=True)