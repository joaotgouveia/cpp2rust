#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <stdlib.h>
#include <utility>

int f650(const pthread_attr_t *a0, sigset_t *a1) {
  return pthread_attr_getsigmask_np(a0, a1);
}

int f651(pthread_attr_t *a0, const sigset_t *a1) {
  return pthread_attr_setsigmask_np(a0, a1);
}
