
#  toybox-rs <img width="40" height="40" alt="Toybox Logo" src="http://toybox.rs/toybox-logo.svg" />
![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg) ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/CI/badge.svg) 

Rust packages and for Toybox; separately versioned from the main project (i.e., Python API): [toybox-rs/Toybox](https://github.com/toybox-rs/Toybox)


## How do I get it? [![PyPI version](https://badge.fury.io/py/ctoybox.svg)](https://badge.fury.io/py/ctoybox)

```bash
pip install ctoybox
pip install pygame # optional dependency
python -m ctoybox.human_play amidar

# play the game, and watch the position of enemy 2 (index starts at zero):
python3 -m ctoybox.human_play amidar --query='.state.enemies[2].position.y'
```

## More getting started help:

Check out our getting-started Jupyter Notebook [hosted by Google Colab](https://colab.research.google.com/drive/1MMPseXpJ5esIXwTcHo0EBSG2iLaR0JPB) which renders images from our three implemented games.

Preliminary API documentation is hosted here: [toybox.rs/docs/ctoybox](https://toybox.rs/docs/ctoybox).

## What is Toybox?

A set of games designed for testing deep RL agents.

If you use this code, or otherwise are inspired by our white-box testing approach, please cite our [NeurIPS workshop paper](https://arxiv.org/abs/1812.02850):

```
@inproceedings{foley2018toybox,
  title={{Toybox: Better Atari Environments for Testing Reinforcement Learning Agents}},
  author={Foley, John and Tosch, Emma and Clary, Kaleigh and Jensen, David},
  booktitle={{NeurIPS 2018 Workshop on Systems for ML}},
  year={2018}
}
```

## How do I edit, develop & test?

1. Edit the rust or python code.
2. Create a virtualenv for your modified ctoybox.
3. Compile and install locally to the virtualenv. ``cd ctoybox && ./test.sh``

When your changes are ready, make sure you've documented your changes, and submit a pull-request.

## Projects

- ``core`` - Contains core logic for games; colors, rendering, simple physics, etc.
- ``tb_amidar`` - Contains our Amidar simulator.
- ``tb_breakout`` - Contains our Breakout simulator.
- ``tb_spaceinvaders`` - Contains our SpaceInvaders simulator.
- ``tb_gridworld`` - Contains our configurable GridWorld environment.
- ``ctoybox`` - Contains C API for toybox; and our python code but no Gym bindings -- we want to have python code here that rarely changes.

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* `cargo install cargo-script`
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg)

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.

## Release Instructions

1. Submit a pull request with the version number in ``ctoybox/Cargo.toml`` -> this is version shown on PyPI.
2. Remember that any change to internal representations is a public API change and requires a major version. Before 1.0, it's OK to use the second number for this, i.e., 0.3.0 -> 0.4.0, afterwards we will need to use the major version 1.3.1 -> 2.0.0, because semver.
3. When merged, create a "Release" through Github's graphical API whose name corresponds to the version number.
