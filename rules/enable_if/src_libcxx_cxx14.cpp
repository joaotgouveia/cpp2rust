#include <type_traits>
#include <utility>

template <bool T1, typename T2> using t4 = std::enable_if_t<T1, T2>;
