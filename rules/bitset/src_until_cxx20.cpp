#include <algorithm>
#include <bitset>
#include <utility>

template <std::size_t T1>
bool f34(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator!=(a1);
}

template <std::size_t T1, typename T2,
          typename std::enable_if<(!(std::is_same<T2, char>::value)), int>::type
              T3 = 0>
std::basic_string<T2, std::char_traits<T2>, std::allocator<T2>>
f35(const std::bitset<T1> &a0, T2 a1, T2 a2) {
  return a0.template to_string<T2>(std::move(a1), std::move(a2));
}
