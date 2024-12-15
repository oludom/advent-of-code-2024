import os
from setuptools import setup, find_packages
from pathlib import Path

THIS_DIR = Path(__file__).parent
PACKAGE_DIR = THIS_DIR / "src" / "aoc"

packages = find_packages("src", include="aoc*")
requirements = [
    "pydantic",
]

# use this to add readme file as long description of package
def read(fname):
    return open(os.path.join(os.path.dirname(__file__), fname)).read()

setup(
    name="aoc",
    version="0.0.1",
    author="Micha Heiß",
    author_email="michaheiss@proton.me",
    description= ("Advent of code 2024", "Some days solved in python"),
    license="BSD",
    packages=packages,
    requirements=requirements,
    package_dir={'': 'src'},
    long_description=read("README.md"),
    entry_points= {
        "console_scripts": {
            "aoc = aoc.__main__:main"
        }
    }
)