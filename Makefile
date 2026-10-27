.PHONY: clean clean_all

PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

EXTENSION_NAME=fakeit

# duckdb-neo uses the V2 C API and is tied to the exact DuckDB runtime version.
USE_UNSTABLE_C_API=1
CAPI_VERSION=2

# Target DuckDB version for extension metadata and local tests.
TARGET_DUCKDB_VERSION=v2.0.0
DUCKDB_TEST_VERSION?=$(patsubst v%,%,$(TARGET_DUCKDB_VERSION))

all: configure debug

# Include makefiles from DuckDB
include extension-ci-tools/makefiles/c_api_extensions/base.Makefile
include extension-ci-tools/makefiles/c_api_extensions/rust.Makefile

# The v2.0-cyanoptera CI makefile still hardcodes the V1 Wasm entrypoint.
ifneq ($(DUCKDB_WASM_PLATFORM),)
link_wasm_debug: build_extension_library_debug
	emcc $(EXTENSION_BUILD_PATH)/debug/$(EXTENSION_LIB_FILENAME) -o $(EXTENSION_BUILD_PATH)/debug/$(EXTENSION_FILENAME_NO_METADATA) -O3 -g -sSIDE_MODULE=2 -sEXPORTED_FUNCTIONS="_$(EXTENSION_NAME)_init_c_api_v2"

link_wasm_release: build_extension_library_release
	emcc $(EXTENSION_BUILD_PATH)/release/$(EXTENSION_LIB_FILENAME) -o $(EXTENSION_BUILD_PATH)/release/$(EXTENSION_FILENAME_NO_METADATA) -O3 -sSIDE_MODULE=2 -sEXPORTED_FUNCTIONS="_$(EXTENSION_NAME)_init_c_api_v2"
endif

configure: venv platform extension_version

debug: build_extension_library_debug build_extension_with_metadata_debug
release: build_extension_library_release build_extension_with_metadata_release

test: test_debug
test_debug: test_extension_debug
test_release: test_extension_release

clean: clean_build clean_rust
clean_all: clean_configure clean
