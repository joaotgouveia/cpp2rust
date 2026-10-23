#include <compare>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1> using t3 = std::common_comparison_category_t<T1...>;
