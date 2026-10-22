#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::insert_iterator<T1>;

template <typename T1>
using t2 = typename std::insert_iterator<T1>::container_type;

template <typename T1>
std::insert_iterator<T1> &f1(std::insert_iterator<T1> &a0,
                             const typename T1::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::insert_iterator<T1> &f2(std::insert_iterator<T1> &a0,
                             typename T1::value_type &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::insert_iterator<T1> &f3(std::insert_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::insert_iterator<T1> &f4(std::insert_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::insert_iterator<T1> &f5(std::insert_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}
