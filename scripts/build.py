# builds the lambda functions and copies them to the infrastructure folder

import os
import sys

commands = [
    "sudo apt-get install openssl libssl musl-tools pkg-config libssl-dev",
    "OPENSSL_LIB_DIR=\"/usr/lib/x86_64-linux-gnu\"",
    "OPENSSL_INCLUDE_DIR=\"/usr/include/openssl\"",
    "pip3 install cargo-lambda",
    "cargo lambda build --release"
]
os.system("; ".join(commands))

functions = [name for name in os.listdir("target/lambda/")]

commands = ["mkdir -p infrastructure/data/lambdas"]

for function in functions:
    commands.append(f"zip {function}.zip target/lambda/{function}/bootstrap")
    commands.append(f"cp {function}.zip infrastructure/data/lambdas/{function}.zip")
os.system("; ".join(commands))

