#include <type_traits>
#include <utility>

template <typename T1> using t2 = std::underlying_type_t<T1>;
