import unittest
from ctoybox import Toybox, Input
import numpy as np

class TestSetSeed(unittest.TestCase):

    def test_ffi_toybox_set_seed_no_new_game(self):
        with Toybox('breakout') as tb:
            json = tb.state_to_json()
            rng1_before, rng2_before = json['rand']['state']
            tb.set_seed(1234)
            json = tb.state_to_json()
            rng1_after, rng2_after = json['rand']['state']
            # You need to call tb.new_game() to reset the RNG
            # Therefore, these should be equal.
            self.assertEqual(rng1_before, rng1_after)
            self.assertEqual(rng2_before, rng2_after)

    def test_ffi_toybox_set_seed_new_game(self):
        with Toybox('breakout') as tb:
            json = tb.state_to_json()
            rng1_before, rng2_before = json['rand']['state']
            tb.set_seed(1234)
            # Get new game
            #tb.new_game()
            json = tb.state_to_json()
            rng1_after, rng2_after = json['rand']['state']
            # Now they should be equal
            self.assertNotEqual(rng1_before, rng1_after)
            self.assertNotEqual(rng2_before, rng2_after)

class TestBreakoutBallRand(unittest.TestCase):

    def test_rand_start_10_seeds(self):
        with Toybox('breakout') as tb:
            balls = []
            input = Input()
            input.button1 = True
            for i in range(10):
                tb.set_seed(i)
                tb.new_game()
                tb.apply_action(input)
                json = tb.state_to_json()
                self.assertNotEqual(len(json['balls']), 0)
                balls.append(json['balls'][0])
            ball_xpos = [ball['position']['x'] for ball in balls]
            x = ball_xpos[0]
            # At least one of these should be different!
            self.assertFalse(all([bx == x for bx in ball_xpos]))

if __name__ == "__main__":
    unittest.main()
