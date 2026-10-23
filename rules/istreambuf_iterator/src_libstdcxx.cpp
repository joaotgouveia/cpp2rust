#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2>
f9(const std::istreambuf_iterator<T1, T2> &a0) {
  return std::istreambuf_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> &
f10(std::istreambuf_iterator<T1, T2> &a0,
    const std::istreambuf_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f11(std::istreambuf_iterator<T1, T2> &a0,
                                     int a1) {
  return a0.operator++(a1);
}
