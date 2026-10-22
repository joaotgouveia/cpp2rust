#include "../common/init.h"
#include "../common/traits.h"
#include <filesystem>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
void f75(std::basic_ofstream<T1, T2> &a0, const std::filesystem::path &a1,
         typename std::basic_ofstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}
