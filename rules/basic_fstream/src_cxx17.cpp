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
std::basic_fstream<T1, T2>
f50(const T3 &a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}
