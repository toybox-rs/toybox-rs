
class Input():
    """An input object represents a game controller having left, right, up, down, and two buttons.
    
    ALE mapping:

     - 0 : "NOOP"
     - 1 : "FIRE"
     - 2 : "UP"
     - 3 : "RIGHT"
     - 4 : "LEFT"
     - 5 : "DOWN"
     - 6 : "UPRIGHT"
     - 7 : "UPLEFT"
     - 8 : "DOWNRIGHT"
     - 9 : "DOWNLEFT"
     - 10 : "UPFIRE"
     - 11 : "RIGHTFIRE"
     - 12 : "LEFTFIRE"
     - 13 : "DOWNFIRE"
     - 14 : "UPRIGHTFIRE"
     - 15 : "UPLEFTFIRE"
     - 16 : "DOWNRIGHTFIRE"
     - 17 : "DOWNLEFTFIRE"
    
    """

    _LEFT = "left"
    _RIGHT = "right"
    _UP = "up"
    _DOWN = "down"
    _BUTTON1 = "button1"
    _BUTTON2 = "button2"
    _NOOP = "noop"

    def __init__(self, left=False, right=False, up=False, down=False, button1=False, button2=False):
        """
        The default constructor creates an input object with no buttons pressed.

        :param left: Move left in most games.
        :param right: Move right in most games.
        :param up: Move up in most games.
        :param down: Move down in most games.
        :param button1: FIRE or ACTION1 or CONFIRM in most games.
        :param button2: ACTION2 or CANCEL in most games.
        """
        self.left = left
        """The left button is a directional command to the game from the player."""
        self.right = right
        """The right button is a directional command to the game from the player."""
        self.up = up
        """The up button is a directional command to the game from the player."""
        self.down = down
        """The down button is a directional command to the game from the player."""
        self.button1 = button1
        """Confirm, FIRE, or any other action may be associated with button1."""
        self.button2 = button2
        """Cancel, or any other action may be associated with button2."""

    def reset(self):
        """
        This method turns all buttons pressed to false.
        """
        self.left = False
        self.right = False
        self.up = False
        self.down = False
        self.button1 = False
        self.button2 = False

    def __str__(self) -> str:
        """
        Get a string representation of this Input object.
        """
        return self.__dict__.__str__()

    def __repr__(self):
        """
        Get a string representation of this Input object.
        """
        return self.__dict__.__str__()

    def set_input(self, input_dir: str, button=_NOOP):
        """
        Set the direction and button separately based on strings.
        """
        input_dir = input_dir.lower()
        button = button.lower()

        # reset all directions
        if   input_dir == Input._NOOP:
            pass
        elif input_dir == Input._LEFT:
            self.left = True
        elif input_dir == Input._RIGHT:
            self.right = True
        elif input_dir == Input._UP:
            self.up = True
        elif input_dir == Input._DOWN:
            self.down = True
        else:
            print('input_dir:', input_dir)
            assert False

        # reset buttons
        if button == Input._NOOP:
            pass
        elif button == Input._BUTTON1:
            self.button1 = True
        elif button == Input._BUTTON2:
            self.button2 = True
        else:
            assert False