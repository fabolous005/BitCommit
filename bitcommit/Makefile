CARGO_BIN ?= $(shell which cargo)

RUST_LIB_NAMES ?= $(shell ls ./libs | grep -v "tests")









clean:
	$(info 	var is $(RUST_LIB_NAMES))
	for number in $(RUST_LIB_NAMES); do \
		$(CARGO_BIN) clean -v --manifest-path ./libs/$$number/Cargo.toml; \
	done


build:
	ifeq ($(MODE),libs)
		$(for file in $(LIB_NAMES); do $(CARGO_BIN) --manifest-path=./$file/Cargo.toml build; done)
	
	endif


