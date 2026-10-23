#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f27(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1> std::move_iterator<T1> f28(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1> std::move_iterator<T1> f29(T1 a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
auto f30(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f31(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f32(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f33(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f34(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
