#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <ostream>
#include <thread>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f6(std::basic_ostream<T1, T2> &a0,
                               std::thread::id a1) {
  return a0 << std::move(a1);
}
