# Developing New Games

## Initial Setup
1. Run `cargo build`
1. Run `cargo run --bin newgame <gamename>` in the top level of the repository. 
2. Run `cargo build`

`tb_<gamename>/src` should now have three files in it: `lib.rs`, `types.rs`, and `<gamename>.rs` and there should be no errors. 

All of the work you will now be doing is in the `tb_<gamename>` directory, and mainly in the `lib.rs` file. You can grep through 
the file for "TODO" to find the methods you will need to implement/customize. 

### Removing a game
If, for some reason, you want to nuke the game you just created, you remove it with the command: `cargo run --bin newgame <gamename> --clear`.

## Basic Game Components

__`lib.rs`__

`lib.rs` is the top-level API. Only modules declared here will be exposed to other modules.

Every `lib.rs` will also have the following declarations:

```
mod types;
mod <gamename>;
```

You will export other structures as needed. We recommend that you update this on an as-need basis -- Rust will tell you when it can't find a module, and you should use this to guide when something needs to be exported. 

__`<types>.rs`__

The types module contains all of the intervenable structs. You should update this file concurrently with `<gamename>.rs` (we recommend having both open at the same time).

Nearly every struct will use the macros: `#[derive(Debug, Clone, Serialize, Deserialize)]`. The following are the necessary structs for every game:

`<GameName>`: This struct should be structurally the same as the Config object. It is used to instantiate the game and is required for restarting the game. It contains the initial values of fields that are updated during gameplay.

`StateCore`: This struct contains any per-frame state snapshots. It will have duplicated fields from `<GameName>` if those fields are updated during gameplay.

`State`: This struct will have the form:

```
pub struct State {
  pub config: `GameName`,
  pub state: StateCore
}
```


__`<gamename>.rs`__

_Necessary components_

* Default instantiation: you will need to specify what the default instantiation of the game looks like: 
```
impl Default for <GameName> {
  fn default() -> Self {
    // any needed computation
    <GameName> {
      // set values here
    }
  }
}
```
While magic numbers are okay, a default module (e.g., `mod default`) is better. Including that default module in `types.rs` and the `<GameName>` configuration struct is even better.

* `impl`s with `new` methods on each of the structs defined in `types.rs`

* `impl toybox_core::Simulation for <GameName>`. The documentation for this trait is in `core/lib.rs` at the top level of this repository. You will need to consult the ALE documentation in order to implement `legal_action_set`. Each Atari game has a predefined legal action set. If you are writing a new game, you will need to map to the ALE set. 

* `impl State` : all of the per-transition computation happens in this struct. 

* `impl toybox_core::State for State`: the trait specification lives in `core/lib.rs` in the top level of this repository. To access the methods you wrote in your `State` implementation, call `self.state`. 

_Optional components_

* A `screen` module. This can be used to generate the static components of the background when you do not expect to be intervening on it.

* `impl <GameName>`: if you have any calculations that need to be done for configuration, put them here. 



## Adding human play

TODO

## Get starting images for reference from ALE

`./scripts/utils/start_images --help` 
