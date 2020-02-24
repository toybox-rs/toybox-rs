from .ffi import Toybox
import argparse
import json
import sys

if __name__ == "__main__":
    parser = argparse.ArgumentParser("json_schema")
    parser.add_argument("game", help="e.g., breakout, amidar, space_invaders")
    parser.add_argument("kind", help="either config or state")
    args = parser.parse_args()

    with Toybox(args.game) as tb:
        if args.kind == "config":
            json.dump(tb.schema_for_config(), sys.stdout, indent=2)
        elif args.kind == "state":
            json.dump(tb.schema_for_state(), sys.stdout, indent=2)
        else:
            raise ValueError(
                "kind={} but should be either config or state".format(args.kind)
            )

