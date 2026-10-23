#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3>
void **f3(const std::inout_ptr_t<T1, T2, T3...> &a0)
  requires((!std::same_as<T2, void *>))
{
  return static_cast<void **>(a0);
}
