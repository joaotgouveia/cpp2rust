#include <filesystem>
#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
void f103(std::basic_ifstream<T1, T2> &a0, const std::filesystem::path &a1,
          unsigned int a2) {
  return a0.open(a1, a2);
}
