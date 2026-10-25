#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3,
          typename T4 = std::__enable_if_t<std::is_convertible<T3, T1>::value>>
__gnu_cxx::__normal_iterator<T1, T2>
f39(const __gnu_cxx::__normal_iterator<T3, T2> &a0) {
  return __gnu_cxx::__normal_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
bool f40(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f41(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f42(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f43(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f44(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f45(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f46(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f47(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
bool f48(const __gnu_cxx::__normal_iterator<T1, T2> &a0,
         const __gnu_cxx::__normal_iterator<T1, T2> &a1) {
  return __gnu_cxx::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f49(const __gnu_cxx::__normal_iterator<T1, T3> &a0,
         const __gnu_cxx::__normal_iterator<T2, T3> &a1) {
  return __gnu_cxx::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1>
std::insert_iterator<T1> f50(T1 &a0, typename T1::iterator a1) {
  return std::inserter<T1>(a0, std::forward<typename T1::iterator>(a1));
}

template <typename T1>
bool f51(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f52(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f53(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1>
bool f54(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f55(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f56(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f57(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f58(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f59(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f60(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f61(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}
