#include <barrier>
#include <iomanip>
#include <istream>
#include <utility>

template <typename T1, typename T2>
using t1 = std::__detail::_Quoted_string<T1, T2>;

template <typename T1, typename T2>
std::__detail::_Quoted_string<T1, T2> f1(T1 a0, T2 a1, T2 a2) {
  return std::__detail::_Quoted_string<T1, T2>(std::move(a0), std::move(a1),
                                               std::move(a2));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f2(std::basic_ostream<T1, T2> &a0,
   const std::__detail::_Quoted_string<const T1 *, T1> &a1) {
  return std::__detail::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &
f3(std::basic_ostream<T1, T2> &a0,
   const std::__detail::_Quoted_string<T3, T1> &a1) {
  return std::__detail::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &
f4(std::basic_istream<T1, T2> &a0,
   const std::__detail::_Quoted_string<std::basic_string<T1, T2, T3> &, T1>
       &a1) {
  return std::__detail::operator>> <T1, T2, T3>(a0, a1);
}
