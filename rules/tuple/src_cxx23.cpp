#include <algorithm>
#include <array>
#include <compare>
#include <tuple>
#include <type_traits>
#include <utility>

void f30(const std::tuple<> &a0, const std::tuple<> &a1) { return a0.swap(a1); }
