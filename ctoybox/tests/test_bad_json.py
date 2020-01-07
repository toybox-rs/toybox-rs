import unittest
from ctoybox import Toybox
import numpy as np


BAD_JSON = {
    'this is': 'good for nothing'
}

class TestRandomAgents(unittest.TestCase):
    def _bad_json_is_recoverable(self, tb):
        try:
            tb.write_state_json(BAD_JSON)
            self.fail('Bad JSON for state should crash!')
        except ValueError:
            pass
        try:
            tb.write_config_json(BAD_JSON)
            self.fail('Bad JSON for config should crash!')
        except ValueError:
            pass

    def test_amidar(self):
        with Toybox("amidar") as tb:
            self._bad_json_is_recoverable(tb)

    def test_breakout(self):
        with Toybox("breakout") as tb:
            self._bad_json_is_recoverable(tb)

    def test_space_invaders(self):
        with Toybox("space_invaders") as tb:
            self._bad_json_is_recoverable(tb)

if __name__ == "__main__":
    unittest.main()
