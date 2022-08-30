aksenv=$PWD
cd gmp
../gmp-6.2.1/configure
cd ../ntl-11.5.1/src
./configure PREFIX="$aksenv/ntl" GMP_PREFIX="$aksenv/gmp"
make
make install