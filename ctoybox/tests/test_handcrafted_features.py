import unittest
from ctoybox import Toybox, Input
import numpy as np


def _exec_actions_collect_features(actions, tb):
    states = []
    for act in actions:
        # note: reasonably fast.
        states.append(tb.get_handcrafted_features())
        tb.apply_ale_action(act)
        if tb.game_over():
            tb.new_game()
    return states


class TestHandcraftedFeatures(unittest.TestCase):
    def _per_game(self, tb):
        # get 1000 random actions:
        action_set = tb.get_legal_action_set()
        actions = np.random.choice(action_set, size=1000)
        # save start state:
        state = tb.state_to_json()

        # determine how many points these actions earn:
        all_feature_vectors = _exec_actions_collect_features(actions, tb)

        for fv in all_feature_vectors:
            for (fname, feature) in fv.items():
                if feature < -1.0 or feature > 1.0:
                    self.fail(
                        "Non-normalized feature {} value={}\nfeatures={}".format(
                            fname, feature, fv
                        )
                    )

    def test_amidar(self):
        with Toybox("amidar") as tb:
            self._per_game(tb)

    def test_breakout(self):
        with Toybox("breakout") as tb:
            self._per_game(tb)

    def test_space_invaders(self):
        with Toybox("space_invaders") as tb:
            self._per_game(tb)


if __name__ == "__main__":
    unittest.main()
