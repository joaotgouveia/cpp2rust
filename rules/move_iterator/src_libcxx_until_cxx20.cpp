#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1> std::move_iterator<T1> f34() {
  return std::move_iterator<T1>();
}

template <typename T1>
std::move_iterator<T1> f35(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_same<T2, T1>::value &&
                                 std::is_convertible<const T2 &, T1>::value,
                             int>
              T3 = 0>
std::move_iterator<T1> f36(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_same<T2, T1>::value &&
                                 std::is_convertible<const T2 &, T1>::value &&
                                 std::is_assignable<T1 &, const T2 &>::value,
                             int>
              T3 = 0>
std::move_iterator<T1> &f37(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1>
std::move_iterator<T1> f38(typename std::move_iterator<T1>::difference_type a0,
                           const std::move_iterator<T1> &a1) {
  return std::operator+ <T1>(
      std::forward<typename std::move_iterator<T1>::difference_type>(a0), a1);
}
