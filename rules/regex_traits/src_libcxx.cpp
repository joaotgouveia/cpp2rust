#include <cstddef>
#include <regex>
#include <utility>

template <typename T1>
using t14 = typename std::regex_traits<T1>::char_class_type;

template <typename T1>
bool f13(const std::regex_traits<T1> &a0,
         typename std::regex_traits<T1>::char_type a1,
         typename std::regex_traits<T1>::char_class_type a2) {
  return a0.isctype(a1, a2);
}

template <typename T1>
int f14(const std::regex_traits<T1> &a0,
        typename std::regex_traits<T1>::char_type a1, int a2) {
  return a0.value(a1, a2);
}
