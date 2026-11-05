#include <barrier>
#include <utility>

using t1 = std::bit_xor<void>;

using t2 = std::bit_xor<>::is_transparent;

std::bit_xor<void> f1() { return std::bit_xor<void>(); }

std::bit_xor<void> f2(const std::bit_xor<void> &a0) {
  return std::bit_xor<void>(a0);
}

std::bit_xor<void> f3(std::bit_xor<void> &&a0) {
  return std::bit_xor<void>(std::move(a0));
}

std::bit_xor<void> &f4(std::bit_xor<void> &a0, const std::bit_xor<void> &a1) {
  return a0.operator=(a1);
}

std::bit_xor<void> &f5(std::bit_xor<void> &a0, std::bit_xor<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
auto f6(const std::bit_xor<void> &a0, T1 &&a1, T2 &&a2) {
  return a0.template operator()<T1, T2>(std::forward<T1>(a1),
                                        std::forward<T2>(a2));
}

template <typename T1> using t3 = std::bit_xor<T1>;

template <typename T1>
T1 f7(const std::bit_xor<T1> &a0, const T1 &a1, const T1 &a2) {
  return a0.operator()(a1, a2);
}
