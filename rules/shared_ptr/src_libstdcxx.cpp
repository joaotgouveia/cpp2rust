#include <algorithm>
#include <condition_variable>
#include <ostream>
#include <utility>

template <typename T1, typename T2, std::_Lock_policy T3>
T1 *f47(const std::__shared_ptr<T2, T3> &a0) {
  return std::get_deleter<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, std::_Lock_policy T4>
std::basic_ostream<T1, T2> &f48(std::basic_ostream<T1, T2> &a0,
                                const std::__shared_ptr<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_constructible<std::__shared_ptr<T1>, T2 *>::value>::type>
std::shared_ptr<T1> f49(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, T2 *, T3>::value>::type>
std::shared_ptr<T1> f50(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f51(std::nullptr_t a0, T2 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, T2 *, T3, T4>::value>::type>
std::shared_ptr<T1> f52(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1> f53(std::nullptr_t a0, T2 a1, T3 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f54(const std::shared_ptr<T2> &a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, const std::weak_ptr<T2> &>::value>::type>
std::shared_ptr<T1> f55(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, std::auto_ptr<T2>>::value>::type>
std::shared_ptr<T1> f56(std::auto_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          typename T4 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, std::unique_ptr<T2, T3>>::value>::type>
std::shared_ptr<T1> f57(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
typename std::enable_if<
    std::is_assignable<std::__shared_ptr<T1> &, std::auto_ptr<T2>>::value,
    std::shared_ptr<T1> &>::type
f58(std::shared_ptr<T1> &a0, std::auto_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::is_assignable<std::__shared_ptr<T1> &, std::unique_ptr<T2, T3>>::value,
    std::shared_ptr<T1> &>::type
f59(std::shared_ptr<T1> &a0, std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1>
typename std::shared_ptr<T1>::element_type *f60(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1> bool f61(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <typename T1, typename T2>
typename std::enable_if<std::__sp_is_constructible<T1, T2>::value>::type
f62(std::shared_ptr<T1> &a0, T2 *a1) {
  return a0.template reset<T2>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::__sp_is_constructible<T1, T2>::value>::type
f63(std::shared_ptr<T1> &a0, T2 *a1, T3 a2) {
  return a0.template reset<T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::__sp_is_constructible<T1, T2>::value>::type
f64(std::shared_ptr<T1> &a0, T2 *a1, T3 a2, T4 a3) {
  return a0.template reset<T2, T3, T4>(a1, a2, a3);
}

template <typename T1, typename T2>
bool f65(const std::shared_ptr<T1> &a0,
         const std::__shared_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1> T1 &f66(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f67(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}
