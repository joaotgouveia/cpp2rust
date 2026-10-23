#include <memory>
#include <utility>

template <typename T1, typename T2>
using t1 = std::raw_storage_iterator<T1, T2>;

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> f1(T1 a0) {
  return std::raw_storage_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f2(std::raw_storage_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f3(std::raw_storage_iterator<T1, T2> &a0,
                                      const T2 &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> &f4(std::raw_storage_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::raw_storage_iterator<T1, T2> f5(std::raw_storage_iterator<T1, T2> &a0,
                                     int a1) {
  return a0.operator++(a1);
}
