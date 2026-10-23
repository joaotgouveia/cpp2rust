#include <algorithm>
#include <barrier>
#include <chrono>
#include <format>
#include <queue>
#include <stack>
#include <string>
#include <string_view>
#include <thread>
#include <utility>
#include <vector>

template <typename T1> void f1(std::formatter<const T1 *, T1> &a0) {
  return a0.set_debug_format();
}
