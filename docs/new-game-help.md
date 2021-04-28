# Developing New Games

## Initial Setup
1. Run `cargo new tb_<gamename> --lib` in the top level of the repository. 
2. Edit the `Cargo.toml` in the top level of the repository to add `tb_<gamename>` to the list of members.
3. Create `<gamename>.rs` in `tb_<gamename>/src`.
4. Create `types.rs` in `tb_<gamename>/src`.

`tb_<gamename>` should now have three files in it: `lib.rs`, `types.rs`, and `<gamename>.rs`. 

## Basic Game Components

__`lib.rs`__

`lib.rs` is the top-level API. Only modules declared here will be exposed to other modules. Copy the following to the top of your `lib.rs`:

```
extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate rand;
```

Every `lib.rs` will also have the following declarations:

```
mod types;
mod <gamename>;
```

You will export other structures as needed. We recommend that you update this on an as-need basis -- Rust will tell you when it can't find a module, and you should use this to guide when something needs to be exported. 

__`<types>.rs`__

The types module contains all of the intervenable structs. You should update this file concurrently with `<gamename.rs>` (we recommend having both open at the same time).

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

* You must import the following namespaces:
```
use toybox_core;
use toybox_core::random;
use types::*;

use serde_json;
use rand::seq::SliceRandom;
```

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
  * The JSON loading and manipulation code can be dropped in with one textual change:

```
fn new_state_from_json(
    &self,
    json_str: &str,
) -> Result<Box<toybox_core::State>, serde_json::Error> {
    let state: StateCore = serde_json::from_str(json_str)?;
    Ok(Box::new(State {
        config: self.clone(),
        state,
    }))

fn from_json(&self, json_str: &str) -> Result<Box<toybox_core::Simulation>, serde_json::Error> {
    let config: <GameName> = serde_json::from_str(json_str;
    Ok(Box::new(config))
}

fn to_json(&self) -> String {
    serde_json::to_string(self).expect("<GameName> shoulbe JSON-serializable!")
}
```

* `impl State` : all of the per-transition computation happens in this struct. 

* `impl toybox_core::State for State`: the trait specification lives in `core/lib.rs` in the top level of this repository. To access the methods you wrote in your `State` implementation, call `self.state`. 

_Optional components_

* A `screen` module. This can be used to generate the static components of the background when you do not expect to be intervening on it.

* `impl <GameName>`: if you have any calculations that need to be done for configuration, put them here. 



## Adding human play

## Get starting images for reference from ALE

`./scripts/utils/start_images --help` 
