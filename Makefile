CC = clang++
CCFLAGS = -O2 -std=c++17 -Wall -fsanitize=address
TEST_DIR = test

all: create_test_data

reformat:
	clang-format -i -style=file $(TEST_DIR)/*.cpp

create_test_data: test_data_gen
	./test_data_gen.out

test_data_gen:
	$(CC) $(CCFLAGS) $(TEST_DIR)/generate_tree.cpp -o test_data_gen.out
