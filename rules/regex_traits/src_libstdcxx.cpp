#include <algorithm>
#include <regex>
#include <utility>

template <typename T1>
bool f11(const std::regex_traits<T1> &a0, T1 a1,
         typename std::regex_traits<T1>::char_class_type a2) {
  return a0.isctype(a1, a2);
}

template <typename T1> int f12(const std::regex_traits<T1> &a0, T1 a1, int a2) {
  return a0.value(a1, a2);
}
