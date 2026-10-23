#include <memory>
#include <utility>

using t5 = std::auto_ptr<void>;

using t6 = std::auto_ptr<void>::element_type;

std::auto_ptr<void> f22() { return std::auto_ptr<void>(); }

std::auto_ptr<void> f23(const std::auto_ptr<void> &a0) {
  return std::auto_ptr<void>(a0);
}

std::auto_ptr<void> f24(std::auto_ptr<void> &&a0) {
  return std::auto_ptr<void>(std::move(a0));
}

std::auto_ptr<void> &f25(std::auto_ptr<void> &a0,
                         const std::auto_ptr<void> &a1) {
  return a0.operator=(a1);
}

std::auto_ptr<void> &f26(std::auto_ptr<void> &a0, std::auto_ptr<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t7 = std::auto_ptr<T1>;

template <typename T1> using t8 = typename std::auto_ptr<T1>::element_type;

template <typename T1> std::auto_ptr<T1> f27(std::auto_ptr<T1> &a0) {
  return {a0};
}

template <typename T1>
std::auto_ptr<T1> &f28(std::auto_ptr<T1> &a0, std::auto_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::auto_ptr<T1> &f29(std::auto_ptr<T1> &a0, std::auto_ptr_ref<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f30(const std::auto_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f31(const std::auto_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f32(const std::auto_ptr<T1> &a0) { return a0.get(); }

template <typename T1> T1 *f33(std::auto_ptr<T1> &a0) { return a0.release(); }

template <typename T1> void f34(std::auto_ptr<T1> &a0, T1 *a1) {
  return a0.reset(a1);
}

template <typename T1> std::auto_ptr<T1> f35(std::auto_ptr_ref<T1> a0) {
  return {a0};
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::auto_ptr<T1> f36(std::auto_ptr<T2> &a0) {
  return {a0};
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::auto_ptr<T1> &f37(std::auto_ptr<T1> &a0, std::auto_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::auto_ptr_ref<T2> f38(std::auto_ptr<T1> &a0) {
  return static_cast<std::auto_ptr_ref<T2>>(a0);
}

template <typename T1, typename T2>
std::auto_ptr<T2> f39(std::auto_ptr<T1> &a0) {
  return static_cast<std::auto_ptr<T2>>(a0);
}
