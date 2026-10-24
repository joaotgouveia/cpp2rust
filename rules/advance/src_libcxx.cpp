#include <iterator>
#include <utility>

template <
    typename T1, typename T2,
    typename T3 = decltype(std::__convert_to_integral(std::declval<T2>())),
    std::__enable_if_t<std::is_integral<decltype(std::__convert_to_integral(
                           std::declval<T2>()))>::value,
                       int>
        T4 = 0>
void f1(T1 &a0, T2 a1) {
  return std::advance<T1, T2>(a0, std::move(a1));
}
