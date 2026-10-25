#include <algorithm>
#include <ios>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f22(const typename std::regex_traits<T1>::_RegexMask &a0,
         typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator!=(std::move(a1));
}
