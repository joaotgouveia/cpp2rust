#include "../common/init.h"
#include "../common/traits.h"
#include <csignal>
#include <cstdint>
#include <stdlib.h>
#include <utility>

int f158(int a0, const sigset_t *__restrict a1, sigset_t *__restrict a2) {
  return pthread_sigmask(a0, a1, a2);
}
