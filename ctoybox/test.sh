# delete old rust wheels
rm -rf ctoybox/ctoybox
# build rust package locally
maturin develop --release
# run unit tests
python -m unittest discover -s tests -v
