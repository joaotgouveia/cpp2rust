#include <type_traits>
#include <utility>

template <typename... T1> using t6 = std::common_reference<T1...>;

template <typename... T1> using t7 = std::common_reference_t<T1...>;
