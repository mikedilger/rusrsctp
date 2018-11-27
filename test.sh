#!/bin/sh

LD_LIBRARY_PATH=$(pwd)/usrsctp/usrsctplib/.libs RUST_BACKTRACE=1 cargo test
