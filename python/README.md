# Jupyter notebooks

The plotting ecosystem in Rust has still a long way to go and it's much easier to produce nicely looking
plots using the existing Python tooling.

## Installation

How can you run these Jupyter notebooks?

### Using Docker

Build the docker image:
```bash
docker build -f Dockerfile -t notebook .
```

Run it, mounting the current directory as a volume:
```bash
docker run -p 8888:8888 -v $PWD:/home/jovyan/work -t notebook
```

You can launch the docker image the first time you need it and, given that the local filesystem is mounted
as a volume, the new `*.npy` will appear in Jupyter as soon as they are generated from the koans.

### Using docker-compose

Running
```bash
docker-compose up
```
should be enough to bootstrap your environment!

Just open in a browser the link shown in the console output to enter your Jupyter environment.


### Using pipenv

You can install the required packages in a virtual environment with:
```bash
pip install pipenv
pipenv sync --dev
```

Then follow these steps to use your virtual environment in Jupyter: https://stackoverflow.com/a/47296960

### Using pip

You can install the required packages with:
```bash
pip install -r requirements.txt
```