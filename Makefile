CC = clang++
CCFLAGS = -O2 -std=c++17 -Wall -fsanitize=address
TEST_DIR = test

DEDUPLICATOR = delete
TIME = time

all: run_test

reformat:
	clang-format -i -style=file $(TEST_DIR)/*.cpp

reformat_rust:
	cargo fmt

run_test: create_test_data deduplicator main

deduplicator:
	cargo build --release

create_test_data: test_data_gen
	./test_data_gen.out

main:
	$(TIME) ./target/release/deduplicator $(DEDUPLICATOR)

test_data_gen:
	$(CC) $(CCFLAGS) $(TEST_DIR)/generate_tree.cpp -o test_data_gen.out
