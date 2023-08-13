# builds the lambda functions and copies them to the infrastructure folder

import os
import sys

commands = [
    "sudo apt install musl-tools",
    "sudo apt install pkg-config",
    "sudo apt install libssl-dev",
    "pip3 install cargo-lambda",
    "cargo lambda build --release"
]
os.system("; ".join(commands))

functions = [name for name in os.listdir("target/lambda/")]

commands = [f"cp target/lambda/{function}/bootstrap infrastructure/data/lambdas/{function}" for function in functions]
os.system("; ".join(commands))

