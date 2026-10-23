#include <scoped_allocator>
#include <utility>

template <typename T1, typename T2>
bool f3(const std::scoped_allocator_adaptor<T1> &a0,
        const std::scoped_allocator_adaptor<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
bool f4(const std::scoped_allocator_adaptor<T1, T3, T4...> &a0,
        const std::scoped_allocator_adaptor<T2, T3, T4...> &a1) {
  return std::operator== <T1, T2, T3, T4...>(a0, a1);
}
