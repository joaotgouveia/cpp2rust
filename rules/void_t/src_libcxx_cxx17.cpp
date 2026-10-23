#include <type_traits>
#include <utility>

template <typename... T1> using t1 = std::void_t<T1...>;
