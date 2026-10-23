#include <memory>
#include <utility>

using t1 = std::auto_ptr<void>;

using t2 = std::auto_ptr<void>::element_type;

std::auto_ptr<void> f1() { return std::auto_ptr<void>(); }

std::auto_ptr<void> f2(const std::auto_ptr<void> &a0) {
  return std::auto_ptr<void>(a0);
}

std::auto_ptr<void> f3(std::auto_ptr<void> &&a0) {
  return std::auto_ptr<void>(std::move(a0));
}

std::auto_ptr<void> &f4(std::auto_ptr<void> &a0,
                        const std::auto_ptr<void> &a1) {
  return a0.operator=(a1);
}

std::auto_ptr<void> &f5(std::auto_ptr<void> &a0, std::auto_ptr<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t3 = std::auto_ptr<T1>;

template <typename T1> using t4 = typename std::auto_ptr<T1>::element_type;

template <typename T1> std::auto_ptr<T1> f6(std::auto_ptr<T1> &a0) {
  return {a0};
}

template <typename T1>
std::auto_ptr<T1> &f7(std::auto_ptr<T1> &a0, std::auto_ptr<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f8(const std::auto_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 *f9(const std::auto_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f10(const std::auto_ptr<T1> &a0) { return a0.get(); }

template <typename T1> T1 *f11(std::auto_ptr<T1> &a0) { return a0.release(); }

template <typename T1> void f12(std::auto_ptr<T1> &a0, T1 *a1) {
  return a0.reset(a1);
}

template <typename T1> std::auto_ptr<T1> f13(std::auto_ptr_ref<T1> a0) {
  return {a0};
}

template <typename T1>
std::auto_ptr<T1> &f14(std::auto_ptr<T1> &a0, std::auto_ptr_ref<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::auto_ptr<T1> f15(std::auto_ptr<T2> &a0) {
  return {a0};
}

template <typename T1, typename T2>
std::auto_ptr<T1> &f16(std::auto_ptr<T1> &a0, std::auto_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::auto_ptr_ref<T2> f17(std::auto_ptr<T1> &a0) {
  return static_cast<std::auto_ptr_ref<T2>>(a0);
}

template <typename T1, typename T2>
std::auto_ptr<T2> f18(std::auto_ptr<T1> &a0) {
  return static_cast<std::auto_ptr<T2>>(a0);
}
