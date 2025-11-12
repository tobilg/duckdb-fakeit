.PHONY: clean clean_all

PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

EXTENSION_NAME=fakeit

# Set to 1 to enable Unstable API (binaries will only work on TARGET_DUCKDB_VERSION, forwards compatibility will be broken)
# Note: currently extension-template-rs requires this, as duckdb-rs relies on unstable C API functionality
USE_UNSTABLE_C_API=1

# Target DuckDB version
TARGET_DUCKDB_VERSION=v1.4.1

# Use working version of duckdb_sqllogictest (before __main__.py was removed)
DUCKDB_SQLLOGICTEST_COMMIT=be27216

all: configure debug

# Include makefiles from DuckDB
include extension-ci-tools/makefiles/c_api_extensions/base.Makefile
include extension-ci-tools/makefiles/c_api_extensions/rust.Makefile

# Override venv target to use working version of duckdb_sqllogictest
configure/venv:
	python3.13 -m venv configure/venv
	./configure/venv/bin/python3 -m pip install $(DUCKDB_PIP_INSTALL)
	./configure/venv/bin/python3 -m pip install git+https://github.com/duckdb/duckdb-sqllogictest-python@$(DUCKDB_SQLLOGICTEST_COMMIT)
	./configure/venv/bin/python3 -m pip install packaging

configure: venv platform extension_version

debug: build_extension_library_debug build_extension_with_metadata_debug
release: build_extension_library_release build_extension_with_metadata_release

test: test_debug
test_debug: test_extension_debug
test_release: test_extension_release

clean: clean_build clean_rust
clean_all: clean_configure clean
