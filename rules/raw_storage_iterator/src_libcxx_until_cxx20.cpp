#include <memory>
#include <utility>

template <typename T1, typename T2>
using t2 = std::raw_storage_iterator<T1, T2>;

template <typename T1, typename T2>
using t3 = typename std::raw_storage_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t4 = typename std::raw_storage_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t5 = typename std::raw_storage_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t6 = typename std::raw_storage_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::raw_storage_iterator<T1, T2>::reference;

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> f8(T1 a0) {
  return std::raw_storage_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f9(std::raw_storage_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f10(std::raw_storage_iterator<T1, T2> &a0,
                                       const T2 &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f11(std::raw_storage_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> f12(std::raw_storage_iterator<T1, T2> &a0,
                                      int a1) {
  return a0.operator++(a1);
}
