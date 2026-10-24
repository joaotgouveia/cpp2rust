#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::__enable_if_t<std::is_convertible<T3, T1>::value>>
__gnu_cxx::__normal_iterator<T1, T2>
f35(const __gnu_cxx::__normal_iterator<T3, T2> &a0) {
  return __gnu_cxx::__normal_iterator<T1, T2>(a0);
}

template <typename T1>
std::insert_iterator<T1> f36(T1 &a0, typename T1::iterator a1) {
  return std::inserter<T1>(a0, std::move(a1));
}

template <typename T1>
bool f37(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f38(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f39(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
bool f40(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f41(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f42(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f43(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f44(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f45(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f46(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f47(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}
