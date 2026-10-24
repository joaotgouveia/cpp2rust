#include <iterator>
#include <utility>

using t1 = std::unreachable_sentinel_t;

std::unreachable_sentinel_t f1() { return std::unreachable_sentinel_t(); }

std::unreachable_sentinel_t f2(const std::unreachable_sentinel_t &a0) {
  return std::unreachable_sentinel_t(a0);
}

std::unreachable_sentinel_t f3(std::unreachable_sentinel_t &&a0) {
  return std::unreachable_sentinel_t(std::move(a0));
}

std::unreachable_sentinel_t &f4(std::unreachable_sentinel_t &a0,
                                const std::unreachable_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::unreachable_sentinel_t &f5(std::unreachable_sentinel_t &a0,
                                std::unreachable_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
bool f6(std::unreachable_sentinel_t a0, const T1 &a1)
  requires(std::weakly_incrementable<T1>)
{
  return operator==(std::move(a0), a1);
}
