#include <iomanip>
#include <ios>
#include <istream>
#include <ostream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
auto f1(T1 &a0, bool a1) -> decltype(std::get_money<T1>(a0, a1)) {
  return std::get_money<T1>(a0, a1);
}

template <typename T1>
auto f2(std::tm *a0, const T1 *a1) -> decltype(std::get_time<T1>(a0, a1)) {
  return std::get_time<T1>(a0, a1);
}

template <typename T1>
auto f3(const T1 &a0, bool a1) -> decltype(std::put_money<T1>(a0, a1)) {
  return std::put_money<T1>(a0, a1);
}

template <typename T1>
auto f4(const std::tm *a0, const T1 *a1)
    -> decltype(std::put_time<T1>(a0, a1)) {
  return std::put_time<T1>(a0, a1);
}

template <typename T1> auto f5(T1 a0) -> decltype(std::setfill<T1>(a0)) {
  return std::setfill<T1>(a0);
}
