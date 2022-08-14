CARGO_BIN ?= $(shell which cargo)

LIB_NAMES ?= $(ls ./libs)
MODE ?= all # default value





GCC_BIN ?= $(shell which gcc)

C_DIR_RSYNC = "/rsync/src/"
C_FILENAME_RSYNC = "something" # TODO make this to the corresponding filename

BUILD_RSYNC_C ?= $(GCC_BIN) ./libs/$(C_DIR)$(C_FILENAME_RSYNC


BUILD_C = $(BUILD_RSYNC_C) # TODO add additional files





build:
	ifeq ($(MODE),libs)
		$(for file in $(LIB_NAMES); do $(CARGO_BIN) --manifest-path=./$file/Cargo.toml build; done)
	
	endif
