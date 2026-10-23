#include <condition_variable>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::owner_less<std::shared_ptr<T1>>;

template <typename T1>
bool f1(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::shared_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f2(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::shared_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f3(const std::owner_less<std::shared_ptr<T1>> &a0,
        const std::weak_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1> using t2 = std::owner_less<std::weak_ptr<T1>>;

template <typename T1>
bool f4(const std::owner_less<std::weak_ptr<T1>> &a0,
        const std::weak_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f5(const std::owner_less<std::weak_ptr<T1>> &a0,
        const std::weak_ptr<T1> &a1, const std::shared_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1>
bool f6(const std::owner_less<std::weak_ptr<T1>> &a0,
        const std::shared_ptr<T1> &a1, const std::weak_ptr<T1> &a2) {
  return a0.operator()(a1, a2);
}
