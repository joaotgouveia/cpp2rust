#include <memory>
#include <utility>

template <typename T1> using t1 = std::auto_ptr_ref<T1>;

template <typename T1> std::auto_ptr_ref<T1> f1(T1 *a0) {
  return std::auto_ptr_ref<T1>(a0);
}
