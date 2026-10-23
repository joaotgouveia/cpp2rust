#include <type_traits>
#include <utility>

template <typename T1, typename T2, template <typename> typename T3,
          template <typename> typename T4>
using t1 = std::basic_common_reference<T1, T2, T3, T4>;
