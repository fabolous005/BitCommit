CARGO_BIN ?= $(shell which cargo)

LIB_NAMES ?= $(ls ./libs)

MODE ?= all


build:
	ifeq ($(MODE),libs)
		$(for file in $(LIB_NAMES); do $(CARGO_BIN) --manifest-path=./$file/Cargo.toml build; done)
	
	endif
