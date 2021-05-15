from .ctoybox import Game, State as FrameState, Input
import numpy as np
from PIL import Image
import json
from typing import Dict, Any, List, Tuple, Union, Optional



def json_str(js: Union[Dict[str, Any], Input, str]) -> str:
    """
    Turn an object into a JSON string -- handles dictionaries, the Input class, and JSON you've already prepared (e.g., strings).
    """
    if type(js) is dict:
        js = json.dumps(js)
    elif type(js) is Input:
        js = json.dumps(js.__dict__)
    elif type(js) is not str:
        raise ValueError(
            "Unknown json type: %s (only str and dict supported)" % type(js)
        )
    return js


class Simulator(object):
    """
    The Simulator is an instance of a game configuration.
    You can call new_game on it to begin.

    """

    def __init__(self, game_name, sim=None):
        """
        Construct a new instance.

        Parameters:
            game_name: one of "breakout", "amidar", etc.
            sim: optionally a Rust pointer to an existing simulator.
        """
        if sim is None:
            sim = Game(game_name)
        self.__sim = sim
        # sim should be a pointer
        self.game_name = game_name

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        pass

    def set_seed(self, seed: int):
        """Configure the random number generator that spawns new game states.
        
        Parameters:
            seed: a parameter to reset the built-in random number generator.
        """
        self.__sim.seed(seed)
    
    def get_frame_size(self) -> Tuple[int, int]:
        """Get the width in pixels of the frames this game renders."""
        return self.__sim.frame_size()

    def get_frame_width(self) -> int:
        """Get the width in pixels of the frames this game renders."""
        return self.__sim.frame_size()[0]

    def get_frame_height(self) -> int:
        """Get the height in pixels of the frames this game renders."""
        return self.__sim.frame_size()[1]

    def get_simulator(self) -> Game:
        """Get access to the raw simulator pointer."""
        return self.__sim

    def new_game(self) -> "State":
        """Start a new game."""
        return State(self, self.__sim.new_game())

    def state_from_json(self, js: Union[Dict[str, Any], str]) -> "State":
        """Generate a State from the state json and this configuration.
        
        Parameters:
            js: a JSON object or string containing a serialized state.
        """
        state: FrameState = self.__sim.new_state(json_str(js))
        return State(self, state=state)

    def to_json(self) -> Dict[str, Any]:
        """Get the configuration of this simulator/config as JSON"""
        return json.loads(self.__sim.to_json())

    def from_json(self, config_js: Union[Dict[str, Any], str]):
        """Mutably update this simulator/config with the replacement json."""
        self.__sim = self.__sim.from_json(json_str(config_js))

    def schema_for_state(self) -> Dict[str, Any]:
        """Get the JSON Schema for any state for this game."""
        return json.loads(self.__sim.frame_schema())

    def schema_for_config(self) -> Dict[str, Any]:
        """Get the JSON Schema for any config for this game."""
        return json.loads(self.__sim.config_schema())


