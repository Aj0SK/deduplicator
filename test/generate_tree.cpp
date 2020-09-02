#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <random>
#include <string>
#include <vector>

namespace fs = std::filesystem;

using std::cin;
using std::cout;
using std::endl;
using std::ofstream;
using std::shared_ptr;
using std::string;
using std::to_string;
using std::vector;

std::map<string, size_t> config = {{"kSeed", 21},
                                   {"kMaxDepth", 4},
                                   {"kMaxCount", 40},
                                   {"kStopRatio", 3},
                                   {"kFileMinSizeBlocks", 128},
                                   {"kFileMaxSizeBlocks", 256}};

const string blockSize("1K");

struct DirTreeNode
{
  string name;
  bool directory = false;
  vector<shared_ptr<DirTreeNode>> desc = {};
  vector<shared_ptr<DirTreeNode>> files = {};
  DirTreeNode(const string& name, bool is_directory)
      : name(name), directory(is_directory){};
  void add_desc(shared_ptr<DirTreeNode>&& x) { desc.push_back(std::move(x)); };
  void add_file(shared_ptr<DirTreeNode>&& x) { files.push_back(std::move(x)); };
};

void create_file(const string& path, std::mt19937& gen, int type = 0)
{
  ofstream f(path);
  int size = gen() % (config["kFileMaxSizeBlocks"] + 1 -
                      config["kFileMinSizeBlocks"]) +
             config["kFileMinSizeBlocks"];
  if (type == 0)
  {
    f << gen();
  }
  else if (type == 1)
  {
    string command = "/bin/dd if=/dev/urandom of=" + path + " bs=" + blockSize +
                     " count=" + to_string(size);
    std::system(command.c_str());
  }
  else
  {
    string command = "/bin/dd if=/dev/zero of=" + path + " bs=" + blockSize +
                     " count=" + to_string(size);
    std::system(command.c_str());
  }

  f.close();
}

shared_ptr<DirTreeNode> create_tree(const string& root_dir)
{
  std::mt19937 gen(config["kSeed"]);

  shared_ptr<DirTreeNode> root = std::make_shared<DirTreeNode>(root_dir, true);
  fs::create_directory(root_dir);

  for (size_t i = 0; i < config["kMaxCount"] / 2; ++i)
  {
    auto curr_root = root;
    int curr_depth = 0;
    string curr_path = root_dir;

    while (1)
    {
      int random = gen() % config["kStopRatio"];
      if (random == 0 || curr_depth == config["kMaxDepth"])
      {
        string filename = to_string(gen()) + ".txt";
        int type = gen() % 3;
        create_file(curr_path + "/" + filename, gen, type);
        curr_root->add_file(std::make_shared<DirTreeNode>(filename, false));
        // add duplicate
        string filename_dup = to_string(gen()) + ".txt";
        fs::copy(curr_path + "/" + filename, curr_path + "/" + filename_dup);
        curr_root->add_file(std::make_shared<DirTreeNode>(filename_dup, false));
        break;
      }

      int dir = gen() % (curr_root->desc.size() + 1);
      if (dir == curr_root->desc.size())
      {
        fs::create_directory(curr_path + "/" + to_string(i));
        curr_root->add_desc(std::make_shared<DirTreeNode>(to_string(i), true));
      }
      ++curr_depth;
      curr_path += "/" + curr_root->desc[dir]->name;
      curr_root = curr_root->desc[dir];
    }
  }

  return root;
}

void print_tree(shared_ptr<DirTreeNode> root_dir, int tabs = 0)
{
  string spaces(tabs, '\t');
  cout << spaces;
  if (root_dir->directory)
    cout << root_dir->name << " dir: " << endl;
  else
    cout << root_dir->name << endl;

  for (const auto& x : root_dir->files)
  {
    cout << spaces << '\t';
    cout << "File " << x->name << endl;
  }

  for (const auto& x : root_dir->desc)
  {
    print_tree(x, tabs + 1);
  }
}

int main(int argc, char* argv[])
{
  // not sure of performance benefits
  // ios_base::sync_with_stdio(false);
  for (size_t i = 1; i < argc; i += 2)
  {
    string arg_name(argv[i]);
    if (config.find(arg_name) != config.end())
    {
      size_t value = static_cast<size_t>(std::stoull(argv[i + 1]));
      config[arg_name] = value;
    }
  }

  string path = "data";

  std::uintmax_t n = fs::remove_all(path);
  std::cout << "Deleted " << n << " files or directories\n";

  auto x = create_tree(path);

  print_tree(x);

  return 0;
}