#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t1 = std::pair<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::pair<T1, T2>::first_type;

template <typename T1, typename T2>
using t3 = typename std::pair<T1, T2>::second_type;

template <typename T1, typename T2>
std::pair<T1, T2> f1(const std::pair<T1, T2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2>
std::pair<T1, T2> f2(std::pair<T1, T2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f3(std::pair<T1, T2> &a0, std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::pair<T1, T2> f4(std::piecewise_construct_t a0, std::tuple<T3...> a1,
                     std::tuple<T4...> a2) {
  return std::pair<T1, T2>(std::move(a0), a1, a2);
}
