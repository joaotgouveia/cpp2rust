#include "../common/init.h"
#include "../common/traits.h"
#include <compare>
#include <utility>

using t1 = std::compare_three_way;

using t2 = std::compare_three_way::is_transparent;

std::compare_three_way f1() { return std::compare_three_way(); }

std::compare_three_way f2(const std::compare_three_way &a0) {
  return std::compare_three_way(a0);
}

std::compare_three_way f3(std::compare_three_way &&a0) {
  return std::compare_three_way(std::move(a0));
}

std::compare_three_way &f4(std::compare_three_way &a0,
                           const std::compare_three_way &a1) {
  return a0.operator=(a1);
}

std::compare_three_way &f5(std::compare_three_way &a0,
                           std::compare_three_way &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::compare_three_way &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}
