#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::is_member_function_pointer<T1>;
