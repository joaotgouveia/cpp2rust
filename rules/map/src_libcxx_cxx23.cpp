#include <initializer_list>
#include <map>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f177(std::from_range_t a0, T5 &&a1, const T3 &a2,
                              const T4 &a3)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::map<T1, T2, T3, T4>::value_type>)
{
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f178(std::from_range_t a0, T5 &&a1, const T4 &a2)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::map<T1, T2, T3, T4>::value_type>)
{
  return std::map<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f179(std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::map<T1, T2, T3, T4>::value_type>)
{
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}
