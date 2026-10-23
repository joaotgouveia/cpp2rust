#include <algorithm>
#include <codecvt>
#include <cstddef>
#include <utility>

template <typename T1, unsigned long T2, std::codecvt_mode T3>
using t1 = std::codecvt_utf8_utf16<T1, T2, T3>;

template <typename T1, unsigned long T2, std::codecvt_mode T3>
std::codecvt_utf8_utf16<T1, T2, T3> f1(std::size_t a0) {
  return std::codecvt_utf8_utf16<T1, T2, T3>(a0);
}
