#include <regex>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f8(const std::regex_iterator<T1, T2, T3> &a0,
        const std::regex_iterator<T1, T2, T3> &a1) {
  return a0.operator!=(a1);
}
