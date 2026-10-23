#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::is_scoped_enum<T1>;
