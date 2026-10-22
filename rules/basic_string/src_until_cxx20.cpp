#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
void f65(std::basic_string<T1, T2, T3> &a0) {
  return a0.reserve();
}
