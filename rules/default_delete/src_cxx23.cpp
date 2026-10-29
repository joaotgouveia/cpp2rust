#include <barrier>
#include <utility>

template <typename T1> using t1 = std::default_delete<T1[]>;

template <typename T1> std::default_delete<T1[]> f1() {
  return std::default_delete<T1[]>();
}

template <typename T1, typename T2>
std::default_delete<T1[]> f2(const std::default_delete<T2[]> &a0) {
  return std::default_delete<T1[]>(a0);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_convertible<T2 (*)[], T1 (*)[]>::value>::type
f3(const std::default_delete<T1[]> &a0, T2 *a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1> using t2 = std::default_delete<T1>;

template <typename T1> std::default_delete<T1> f4() {
  return std::default_delete<T1>();
}

template <typename T1> void f5(const std::default_delete<T1> &a0, T1 *a1) {
  return a0.operator()(a1);
}

template <typename T1, typename T2>
std::default_delete<T1> f6(const std::default_delete<T2> &a0) {
  return std::default_delete<T1>(a0);
}
