#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename... T4>
using t6 = std::common_reference<T1, T2, T3, T4...>;

template <typename... T1> using t7 = std::common_reference<T1...>;

template <typename... T1> using t8 = std::common_reference_t<T1...>;
