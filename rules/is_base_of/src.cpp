#include <type_traits>
#include <utility>

template <typename T1, typename T2> using t1 = std::is_base_of<T1, T2>;
