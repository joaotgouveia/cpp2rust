#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f29(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1>
std::reverse_iterator<T1>
f30(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1, typename T2>
auto f31(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f32(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f35(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1>
using t9 = typename std::reverse_iterator<T1>::iterator_category;

template <typename T1>
using t10 = typename std::reverse_iterator<T1>::value_type;

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_same<T2, T1>::value &&
                                 std::is_convertible<const T2 &, T1>::value,
                             int>
              T3 = 0>
std::reverse_iterator<T1> f36(const std::reverse_iterator<T2> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<!std::is_same<T2, T1>::value &&
                                 std::is_convertible<const T2 &, T1>::value &&
                                 std::is_assignable<T1 &, const T2 &>::value,
                             int>
              T3 = 0>
std::reverse_iterator<T1> &f37(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
