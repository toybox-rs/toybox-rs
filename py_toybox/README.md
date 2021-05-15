# ctoybox

This is the bare-bones api for [Toybox](https://toybox.rs).

## Dependencies

- pillow (for rendering images)
- numpy (for allocating arrays of the right type for the FFI)
- cffi
- pygame (optionally for ``human_play``)

## Build locally

If you want to make changes to any part of the rust-python pipline, you will need to build and test locally (ran from top-level of the repository):

```
pip install -r ctoybox/requirements.txt
pip install --upgrade pip
pip install maturin
cd ctoybox
./test.sh
```