#include <algorithm>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::regex_constants::syntax_option_type f36() {
  return std::basic_regex<T1, T2>::multiline;
}
