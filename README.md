# toybox-rs ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg) ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/CI/badge.svg)
Rust packages and for Toybox; separately versioned from Python code.

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

- ``toybox`` - Contains core logic for games.
- ``ctoybox`` - Contains C API for toybox; and our python code, e.g., Gym environment bindings.

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust ![RustFmt Badge](https://github.com/toybox-rs/toybox-rs/workflows/rustfmt-check/badge.svg)

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.
