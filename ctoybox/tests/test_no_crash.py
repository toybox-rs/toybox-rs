import unittest
from ctoybox import Toybox

class TestNoCrash(unittest.TestCase):
    def test_amidar(self):
        config = None
        state = None
        with Toybox("amidar") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
    
    def test_breakout(self):
        config = None
        state = None
        with Toybox("breakout") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
    
    def test_space_invaders(self):
        config = None
        state = None
        with Toybox("space_invaders") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
    
    def test_gridworld(self):
        config = None
        state = None
        with Toybox("gridworld") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
    



if __name__ == "__main__":
    unittest.main()
