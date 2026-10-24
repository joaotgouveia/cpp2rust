#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, std::_Lock_policy T2>
using t4 = typename std::__shared_ptr<T1, T2>::weak_type;

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f60(const std::__weak_ptr<T3, T2> &a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f61(const std::__shared_ptr<T1, T2> &a0,
         const std::__weak_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2> using t5 = std::__weak_ptr<T1, T2>;

template <typename T1, std::_Lock_policy T2>
using t6 = typename std::__weak_ptr<T1, T2>::element_type;

template <typename T1, std::_Lock_policy T2> std::__weak_ptr<T1, T2> f62() {
  return std::__weak_ptr<T1, T2>();
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> f63(const std::__weak_ptr<T1, T2> &a0) {
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> f64(std::__weak_ptr<T1, T2> &&a0) {
  return std::__weak_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> &f65(std::__weak_ptr<T1, T2> &a0,
                             const std::__weak_ptr<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::_Lock_policy T2>
std::__weak_ptr<T1, T2> &f66(std::__weak_ptr<T1, T2> &a0,
                             std::__weak_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f67(const std::__weak_ptr<T1, T2> &a0) {
  return a0.lock();
}

template <typename T1, std::_Lock_policy T2>
long f68(const std::__weak_ptr<T1, T2> &a0) {
  return a0.use_count();
}

template <typename T1, std::_Lock_policy T2>
bool f69(const std::__weak_ptr<T1, T2> &a0) {
  return a0.expired();
}

template <typename T1, std::_Lock_policy T2>
void f70(std::__weak_ptr<T1, T2> &a0) {
  return a0.reset();
}

template <typename T1, std::_Lock_policy T2>
void f71(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__weak_ptr<T1, T2> f72(const std::__shared_ptr<T3, T2> &a0) {
  return std::__weak_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__weak_ptr<T1, T2> &>::type
f73(std::__weak_ptr<T1, T2> &a0, const std::__shared_ptr<T3, T2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f74(const std::__weak_ptr<T1, T2> &a0,
         const std::__shared_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f75(const std::__weak_ptr<T1, T2> &a0, const std::__weak_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f76(const std::__shared_ptr<T2, T3> &a0) {
  return std::reinterpret_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, std::_Lock_policy T2>
void f77(std::__weak_ptr<T1, T2> &a0, std::__weak_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
