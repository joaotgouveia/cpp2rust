#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::has_virtual_destructor<T1>;
