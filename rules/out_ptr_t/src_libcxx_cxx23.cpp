#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3>
void **f4(const std::out_ptr_t<T1, T2, T3...> &a0)
  requires((!std::is_same_v<T2, void *>))
{
  return static_cast<void **>(a0);
}
