#include "../common/init.h"
#include "../common/traits.h"
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f4(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.contains(a1);
}
