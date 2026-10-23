#include <algorithm>
#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
void f59(std::basic_string<T1, T2, T3> &a0) {
  return a0.reserve();
}
