#include <iomanip>
#include <ios>
#include <istream>
#include <ostream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
using t21 = std::__quoted_output_proxy<T1, T2>;

template <typename T1, typename T2>
std::__quoted_output_proxy<T1, T2> f111(const T1 *a0, const T1 *a1, T1 a2,
                                        T1 a3) {
  return std::__quoted_output_proxy<T1, T2>(a0, a1, std::move(a2),
                                            std::move(a3));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::_IsSame<T2, void>::value || std::_IsSame<T2, T3>::value, int>
              T4 = 0>
std::basic_ostream<T1, T3> &f112(std::basic_ostream<T1, T3> &a0,
                                 const std::__quoted_output_proxy<T1, T2> &a1) {
  return operator<<(a0, a1);
}

template <typename T1, typename T2, typename T3>
using t22 = std::__quoted_proxy<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::__quoted_proxy<T1, T2, T3> f113(std::basic_string<T1, T2, T3> &a0, T1 a1,
                                     T1 a2) {
  return std::__quoted_proxy<T1, T2, T3>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f114(std::basic_istream<T1, T2> &a0,
                                 const std::__quoted_proxy<T1, T2, T3> &a1) {
  return operator>>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f115(std::basic_ostream<T1, T2> &a0,
                                 const std::__quoted_proxy<T1, T2, T3> &a1) {
  return operator<<(a0, a1);
}

template <typename T1, typename T2>
auto f116(std::basic_string_view<T1, T2> a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}
