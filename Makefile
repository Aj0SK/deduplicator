CC = clang++
CCFLAGS = -O2 -std=c++17 -Wall -fsanitize=address
RUST_FLAGS = --release
TIME = time

TEST_DIR = test
CREATE_DATA_CPP = kMaxCount 50
DEFAULTPATH = --path data
PRINT = --print
DELETE = --delete
HASHWYHASH = --hash_fun wyhash
HASHDUMMY =  --hash_fun dummy

all: run_test

reformat:
	clang-format -i -style=file $(TEST_DIR)/*.cpp
	cargo fmt

run_test: create_test_data deduplicator main

deduplicator:
	cargo build $(RUST_FLAGS)

create_test_data: test_data_gen
	./test_data_gen.out data $(CREATE_DATA_CPP)

test_data_gen:
	$(CC) $(CCFLAGS) $(TEST_DIR)/generate_tree.cpp -o test_data_gen.out

main:
	$(TIME) ./target/release/deduplicator $(DEFAULTPATH) $(PRINT) $(DELETE) $(HASHWYHASH)

main-notime:
	./target/release/deduplicator $(DEFAULTPATH) $(PRINT) $(DELETE) $(HASHWYHASH)

main-notime-nodelete:
	./target/release/deduplicator $(DEFAULTPATH) $(PRINT) $(HASHWYHASH)

main-notime-nodelete-dummyhash:
	./target/release/deduplicator $(DEFAULTPATH) $(PRINT) $(HASHDUMMY)

unit_tests: unit_tests_install unit_tests_run

unit_tests_install:
	cd test && npm i

unit_tests_run: deduplicator
	cd test && npm run test

unit_tests_run_heavy: deduplicator
	cd test && npm run test_heavy