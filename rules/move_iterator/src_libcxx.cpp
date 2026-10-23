#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f28(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 == a1;
}

template <typename T1> std::move_iterator<T1> f29(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f30(T1 a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
auto f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
bool f32(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f34(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f35(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return a0 >= a1;
}
