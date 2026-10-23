#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, template <class> typename T3,
          template <class> typename T4>
  requires((std::__ref_wrap_common_reference_exists_with<T1, T2, T3<T1>,
                                                         T4<T2>> &&
            !std::__ref_wrap_common_reference_exists_with<T2, T1, T4<T2>,
                                                          T3<T1>>))
using t5 = std::basic_common_reference<T1, T2, T3, T4>;

template <typename T1, typename T2, template <class> typename T3,
          template <class> typename T4>
using t6 = typename std::basic_common_reference<T1, T2, T3, T4>::type;

template <typename T1, typename T2, template <class> typename T3,
          template <class> typename T4>
using t7 = std::basic_common_reference<T1, T2, T3, T4>;
