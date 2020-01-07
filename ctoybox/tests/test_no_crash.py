import unittest
from ctoybox import Toybox


class TestNoCrash(unittest.TestCase):
    def test_amidar(self):
        config = None
        state = None
        with Toybox("amidar") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
            self.assertEqual(1, tb.get_level())
            self.assertEqual(356, tb.query_state_json("num_tiles_unpainted"))
            self.assertEqual(True, tb.query_state_json("jumps_remaining"))
            self.assertEqual(0, tb.get_score())
            self.assertIsNotNone(tb.get_state())

    def test_breakout(self):
        config = None
        state = None
        with Toybox("breakout") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
            self.assertEqual(108, tb.query_state_json("bricks_remaining"))
            self.assertEqual(0, tb.query_state_json("count_channels"))
            self.assertEqual(1, tb.get_level())
            self.assertEqual(0, tb.get_score())
            self.assertIsNotNone(tb.get_state())

    def test_space_invaders(self):
        config = None
        state = None
        with Toybox("space_invaders") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()
            self.assertEqual(3, tb.query_state_json("shield_count"))
            self.assertEqual(state["ship"]["x"], tb.query_state_json("ship_x"))
            self.assertEqual(1, tb.get_level())
            self.assertEqual(0, tb.get_score())
            self.assertIsNotNone(tb.get_state())
            self.assertIsNotNone(tb.get_state())

    def test_gridworld(self):
        config = None
        state = None
        with Toybox("gridworld") as tb:
            config = tb.config_to_json()
            state = tb.state_to_json()


if __name__ == "__main__":
    unittest.main()
