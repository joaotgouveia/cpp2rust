#include <algorithm>
#include <condition_variable>
#include <utility>

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename T4 = typename std::enable_if<
        std::__sp_compatible_with<T3 *, T1 *>::value, void>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::__shared_ptr<T1, T2> f40(const std::__shared_ptr<T3, T2> &a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename T4 = typename std::enable_if<
        std::__sp_compatible_with<T3 *, T1 *>::value, void>::type,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T5 = 0>
std::__shared_ptr<T1, T2> f41(std::__shared_ptr<T3, T2> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T4 = 0>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f42(std::__shared_ptr<T1, T2> &a0, const std::__shared_ptr<T3, T2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <
    typename T1, std::_Lock_policy T2, typename T3,
    typename std::enable_if<(!(std::is_same<T3, T1>::value)), int>::type T4 = 0>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f43(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T3, T2> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}

template <
    typename T1, std::_Lock_policy T2,
    typename std::enable_if<(T2 == T2) && (std::is_array<T1>::value == false) &&
                                (std::is_void<T1>::value == true),
                            int>::type T3 = 0>
T1 *f44(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator->();
}

template <
    typename T1, std::_Lock_policy T2,
    typename std::enable_if<(T2 == T2) && (std::is_array<T1>::value == true) &&
                                (std::is_void<T1>::value == false),
                            int>::type T3 = 0>
typename std::__shared_ptr<T1, T2>::element_type &
f45(const std::__shared_ptr<T1, T2> &a0, std::ptrdiff_t a1) {
  return a0.operator[](a1);
}

template <typename T1, std::_Lock_policy T2>
bool f46(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f47(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f48(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f49(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f50(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f51(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f52(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f53(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f54(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f55(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f56(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f57(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f58(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f59(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
bool f60(std::nullptr_t a0, const std::__shared_ptr<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f61(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}
