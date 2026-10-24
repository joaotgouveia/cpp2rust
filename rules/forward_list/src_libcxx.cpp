#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
std::forward_list<T1, T2>
f83(typename std::forward_list<T1, T2>::size_type a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f84(typename std::forward_list<T1, T2>::size_type a0,
                              const T2 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f85(typename std::forward_list<T1, T2>::size_type a0,
                              const T1 &a1) {
  return std::forward_list<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f86(std::initializer_list<T1> a0) {
  return std::forward_list<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f87(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f88(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f89(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1,
    typename std::forward_list<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert_after(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2>
void f90(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f91(std::forward_list<T1, T2> &a0,
         typename std::forward_list<T1, T2>::size_type a1, const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::forward_list<T1, T2> f92(typename std::forward_list<T1, T2>::size_type a0,
                              const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f93(T3 a0, T3 a1) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::forward_list<T1, T2> f94(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
void f95(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::forward_list<T1, T2>::iterator
f96(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(std::move(a1), std::move(a2),
                                      std::move(a3));
}
