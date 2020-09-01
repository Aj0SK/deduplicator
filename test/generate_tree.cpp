#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
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

constexpr size_t kMaxDepth = 4;
constexpr size_t kMaxCount = 10;
constexpr size_t kStopRatio = 3;

struct DirTreeNode
{
  string name;
  bool directory = false;
  vector<shared_ptr<DirTreeNode>> desc = {};
  DirTreeNode(const string& name, bool is_directory)
      : name(name), directory(is_directory){};
  void add_desc(shared_ptr<DirTreeNode>&& x) { desc.push_back(std::move(x)); };
};

void create_file(const string& path, std::mt19937& gen, int type = 0)
{
  ofstream f(path);
  if (type == 0)
  {
    f << "Small file\n";
  }
  else if (type == 1)
  {
    f << gen();
  }

  f.close();
}

shared_ptr<DirTreeNode> create_tree(const string& root_dir)
{
  unsigned seed = std::chrono::system_clock::now().time_since_epoch().count();
  std::mt19937 gen(seed);

  shared_ptr<DirTreeNode> root = std::make_shared<DirTreeNode>(root_dir, true);
  fs::create_directory(root_dir);

  for (size_t i = 0; i < kMaxCount; ++i)
  {
    auto curr_root = root;
    int curr_depth = 0;
    string curr_path = root_dir;

    while (1)
    {
      int random = gen() % kStopRatio;
      if (random == 0 || curr_depth == kMaxDepth)
      {
        create_file(curr_path + "/" + to_string(gen()) + ".txt", gen, 1);
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

  for (const auto& x : root_dir->desc)
  {
    print_tree(x, tabs + 1);
  }
}

int main()
{
  // not sure of performance benefits
  // ios_base::sync_with_stdio(false);
  string path = "data";

  std::uintmax_t n = fs::remove_all(path);
  std::cout << "Deleted " << n << " files or directories\n";

  auto x = create_tree(path);

  print_tree(x);

  return 0;
}