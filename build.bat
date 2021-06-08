@echo off
setlocal
set LIBPCAP_LIBDIR=Lib\x64
set LIBPCAP_VER=4.1.3
cargo build --release
