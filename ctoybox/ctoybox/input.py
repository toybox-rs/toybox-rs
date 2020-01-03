
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

    def __init__(self):
        """
        The default constructor creates an input object with no buttons pressed.
        """
        self.reset()

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