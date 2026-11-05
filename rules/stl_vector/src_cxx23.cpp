#include <utility>
#include <vector>

template <typename T1, typename T2>
auto f1(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f2(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
void f3(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap(a0, a1);
}
