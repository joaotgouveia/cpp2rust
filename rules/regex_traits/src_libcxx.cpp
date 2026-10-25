#include <cstddef>
#include <ios>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f23(const std::regex_traits<T1> &a0, T1 a1, std::ctype_base::mask a2) {
  return a0.isctype(std::move(a1), a2);
}

template <typename T1, typename T2>
std::ctype_base::mask f24(const std::regex_traits<T1> &a0, T2 a1, T2 a2,
                          bool a3) {
  return a0.template lookup_classname<T2>(std::move(a1), std::move(a2), a3);
}
