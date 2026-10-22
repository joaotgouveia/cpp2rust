#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ofstream<T1, T2>
f44(const T3 &a0, typename std::basic_ofstream<T1, T2>::openmode a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}
