#include <cassert>
#include <fstream>

int main() {
  std::ofstream ofs("./ifstream_rdbuf_not_null.txt");
  std::filebuf *ob = ofs.rdbuf();
  assert(ob != nullptr);
  std::ifstream ifs("./ifstream_rdbuf_not_null.txt");
  auto buf = ifs.rdbuf();
  assert(buf != nullptr);
  std::filebuf *fb = ifs.rdbuf();
  assert(fb != nullptr);
  return 0;
}
