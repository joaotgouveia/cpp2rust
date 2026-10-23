#include <filesystem>
#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

template <typename T1, typename T2>
void f88(std::basic_ifstream<T1, T2> &a0, const std::filesystem::path &a1,
         typename std::basic_ifstream<T1, T2>::openmode a2) {
  return a0.open(a1, a2);
}

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::enable_if_t<std::is_same_v<T3, std::filesystem::path>>>
std::basic_ifstream<T1, T2>
f89(const T3 &a0, typename std::basic_ifstream<T1, T2>::openmode a1) {
  return std::basic_ifstream<T1, T2>(a0, a1);
}
