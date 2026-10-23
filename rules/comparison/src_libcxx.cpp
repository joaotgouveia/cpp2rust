#include <utility>
#include <vector>

template <typename T1, typename T2>
bool f6(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}
