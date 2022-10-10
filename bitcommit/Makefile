CARGO_BIN ?= $(shell which cargo)

RUST_LIB_NAMES ?= $(shell ls ./libs | grep -v "tests")




build:
	#ifeq ($(MODE),libs)
	for file in $(RUST_LIB_NAMES); do \
		$(CARGO_BIN) build --manifest-path=./libs/$$file/Cargo.toml; \
	done
	#endif

clean:
	$(info 	var is $(RUST_LIB_NAMES))
	for number in $(RUST_LIB_NAMES); do \
		$(CARGO_BIN) clean -v --manifest-path ./libs/$$number/Cargo.toml; \
	done
