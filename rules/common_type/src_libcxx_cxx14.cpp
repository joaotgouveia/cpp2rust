#include <chrono>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1> using t41 = std::common_type_t<T1...>;
