#include <chrono>
#include <type_traits>
#include <utility>

template <typename... T1> using t26 = std::common_type_t<T1...>;
