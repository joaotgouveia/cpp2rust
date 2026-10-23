#include <type_traits>
#include <utility>

template <typename T1> using t3 = std::result_of_t<T1>;
