import unittest
from ctoybox import Toybox

class TestOptionalArgs(unittest.TestCase):
    def test_with_seed(self):
        # get default seed
        v1, v2, v1_, v2_ = [None]*4
        with Toybox("breakout") as tb:
            (v1, v2) = tb.state_to_json()["rand"]["state"]
        with Toybox("breakout", seed=1234) as tb:
            (v1_, v2_) = tb.state_to_json()["rand"]["state"]
            self.assertNotEqual(v1, v1_)
            self.assertNotEqual(v2, v2_)

    def test_with_state(self):
        state = None
        with Toybox("breakout") as tb:
            state = tb.state_to_json()
            state['lives'] = 10
        with Toybox("breakout", withstate=state) as tb:
            state = tb.state_to_json()
            self.assertEqual(state['lives'], 10)
            

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
        with Toybox("spaceinvaders") as tb:
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

class TestNewQueries(unittest.TestCase):
    def test_amidar(self):
        with Toybox("amidar") as tb:
            state = tb.state_to_json()
            self.assertEqual(1, tb.query_state_json('.state.level'))
            self.assertEqual(0, tb.query_state_json(".state.score"))
            self.assertEqual(4, tb.query_state_json(".state.jumps"))
            self.assertEqual(state['enemies'][3]['position']['x'], tb.query_state_json(".state.enemies[3].position.x"))
            self.assertEqual(state['enemies'][2]['position']['y'], tb.query_state_json(".state.enemies[2].position.y"))
            print(tb.query_state_json(".state.enemies[2].position.y"))

if __name__ == "__main__":
    unittest.main()
