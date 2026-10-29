#include <barrier>
#include <utility>

template <typename T1, typename T2>
std::binder1st<T1> f1(const T1 &a0, const T2 &a1) {
  return std::bind1st(a0, a1);
}

template <typename T1, typename T2>
std::binder2nd<T1> f2(const T1 &a0, const T2 &a1) {
  return std::bind2nd(a0, a1);
}
