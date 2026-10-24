#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f71(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f72(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f73(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f74(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f75(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f76(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2>
bool f77(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f78(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f79(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f80(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