class State(object):
    """
    The State object represents everything the game needs to know about any single simulated frame.

    You can rewind in time by storing and restoring these state representations.

    - Access the json: ``to_json``
    - Access the image: ``render_frame``
    """

    def __init__(self, sim: Simulator, state=None):
        """
        Construct a new State instance wrapper.

        Parameters:
            sim: The simulator responsible for this state.
            state: Optional pointer to a state to use (otherwise it will create one). 
        """
        self.sim = sim
        """A reference to the simulator that created this state."""
        self.__state = state or sim.__sim.new_game()
        """The raw pointer to the state itself."""
        self.game_name = sim.game_name
        """The name of the game that created this state."""

    def __enter__(self):
        return self

    def __del__(self):
        self.__state = None
        self.sim = None

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def clone(self) -> 'State':
        """Quickly make a copy of this state; should be more efficient than saving the JSON."""
        return State(self.sim, state=self.get_state().copy())

    def get_state(self) -> FrameState:
        """Get the raw state pointer."""
        assert self.__state is not None
        return self.__state

    def lives(self) -> int:
        """How many lives are remaining in the current state?"""
        return self.__state.lives()

    def level(self) -> int:
        """How many levels have been completed in the current state?"""
        return self.__state.level()

    def score(self) -> int:
        """How many points have been earned in the current state?"""
        return self.__state.score()

    def game_over(self):
        """Determine whether the game has ended; i.e., the player has run out of lives.

        >>> assert self.lives() < 0 == self.game_over()
        """
        return self.lives() < 0

    def query_json(
        self, query: str, args: Union[Dict[str, Any], str] = "null"
    ) -> Dict[str, Any]:
        """
        Ask a question of the Rust state; queries are currently implemented manually.

        Parameters:
            query: the message to send to the rust state.
            args: the arguments to send to the rust state, defaults to "null".

        Returns:
            response: A JSON response loaded to python objects.

        Raises:
            ValueError: if anything goes wrong with the query

        ```python
        with Toybox("breakout") as tb:
          tb.query_json("bricks_remaining")
        ```
        """
        return json.loads(self.__state.query(json_str(query), json_str(args)))

    def render_frame(self, sim: Simulator, grayscale: bool = True) -> np.array:
        """Generate an image from the current frame state object.

        Parameters:
            sim: the simulator to use; this tells us the width/height necessary.
            grayscale: True if we want to render in grayscale rather than in color (RGBA).
        """
        if grayscale:
            return self.render_frame_rgb(sim)
        else:
            return self.render_frame_color(sim)

    def render_frame_color(self, sim: Simulator) -> np.array:
        """Generate an RGBA image from the current frame state object.

        Parameters:
            sim: the simulator to use; this tells us the width/height necessary.
        """
        (w, h) = sim.get_frame_size()
        rgba = 4
        size = h * w * rgba
        frame = bytearray(size)
        self.get_state().render_into_buffer(frame, True)
        return np.asarray(frame, dtype=np.uint8).reshape(h, w, rgba)

    def render_frame_rgb(self, sim: Simulator) -> np.array:
        """Generate an RGB image from the current frame state object.

        Parameters:
            sim: the simulator to use; this tells us the width/height necessary.
        """
        rgba_frame = self.render_frame_color(sim)
        return rgba_frame[:, :, :3]

    def render_frame_grayscale(self, sim: Simulator) -> np.array:
        """Generate a grayscale image from the current frame state object.

        Parameters:
            sim: the simulator to use; this tells us the width/height necessary.
        """
        (w, h) = sim.get_frame_size()
        depth = 1
        size = h * w * depth
        frame = bytearray(size)
        self.get_state().render_into_buffer(frame, False)
        return np.asarray(frame, dtype=np.uint8).reshape(h, w, depth)

    def to_json(self) -> Dict[str, Any]:
        """Get a JSON representation of the state."""
        return json.loads(self.get_state().to_json())



