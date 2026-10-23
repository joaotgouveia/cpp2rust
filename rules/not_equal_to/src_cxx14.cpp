#include <functional>
#include <string>
#include <utility>

using t2 = std::not_equal_to<void>;

using t3 = std::not_equal_to<>::is_transparent;

std::not_equal_to<void> f2() { return std::not_equal_to<void>(); }

std::not_equal_to<void> f3(const std::not_equal_to<void> &a0) {
  return std::not_equal_to<void>(a0);
}

std::not_equal_to<void> f4(std::not_equal_to<void> &&a0) {
  return std::not_equal_to<void>(std::move(a0));
}

std::not_equal_to<void> &f5(std::not_equal_to<void> &a0,
                            const std::not_equal_to<void> &a1) {
  return a0.operator=(a1);
}

std::not_equal_to<void> &f6(std::not_equal_to<void> &a0,
                            std::not_equal_to<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f7(const std::not_equal_to<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
