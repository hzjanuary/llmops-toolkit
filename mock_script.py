"""Historical filename retained. Runs only the offline evaluation example."""
import subprocess

if __name__ == "__main__":
    raise SystemExit(subprocess.call([
        "cargo", "run", "--locked", "--offline", "--", "eval",
        "--dataset", "examples/dataset.json",
    ]))
