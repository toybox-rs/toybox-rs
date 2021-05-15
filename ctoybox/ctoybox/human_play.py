from .ffi import Toybox, Input
import pygame._numpysurfarray as numpysf

import numpy as np
import argparse
import pygame
import pygame.key
from pygame.locals import *
import pygame.surfarray
import json
import sys

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="human_play for toybox")
    parser.add_argument(
        "game", type=str, help="try one of amidar, breakout, space_invaders"
    )
    parser.add_argument("--scale", type=int, default=2)
    parser.add_argument("--fps", type=int, default=32)
    parser.add_argument("--query", type=str, default=None)
    parser.add_argument("--query_args", type=str, default="null")
    parser.add_argument("--seed", type=int, default=-1)
    parser.add_argument("--record", action='store_const', const=True)

    args = parser.parse_args()
    print("Starting up: " + args.game)
    pygame.init()

    with Toybox(args.game) as tb:
        w = tb.get_width()
        h = tb.get_height()

        if args.seed >= 0:
            tb.set_seed(args.seed)
            tb.new_game()

        config_json = tb.config_to_json()
        state_json = tb.to_state_json()

        dim = (w * args.scale, h * args.scale)

        pygame.display.set_mode(dim)
        clock = pygame.time.Clock()
        FPS = args.fps

        quit = False
        while not quit:
            # close human_play on game over
            if tb.game_over():
                break
            for event in pygame.event.get():
                if event.type == QUIT:
                    quit = True
                    break
                if event.type == KEYDOWN and event.key == K_ESCAPE:
                    quit = True
                    break
            key_state = pygame.key.get_pressed()
            player_input = Input()

            # Explicitly casting to bools because in some versions, the RHS gets converted
            # to ints, causing problems when we load into the associated rust structs.
            player_input.left = bool(key_state[K_LEFT] or key_state[K_a])
            player_input.right = bool(key_state[K_RIGHT] or key_state[K_d])
            player_input.up = bool(key_state[K_UP] or key_state[K_w])
            player_input.down = bool(key_state[K_DOWN] or key_state[K_s])
            player_input.button1 = bool(key_state[K_z] or key_state[K_SPACE])
            player_input.button2 = bool(
                key_state[K_x] or key_state[K_RSHIFT] or key_state[K_LSHIFT]
            )

            tb.apply_action(player_input)
            if args.record:
                if player_input.left: print('left')
                elif player_input.right: print('right')
                elif player_input.up: print('up')
                elif player_input.down: print('down')
            if args.query is not None:
                print(args.query, tb.query_state_json(args.query, args.query_args))
            image = tb.get_rgb_frame()
            screen = pygame.display.get_surface()
            img = pygame.surfarray.make_surface(np.swapaxes(image, 0, 1))
            numpysf.make_surface()
            img2x = pygame.transform.scale(img, dim)
            screen.blit(img2x, dest=(0, 0))
            pygame.display.update()
            if key_state[K_TAB]:
                clock.tick(FPS * 4)
            else:
                clock.tick(FPS)
    pygame.quit()
    sys.exit()
