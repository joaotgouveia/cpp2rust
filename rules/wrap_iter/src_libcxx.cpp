#include <iterator>
#include <utility>

template <typename T1> using t1 = std::__wrap_iter<T1>;

template <typename T1> using t2 = typename std::__wrap_iter<T1>::value_type;

template <typename T1>
using t3 = typename std::__wrap_iter<T1>::difference_type;

template <typename T1> using t4 = typename std::__wrap_iter<T1>::pointer;

template <typename T1> using t5 = typename std::__wrap_iter<T1>::reference;

template <typename T1>
using t6 = typename std::__wrap_iter<T1>::iterator_category;

template <typename T1> std::__wrap_iter<T1> f1() {
  return std::__wrap_iter<T1>();
}

template <typename T1>
typename std::__wrap_iter<T1>::reference f2(const std::__wrap_iter<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__wrap_iter<T1>::pointer f3(const std::__wrap_iter<T1> &a0) {
  return a0.operator->();
}

template <typename T1> std::__wrap_iter<T1> &f4(std::__wrap_iter<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__wrap_iter<T1> f5(std::__wrap_iter<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> std::__wrap_iter<T1> &f6(std::__wrap_iter<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::__wrap_iter<T1> f7(std::__wrap_iter<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::__wrap_iter<T1> f8(const std::__wrap_iter<T1> &a0,
                        typename std::__wrap_iter<T1>::difference_type a1) {
  return a0.operator+(
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a1));
}

template <typename T1>
std::__wrap_iter<T1> &f9(std::__wrap_iter<T1> &a0,
                         typename std::__wrap_iter<T1>::difference_type a1) {
  return a0.operator+=(
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a1));
}

template <typename T1>
std::__wrap_iter<T1> f10(const std::__wrap_iter<T1> &a0,
                         typename std::__wrap_iter<T1>::difference_type a1) {
  return a0.operator-(
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a1));
}

template <typename T1>
std::__wrap_iter<T1> &f11(std::__wrap_iter<T1> &a0,
                          typename std::__wrap_iter<T1>::difference_type a1) {
  return a0.operator-=(
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a1));
}

template <typename T1>
typename std::__wrap_iter<T1>::reference
f12(const std::__wrap_iter<T1> &a0,
    typename std::__wrap_iter<T1>::difference_type a1) {
  return a0.operator[](
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a1));
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::_And<
            std::is_convertible<const T2 &, T1>,
            std::_Or<std::is_same<typename std::iterator_traits<T1>::reference,
                                  std::__iterator_reference<T2>>,
                     std::is_same<typename std::iterator_traits<T1>::reference,
                                  std::__make_const_lvalue_ref<
                                      std::__iterator_reference<T2>>>>>::value,
        int>
        T3 = 0>
std::__wrap_iter<T1> f13(const std::__wrap_iter<T2> &a0) {
  return std::__wrap_iter<T1>(a0);
}

template <typename T1>
std::__wrap_iter<T1> f14(typename std::__wrap_iter<T1>::difference_type a0,
                         std::__wrap_iter<T1> a1) {
  return operator+(
      std::forward<typename std::__wrap_iter<T1>::difference_type>(a0),
      std::forward<std::__wrap_iter<T1>>(a1));
}

template <typename T1, typename T2>
auto f15(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1)
    -> decltype(operator-(a0, a1)) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
bool f16(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1>
bool f17(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator==(a0, a1);
}
