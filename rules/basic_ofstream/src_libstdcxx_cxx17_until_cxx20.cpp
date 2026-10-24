#include <algorithm>
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::_If_fs_path<T3>,
          typename std::enable_if<(!(std::is_same<T3, std::string>::value)),
                                  int>::type T5 = 0>
std::basic_ofstream<T1, T2>
f69(const T3 &a0, typename std::basic_ofstream<T1, T2>::openmode a1) {
  return std::basic_ofstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<(!(std::is_same<T3, std::string>::value)),
                                  int>::type T4 = 0>
std::_If_fs_path<T3, void>
f70(std::basic_ofstream<T1, T2> &a0, const T3 &a1,
    typename std::basic_ofstream<T1, T2>::openmode a2) {
  return a0.template open<T3>(a1, a2);
}
