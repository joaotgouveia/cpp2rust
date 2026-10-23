#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
T3 f228(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
auto f229(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f230(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
