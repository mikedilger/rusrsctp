#!/bin/sh

LD_LIBRARY_PATH=$(pwd)/usrsctp/usrsctplib/.libs cargo test
