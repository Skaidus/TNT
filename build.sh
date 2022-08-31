#!/usr/bin/env bash
set -eu -o pipefail

GMP_VERSION=6.2.1
NTL_VERSION=11.5.1

TMP_DIR="$(mktemp -d)"
PREFIX="$PWD/output"
echo Using PREFIX="$PREFIX"
pushd "$TMP_DIR"
mkdir -p "$PREFIX"

wget https://gmplib.org/download/gmp/gmp-$GMP_VERSION.tar.xz
tar -xf gmp-$GMP_VERSION.tar.xz
cd gmp-$GMP_VERSION
./configure --disable-shared --enable-static --prefix="$PREFIX"
make -j "$(nproc)"
make install
cd ..
wget https://shoup.net/ntl/ntl-$NTL_VERSION.tar.gz
tar xf ntl-$NTL_VERSION.tar.gz
cd ntl-$NTL_VERSION/src
./configure NTL_GF2X_LIB=on DEF_PREFIX="$PREFIX" NATIVE=off
make -j "$(nproc)"
make install
popd


# aksenv=$PWD
# cd gmp
# ../gmp-6.2.1/configure
# cd ../ntl-11.5.1/src
# ./configure PREFIX="$aksenv/ntl" GMP_PREFIX="$aksenv/gmp"
# make
# make install