#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1[], T2>::element_type>::type
f58(const std::unique_ptr<T1[], T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::unique_ptr<T1[], T2> f59(T3 a0, T4 &&a1) {
  return std::unique_ptr<T1[], T2>(a0, std::move(a1));
}

template <typename T1, typename T2>
typename std::add_lvalue_reference<
    typename std::unique_ptr<T1, T2>::element_type>::type
f60(const std::unique_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f61(typename std::unique_ptr<T1, T2>::pointer a0,
                            T3 &&a1) {
  return std::unique_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::unique_ptr<T1, T2> f62(std::auto_ptr<T3> &&a0) {
  return std::unique_ptr<T1, T2>(std::move(a0));
}
