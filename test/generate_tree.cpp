#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <random>
#include <string>
#include <thread>
#include <unordered_map>
#include <vector>

namespace fs = std::filesystem;

using namespace std::chrono_literals;

using std::cin;
using std::cout;
using std::ofstream;
using std::shared_ptr;
using std::string;
using std::string_view;
using std::to_string;
using std::vector;

std::unordered_map<string_view, size_t>& GetConfig()
{
  static auto* config = new std::unordered_map<string_view, size_t>(
      {{"kSeed", 21},
       {"kMaxDepth", 4},
       {"kMaxCount", 40},
       {"kStopRatio", 3},
       {"kMaxDupCount", 1},
       {"kFileMinSizeBlocks", 128},
       {"kFileMaxSizeBlocks", 256},
       {"kBlockSize", 1024}}); // 1'048'576}};
  return *config;
}

struct DirTreeNode
{
  string name;
  bool directory = false;
  vector<DirTreeNode> dirs = {};
  vector<DirTreeNode> files = {};
  DirTreeNode(string name, bool is_directory)
      : name(std::move(name)), directory(is_directory){};
  void add_desc(DirTreeNode&& x) { dirs.push_back(std::move(x)); };
  void add_file(DirTreeNode&& x) { files.push_back(std::move(x)); };
};

enum class FileType : int
{
  Small,
  Random,
  // Zero,
  // Keep last
  MaxType,
};

template <typename T>
void create_file(const string& path, T& gen, FileType type)
{
  ofstream f(path);
  auto& config = GetConfig();
  int size = gen() % (config["kFileMaxSizeBlocks"] + 1 -
                      config["kFileMinSizeBlocks"]) +
             config["kFileMinSizeBlocks"];
  switch (type)
  {
  case FileType::Small:
    f << std::uniform_int_distribution<size_t>()(gen);
    break;
  case FileType::Random:
  {
    string command = "/bin/dd if=/dev/urandom of=" + path +
                     " bs=" + to_string(config["kBlockSize"]) +
                     " count=" + to_string(size) + "> /dev/null 2>&1";
    std::system(command.c_str());
  }
  break;
  /*case FileType::Zero:
  {
    string command = "/bin/dd if=/dev/zero of=" + path +
                     " bs=" + to_string(config["kBlockSize"]) +
                     " count=" + to_string(size) + "> /dev/null 2>&1";
    std::system(command.c_str());
  }
  break;*/
  default:
    throw string("Unkonwn FileType");
  }
  f.close();
}

DirTreeNode create_tree(const string& root_dir)
{
  auto& config = GetConfig();
  std::mt19937 gen(config["kSeed"]);

  DirTreeNode root(root_dir, true);
  fs::create_directory(root_dir);

  size_t created_files = 0;
  while (created_files < config["kMaxCount"])
  {
    auto* curr_root = &root;
    int curr_depth = 0;
    string curr_path = root_dir;

    while (1)
    {
      int random =
          std::uniform_int_distribution<size_t>(0, config["kStopRatio"])(gen);
      if (random == 0 || curr_depth == config["kMaxDepth"])
      {
        string filename =
            to_string(std::uniform_int_distribution<size_t>()(gen)) + ".txt";
        int type = std::uniform_int_distribution<int>(
            0, static_cast<int>(FileType::MaxType) - 1)(gen);

        string original = curr_path + "/" + filename;
        create_file(original, gen, static_cast<FileType>(type));
        curr_root->add_file(DirTreeNode(filename, false));
        ++created_files;
        // add duplicate(s)

        size_t dup_count = std::uniform_int_distribution<size_t>(
            0, config["kMaxDupCount"])(gen);
        dup_count = std::min(dup_count, config["kMaxCount"] - created_files);

        if (dup_count == 0)
          break;

        cout << original;

        std::this_thread::sleep_for(10ms);

        for (int i = 0; i < dup_count; ++i)
        {
          string filename_dup = to_string(gen()) + ".txt";
          string duplicate = curr_path + "/" + filename_dup;

          fs::copy(original, duplicate);
          curr_root->add_file(DirTreeNode(filename_dup, false));
          cout << " " << duplicate;
        }

        cout << "\n";

        break;
      }

      int dir =
          std::uniform_int_distribution<size_t>(0, curr_root->dirs.size())(gen);
      if (dir == curr_root->dirs.size())
      {
        fs::create_directory(curr_path + "/" + to_string(created_files));
        curr_root->add_desc(DirTreeNode(to_string(created_files), true));
      }
      ++curr_depth;
      curr_path += "/" + curr_root->dirs[dir].name;
      curr_root = &curr_root->dirs[dir];
    }
  }

  return root;
}

void print_tree(const DirTreeNode& root_dir, int tabs = 0)
{
  string spaces(tabs, '\t');
  cout << spaces;
  if (root_dir.directory)
    cout << root_dir.name << " dir: \n";
  else
    cout << root_dir.name << "\n";

  for (const auto& x : root_dir.files)
  {
    cout << spaces << '\t';
    cout << "File " << x.name << "\n";
  }

  for (const auto& x : root_dir.dirs)
  {
    print_tree(x, tabs + 1);
  }
}

int main(int argc, char* argv[])
{
  // not sure of performance benefits
  // ios_base::sync_with_stdio(false);

  if (argc < 2 || (argc % 2) == 1)
  {
    std::cerr << "Bad number of params. Usage is " << argv[0] << " data_dir "
              << "[arg_name arg_value]*\n";
    return 1;
  }

  auto& config = GetConfig();
  {
    std::random_device rd;
    config["kSeed"] = std::uniform_int_distribution<size_t>()(rd);
  }
  for (size_t i = 2; i < argc; i += 2)
  {
    string arg_name(argv[i]);
    if (config.find(arg_name) != config.end())
    {
      size_t value = static_cast<size_t>(std::stoull(argv[i + 1]));
      config[arg_name] = value;
    }
    else
    {
      std::cerr << "Parameter with unknown name. Exiting!\n";
      return 1;
    }
  }

  string path = argv[1];
  fs::remove_all(path);
  // std::uintmax_t n = fs::remove_all(path);
  // std::cout << "Deleted " << n << " files or
  // directories\n";

  auto x = create_tree(path);

  // print_tree(x);

  return 0;
}
