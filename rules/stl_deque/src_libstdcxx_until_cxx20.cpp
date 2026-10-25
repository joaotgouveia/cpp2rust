#include <algorithm>
#include <deque>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::_Require<
        std::is_same<std::_Deque_iterator<T1, T2, T3>,
                     std::_Deque_iterator<T1, const T1 &,
                                          std::__ptr_rebind<T3, const T1>>>,
        std::is_same<
            T4, std::_Deque_iterator<T1, T1 &, std::__ptr_rebind<T3, T1>>>>,
    typename std::enable_if<
        (!(std::is_same<T4, std::_Deque_iterator<T1, T2, T3>>::value)),
        int>::type T6 = 0>
std::_Deque_iterator<T1, T2, T3> f23(const T4 &a0) {
  return std::_Deque_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f24(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f25(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f26(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f27(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f28(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f29(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f30(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f31(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f32(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T4, T5> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f33(const std::_Deque_iterator<T1, T2, T3> &a0,
         const std::_Deque_iterator<T1, T2, T3> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f35(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f36(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f37(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f38(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
