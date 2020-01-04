# cToybox Documentation

<img style="float:right" width="128" height="128" alt="Toybox Logo" src="http://toybox.rs/toybox-logo.svg" />


This package ``ctoybox`` is separately versioned from the main project (i.e., Python API): [toybox-rs/Toybox](https://github.com/toybox-rs/Toybox). It presents the minimal, safe, python interface to the games through the ``Toybox`` and ``Input`` classes.


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