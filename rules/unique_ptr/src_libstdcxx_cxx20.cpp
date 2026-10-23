#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> std::unique_ptr<T1> f59() {
  return std::make_unique_for_overwrite<T1>();
}

template <typename T1> auto f60(std::size_t a0) {
  return std::make_unique_for_overwrite<T1>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostream<T1, T2> &f61(std::basic_ostream<T1, T2> &a0,
                                const std::unique_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}
