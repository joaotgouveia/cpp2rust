#include <algorithm>
#include <ios>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f11(const std::regex_traits<T1> &a0, T1 a1,
         typename std::regex_traits<T1>::char_class_type a2) {
  return a0.isctype(a1, a2);
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::char_class_type
f12(const std::regex_traits<T1> &a0, T2 a1, T2 a2, bool a3) {
  return a0.template lookup_classname<T2>(a1, a2, a3);
}
