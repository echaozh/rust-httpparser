VPATH=/home/echaozh/rust-http-client

CC ?= gcc
RUSTC ?= rustc
AR ?= ar
RUSTFLAGS ?=
CFLAGS ?=

CFLAGS += -fPIC

RUST_SRC = http_parser.rs parser.rs

.PHONY: all
all: libhttpparser.dummy

libhttpparser.dummy: httpparser.rc $(RUST_SRC) libhttp_parser.a
	$(RUSTC) $(RUSTFLAGS) $<
	touch $@

# httpparser-test: http_client.rc $(RUST_SRC) libhttp_parser.a
# 	$(RUSTC) $(RUSTFLAGS) $< -o $@ --test

libhttp_parser.a: http_parser.o
	$(AR) rcs $@ $<

http_parser.o: http-parser/http_parser.c
	$(CC) $(CFLAGS) $< -o $@ -c

# check: http_client-test
# 	./http_client-test

.PHONY: clean
clean:
	rm -f httpparser-test *.o *.a *.so *.dylib *.dll *.dummy