class Toybox(object):
    """
    This is a stateful representation of Toybox -- since it manages memory, we provide ``__enter__`` and ``__exit__`` usage for Python's with-blocks:
    
    ```python
    with Toybox("amidar") as tb:
        print(tb.get_score())
    # the 'tb' variable only lives in the block.
    ```

    Important:
        Note how we should use this in a with-block; this will clean up pointers and prevent memory leaks.

    """

    def __init__(self, 
                 game_name: str, 
                 grayscale: bool = True, 
                 frameskip: int = 0, 
                 seed: Optional[int] = None, 
                 withstate: Optional[dict] = None):
        """
        Construct a new Toybox state/game wrapper. Use this in a with block!

        Parameters:
            game_name: One of "breakout", "space_invaders", "amidar", etc.
            grayscale: Toybox can render directly to grayscale, saving time. Default is True.
            frameskip: When an action is submitted, for how many extra frames should it be applied? Default is 0.
            seed: The seed 
        """
        self.game_name = game_name
        self.frames_per_action = frameskip + 1
        self.rsimulator = Simulator(game_name)
        self.rstate = self.rsimulator.new_game()
        self.grayscale = grayscale
        if seed:
            self.set_seed(seed)
        self.new_game()
        if withstate:
            self.write_state_json(withstate)

    def new_game(self):
        """
        Modify this Toybox wrapper to have a new_game state.

        Important:
            This discards the old state!
        """
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.new_game()

    def get_height(self) -> int:
        """Get the height of the rendered game in pixels."""
        return self.rsimulator.get_frame_height()

    def get_width(self) -> int:
        """Get the width of the rendered game in pixels."""
        return self.rsimulator.get_frame_width()

    def get_legal_action_set(self) -> List[int]:
        """Get the set of actions consumed by this game: they are ALE numbered."""
        sim = self.rsimulator.get_simulator()
        return sim.legal_actions()

    def apply_ale_action(self, action_int: int):
        """Takes an integer corresponding to an action, as specified in ALE.
    
        This applies the action *k* times, where *k* based on the frameskip passed to the Toybox constructor.
    
        ```python
        ALE_INPUT_MAPPING = {
            0 : "NOOP",
            1 : "FIRE",
            2 : "UP",
            3 : "RIGHT",
            4 : "LEFT",
            5 : "DOWN",
            6 : "UPRIGHT",
            7 : "UPLEFT",
            8 : "DOWNRIGHT",
            9 : "DOWNLEFT",
            10 : "UPFIRE",
            11 : "RIGHTFIRE",
            12 : "LEFTFIRE",
            13 : "DOWNFIRE",
            14 : "UPRIGHTFIRE",
            15 : "UPLEFTFIRE",
            16 : "DOWNRIGHTFIRE",
            17 : "DOWNLEFTFIRE"
        }
        ```

        Parameters:
            action_int: A number from 0 to 17 inclusive.
        """
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            if not self.rstate.get_state().apply_ale_action(action_int):
                raise ValueError(
                    "Expected to apply action, but failed: {0}".format(action_int)
                )

    def apply_action(self, action_input_obj: Input):
        """Takes an [ctoybox.Input][] action and applies it - unlike the ALE actions (which allow some permutations) this allows for fine-grained button pressing.

        This applies the action *k* times, where *k* based on the frameskip passed to the Toybox constructor.
        
        Parameters:
            action_input_obj: An instance of the [ctoybox.Input][] class.
        """
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            self.rstate.get_state().apply_action(action_input_obj)

    def get_state(self) -> np.array:
        """This state here actually refers to the graphical, RGBA or grayscale representation of the current state."""
        return self.rstate.render_frame(self.rsimulator, self.grayscale)

    def set_seed(self, seed: int):
        """Control the random number generator of the config -- only affects a new_game.
        
        Parameters:
            seed: a parameter to reset the built-in random number generator.
        """
        self.rsimulator.set_seed(seed)
        # Maybe call new game here?

    def save_frame_image(self, path: str, grayscale: bool = False):
        """Save the current frame image to a PNG file.
        
        Parameters:
            path: the filename to save to.
            grayscale: whether images should be saved in color or black & white.
        """
        img = None
        if grayscale:
            img = Image.fromarray(
                self.rstate.render_frame_grayscale(self.rsimulator), "L"
            )
        else:
            img = Image.fromarray(
                self.rstate.render_frame_color(self.rsimulator), "RGBA"
            )
        img.save(path, format="png")

    def get_rgb_frame(self) -> np.array:
        """Get the RGB frame as a numpy array."""
        return self.rstate.render_frame_rgb(self.rsimulator)

    def get_score(self) -> int:
        """Access the current score.

        Returns:
            The number of points earned in the current state."""
        return self.rstate.score()

    def get_lives(self) -> int:
        """Access the number of lives.

        Returns:
            The number of lives remaining in the current state."""
        return self.rstate.lives()

    def get_level(self) -> int:
        """
        Access the number of levels.
        
        Returns:
            The number of levels completed in the current state."""
        return self.rstate.level()

    def game_over(self) -> bool:
        """
        Check for game over condition.

        Returns:
            ``True`` if the player has run out of lives in the current state.
        """
        return self.rstate.game_over()

    def state_to_json(self) -> Dict[str, Any]:
        """Get the state's JSON representation as a python object."""
        return self.rstate.to_json()

    def to_state_json(self) -> Dict[str, Any]:
        """Get the state's JSON representation as a python dict.
        
        Important:
            This method is deprecated; please use ``state_to_json`` instead!
        """
        return self.state_to_json()

    def config_to_json(self) -> Dict[str, Any]:
        """Get the state's JSON representation as a python dict."""
        return self.rsimulator.to_json()

    def write_state_json(self, js: Dict[str, Any]):
        """Overwrite the state's JSON representation from a python dict.
        
        Parameters:
            js: the python representation of the JSON state.
        """
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.state_from_json(js)

    def write_config_json(self, config_js: Dict[str, Any]):
        """Overwrite the config's JSON representation from a python dict. 
        
        It is likely that some changes will be seen until you call new_game()

        Parameters:
            config_js: the python representation of the config JSON
        """
        # from_json replaces simulator!
        self.rsimulator.from_json(config_js)
        # new_game replaces state!
        self.new_game()

    def query_state_json(
        self, query: str, args: Union[Dict[str, Any], str] = "null"
    ) -> Dict[str, Any]:
        """Submit a query to the game's query system -- faster than accessing the whole JSON for quick introspection.

        Parameters:
            query: the query string to send to the game.
            args: a JSON argument to attach to the query string.
        """
        return self.rstate.query_json(query, args)

    def __del__(self):
        self.rstate = None
        self.rsimulator = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def schema_for_state(self) -> Dict[str, Any]:
        """Get the JSON Schema for the frame State object."""
        return self.rsimulator.schema_for_state()

    def schema_for_config(self) -> Dict[str, Any]:
        """Get the JSON Schema for the Config object."""
        return self.rsimulator.schema_for_config()
