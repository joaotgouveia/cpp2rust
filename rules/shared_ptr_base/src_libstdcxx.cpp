#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, std::_Lock_policy T2>
using t1 = std::__shared_ptr<T1, T2>;

template <typename T1, std::_Lock_policy T2>
using t2 = typename std::__shared_ptr<T1, T2>::element_type;

template <typename T1, std::_Lock_policy T2> std::__shared_ptr<T1, T2> f1() {
  return std::__shared_ptr<T1, T2>();
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f2(const std::__shared_ptr<T1, T2> &a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> &f3(std::__shared_ptr<T1, T2> &a0,
                              const std::__shared_ptr<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f4(std::__shared_ptr<T1, T2> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f5(std::nullptr_t a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> &f6(std::__shared_ptr<T1, T2> &a0,
                              std::__shared_ptr<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, std::_Lock_policy T2>
void f7(std::__shared_ptr<T1, T2> &a0) {
  return a0.reset();
}

template <typename T1, std::_Lock_policy T2>
typename std::__shared_ptr<T1, T2>::element_type *
f8(const std::__shared_ptr<T1, T2> &a0) {
  return a0.get();
}

template <typename T1, std::_Lock_policy T2>
bool f9(const std::__shared_ptr<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::_Lock_policy T2>
bool f10(const std::__shared_ptr<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, std::_Lock_policy T2>
long f11(const std::__shared_ptr<T1, T2> &a0) {
  return a0.use_count();
}

template <typename T1, std::_Lock_policy T2>
void f12(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_is_constructible<T1, T3>::value>::type>
std::__shared_ptr<T1, T2> f13(T3 *a0) {
  return std::__shared_ptr<T1, T2>(a0);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5 = typename std::enable_if<
              std::__sp_is_constructible<T1, T3>::value>::type>
std::__shared_ptr<T1, T2> f14(T3 *a0, T4 a1) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5,
          typename T6 = typename std::enable_if<
              std::__sp_is_constructible<T1, T3>::value>::type>
std::__shared_ptr<T1, T2> f15(T3 *a0, T4 a1, T5 a2) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1), std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2> f16(std::nullptr_t a0, T3 a1) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__shared_ptr<T1, T2> f17(std::nullptr_t a0, T3 a1, T4 a2) {
  return std::__shared_ptr<T1, T2>(a0, std::move(a1), std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2>
f18(const std::__shared_ptr<T3, T2> &a0,
    typename std::__shared_ptr<T1, T2>::element_type *a1) {
  return std::__shared_ptr<T1, T2>(a0, a1);
}

template <typename T1, std::_Lock_policy T2, typename T3>
std::__shared_ptr<T1, T2>
f19(std::__shared_ptr<T3, T2> &&a0,
    typename std::__shared_ptr<T1, T2>::element_type *a1) {
  return std::__shared_ptr<T1, T2>(std::move(a0), a1);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5 = std::__enable_if_t<
              std::__and_<
                  std::__sp_compatible_with<T3 *, T1 *>,
                  std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                                      typename std::remove_extent<T1>::type *>,
                  std::is_move_constructible<T4>>::value,
              void>>
std::__shared_ptr<T1, T2> f20(std::unique_ptr<T3, T4> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3,
          typename T4 = typename std::enable_if<
              std::__sp_compatible_with<T3 *, T1 *>::value, void>::type>
std::__shared_ptr<T1, T2> f21(std::auto_ptr<T3> &&a0) {
  return std::__shared_ptr<T1, T2>(std::move(a0));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_compatible_with<T3 *, T1 *>::value,
                        std::__shared_ptr<T1, T2> &>::type
f22(std::__shared_ptr<T1, T2> &a0, std::auto_ptr<T3> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
std::__enable_if_t<
    std::__and_<std::__sp_compatible_with<T3 *, T1 *>,
                std::is_convertible<typename std::unique_ptr<T3, T4>::pointer,
                                    typename std::remove_extent<T1>::type *>,
                std::is_move_constructible<T4>>::value,
    std::__shared_ptr<T1, T2> &>
f23(std::__shared_ptr<T1, T2> &a0, std::unique_ptr<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, std::_Lock_policy T2, typename T3>
typename std::enable_if<std::__sp_is_constructible<T1, T3>::value>::type
f24(std::__shared_ptr<T1, T2> &a0, T3 *a1) {
  return a0.template reset<T3>(a1);
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4>
typename std::enable_if<std::__sp_is_constructible<T1, T3>::value>::type
f25(std::__shared_ptr<T1, T2> &a0, T3 *a1, T4 a2) {
  return a0.template reset<T3, T4>(a1, std::move(a2));
}

template <typename T1, std::_Lock_policy T2, typename T3, typename T4,
          typename T5>
typename std::enable_if<std::__sp_is_constructible<T1, T3>::value>::type
f26(std::__shared_ptr<T1, T2> &a0, T3 *a1, T4 a2, T5 a3) {
  return a0.template reset<T3, T4, T5>(a1, std::move(a2), std::move(a3));
}

template <typename T1, std::_Lock_policy T2, typename T3>
bool f27(const std::__shared_ptr<T1, T2> &a0,
         const std::__shared_ptr<T3, T2> &a1) {
  return a0.template owner_before<T3>(a1);
}

template <typename T1, std::_Lock_policy T2>
T1 &f28(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, std::_Lock_policy T2>
T1 *f29(const std::__shared_ptr<T1, T2> &a0) {
  return a0.operator->();
}

using t3 = std::bad_weak_ptr;

const char *f30(const std::bad_weak_ptr &a0) { return a0.what(); }

std::bad_weak_ptr f31() { return std::bad_weak_ptr(); }

std::bad_weak_ptr f32(const std::bad_weak_ptr &a0) {
  return std::bad_weak_ptr(a0);
}

std::bad_weak_ptr &f33(std::bad_weak_ptr &a0, const std::bad_weak_ptr &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f34(const std::__shared_ptr<T2, T3> &a0) {
  return std::const_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f35(const std::__shared_ptr<T2, T3> &a0) {
  return std::dynamic_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, std::_Lock_policy T2>
bool f36(const std::__shared_ptr<T1, T2> &a0, std::nullptr_t a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
bool f37(const std::__shared_ptr<T1, T3> &a0,
         const std::__shared_ptr<T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::_Lock_policy T3>
std::__shared_ptr<T1, T3> f38(const std::__shared_ptr<T2, T3> &a0) {
  return std::static_pointer_cast<T1, T2, T3>(a0);
}

template <typename T1, std::_Lock_policy T2>
void f39(std::__shared_ptr<T1, T2> &a0, std::__shared_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
