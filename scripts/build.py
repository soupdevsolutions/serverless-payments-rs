# builds the lambda functions and copies them to the infrastructure folder
import os
import sys

commands = [
    "pip3 install cargo-lambda",
    "cargo lambda build --release --output-format zip"
]
os.system("; ".join(commands))

functions = [name for name in os.listdir("target/lambda/")]

commands.append("mkdir -p infrastructure/data/lambdas")

for function in functions:
    commands.append(f"cp target/lambda/{function}/bootstrap.zip infrastructure/data/lambdas/{function}.zip")
os.system("; ".join(commands))

