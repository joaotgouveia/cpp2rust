#include <chrono>
#include <utility>

using t1 = std::owner_less<void>;

std::owner_less<void> f1() { return std::owner_less<void>(); }

std::owner_less<void> f2(const std::owner_less<void> &a0) {
  return std::owner_less<void>(a0);
}

std::owner_less<void> f3(std::owner_less<void> &&a0) {
  return std::owner_less<void>(std::move(a0));
}

std::owner_less<void> &f4(std::owner_less<void> &a0,
                          const std::owner_less<void> &a1) {
  return a0.operator=(a1);
}

std::owner_less<void> &f5(std::owner_less<void> &a0,
                          std::owner_less<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::owner_less<void> &a0, const T1 &a1, const T2 &a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1> using t2 = std::owner_less<std::shared_ptr<T1>>;

template <typename T1>
bool f7(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::shared_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f8(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::shared_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f9(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::weak_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1> using t3 = std::owner_less<std::weak_ptr<T1>>;

template <typename T1>
bool f10(const std::owner_less<std::weak_ptr<T1>> &a0,
         const std::weak_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f11(const std::owner_less<std::weak_ptr<T1>> &a0,
         const std::weak_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f12(const std::owner_less<std::weak_ptr<T1>> &a0,
         const std::shared_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}
