#include <memory>
#include <utility>

template <typename T1> std::auto_ptr<T1> f19(T1 *a0) {
  return std::auto_ptr<T1>(a0);
}
