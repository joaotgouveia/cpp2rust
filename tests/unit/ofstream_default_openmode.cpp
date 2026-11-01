#include <cassert>
#include <fstream>

int main() {
  std::ofstream ofs("./ofstream_default_openmode.txt");
  std::ofstream app("./ofstream_default_openmode.txt", std::ios_base::app);
  assert(std::ios_base::out != std::ios_base::app);
  return 0;
}
