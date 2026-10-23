#include <memory>
#include <utility>

using t4 = std::owner_less<void>;

using t5 = std::owner_less<>::is_transparent;

std::owner_less<void> f12() { return std::owner_less<void>(); }

std::owner_less<void> f13(const std::owner_less<void> &a0) {
  return std::owner_less<void>(a0);
}

std::owner_less<void> f14(std::owner_less<void> &&a0) {
  return std::owner_less<void>(std::move(a0));
}

std::owner_less<void> &f15(std::owner_less<void> &a0,
                           const std::owner_less<void> &a1) {
  return a0.operator=(a1);
}

std::owner_less<void> &f16(std::owner_less<void> &a0,
                           std::owner_less<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f17(const std::owner_less<void> &a0, const std::shared_ptr<T1> &a1,
         const std::shared_ptr<T2> &a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1, typename T2>
bool f18(const std::owner_less<void> &a0, const std::shared_ptr<T1> &a1,
         const std::weak_ptr<T2> &a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1, typename T2>
bool f19(const std::owner_less<void> &a0, const std::weak_ptr<T1> &a1,
         const std::shared_ptr<T2> &a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}

template <typename T1, typename T2>
bool f20(const std::owner_less<void> &a0, const std::weak_ptr<T1> &a1,
         const std::weak_ptr<T2> &a2) {
  return a0.template operator()<T1, T2>(a1, a2);
}
