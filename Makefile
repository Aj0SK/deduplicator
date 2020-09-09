CC = clang++
CCFLAGS = -O2 -std=c++17 -Wall -fsanitize=address
TEST_DIR = test

DEFAULTPATH = --path data
DELETE = --action delete
HASHWYHASH = --hash_fun wyhash
HASHDUMMY =  --hash_fun dummy

TIME = time

all: run_test

reformat: reformat_rust
	clang-format -i -style=file $(TEST_DIR)/*.cpp

reformat_rust:
	cargo fmt

run_test: create_test_data deduplicator main

deduplicator:
	cargo build --release

create_test_data: test_data_gen
	./test_data_gen.out data kMaxCount 50

main:
	$(TIME) ./target/release/deduplicator $(DEFAULTPATH) $(DELETE) $(HASHWYHASH)

main-notime:
	./target/release/deduplicator $(DEFAULTPATH) $(DELETE) $(HASHWYHASH)

main-notime-nodelete:
	./target/release/deduplicator $(DEFAULTPATH) $(HASHWYHASH)

main-notime-nodelete-dummyhash:
	./target/release/deduplicator $(DEFAULTPATH) $(HASHDUMMY)

test_data_gen:
	$(CC) $(CCFLAGS) $(TEST_DIR)/generate_tree.cpp -o test_data_gen.out

unit_tests: unit_tests_install unit_tests_run

unit_tests_install:
	cd test && npm i

unit_tests_run: deduplicator
	cd test && npm run test
