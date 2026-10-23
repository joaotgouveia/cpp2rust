#include <iomanip>
#include <ios>
#include <istream>
#include <ostream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
auto f24(std::basic_string_view<T1, T2> a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2>(a0, a1, a2);
}
