#include <type_traits>
#include <utility>

template <typename T1, typename... T2> using t2 = std::result_of<T1(T2...)>;
