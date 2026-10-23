#include <bitset>
#include <cstddef>
#include <memory>
#include <utility>

template <std::size_t T1, typename T2,
          std::__enable_if_t<std::_IsCharLikeType<T2>::value, int> T3 = 0>
std::bitset<T1> f46(const T2 *a0, std::size_t a1, T2 a2, T2 a3) {
  return std::bitset<T1>(a0, a1, a2, a3);
}

template <std::size_t T1, typename T2, typename T3, typename T4>
std::bitset<T1> f47(const std::basic_string<T2, T3, T4> &a0,
                    typename std::basic_string<T2, T3, T4>::size_type a1,
                    typename std::basic_string<T2, T3, T4>::size_type a2, T2 a3,
                    T2 a4) {
  return std::bitset<T1>(a0, a1, a2, a3, a4);
}
