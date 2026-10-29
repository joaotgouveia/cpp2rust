#include <type_traits>
#include <utility>

template <typename T1, typename... T2> using t1 = std::invoke_result<T1, T2...>;
