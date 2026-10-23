#include <type_traits>
#include <utility>

template <typename T1> using t3 = std::remove_all_extents_t<T1>;
