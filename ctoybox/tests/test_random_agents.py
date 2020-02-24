import unittest
from ctoybox import Toybox, Input
import numpy as np


def _exec_actions_max_score(actions, tb):
    score = 0
    states = []
    for act in actions:
        states.append(tb.rstate.clone())
        # states.append(tb.state_to_json())
        tb.apply_ale_action(act)
        if tb.get_score() > score:
            score = tb.get_score()
        if tb.game_over():
            break
    return score


class TestRandomAgents(unittest.TestCase):
    def _1000_random_actions_are_deterministic(self, tb):
        # get 1000 random actions:
        action_set = tb.get_legal_action_set()
        actions = np.random.choice(action_set, size=1000)
        # save start state:
        state = tb.state_to_json()

        # determine how many points these actions earn:
        max_score1 = _exec_actions_max_score(actions, tb)

        # make sure it happens again:
        tb.write_state_json(state)
        max_score2 = _exec_actions_max_score(actions, tb)
        self.assertEqual(max_score1, max_score2)

    def test_amidar(self):
        with Toybox("amidar") as tb:
            self._1000_random_actions_are_deterministic(tb)

    def test_breakout(self):
        with Toybox("breakout") as tb:
            self._1000_random_actions_are_deterministic(tb)

    def test_space_invaders(self):
        with Toybox("space_invaders") as tb:
            self._1000_random_actions_are_deterministic(tb)

    def test_amidar_straight_up(self):
        with Toybox("amidar") as tb:
            up = Input()
            up.up = True
            score = 0
            while not tb.game_over():
                score = max(tb.get_score(), score)
                tb.apply_action(up)
            self.assertEqual(2, score)

    def test_amidar_straight_down(self):
        with Toybox("amidar") as tb:
            down_fire = Input()
            # down and fire!
            down_fire.down = True
            down_fire.button1 = True
            score = 0
            while not tb.game_over():
                score = max(tb.get_score(), score)
                tb.apply_action(down_fire)
            self.assertEqual(2, score)


if __name__ == "__main__":
    unittest.main()
