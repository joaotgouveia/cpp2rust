#include <algorithm>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<std::__is_swappable<T2>::value>::type
f60(std::unique_ptr<T1, T2> &a0, std::unique_ptr<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
