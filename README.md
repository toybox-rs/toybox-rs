
# toybox-rs ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg) ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/CI/badge.svg)
Rust packages and for Toybox; separately versioned from the main project (i.e., Python API): [toybox-rs/Toybox](https://github.com/toybox-rs/Toybox)

<p align="center"><img width="100" height="100" alt="Toybox Logo" src="http://toybox.rs/toybox-logo.svg" />
</p>

## How do I get it? [![PyPI version](https://badge.fury.io/py/ctoybox.svg)](https://badge.fury.io/py/ctoybox)

```bash
pip install ctoybox
pip install pygame # optional dependency
python -m ctoybox.human_play amidar
```

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
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg)

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.
