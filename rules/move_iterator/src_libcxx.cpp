#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f35(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1> std::move_iterator<T1> f36(T1 a0) {
  return std::make_move_iterator<T1>(a0);
}

template <typename T1, typename T2>
auto f37(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1)
    -> decltype(std::operator- <T1, T2>(a0, a1)) {
  return std::operator- <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f38(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f39(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f40(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f41(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
