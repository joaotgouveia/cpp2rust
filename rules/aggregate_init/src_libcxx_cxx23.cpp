#include <memory>

template <typename T1, typename T2>
std::allocation_result<T1, T2> f16(T1 a0, T2 a1) {
  return {a0, a1};
}
