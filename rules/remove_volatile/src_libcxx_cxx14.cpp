#include <type_traits>
#include <utility>

template <typename T1> using t3 = std::remove_volatile_t<T1>;
