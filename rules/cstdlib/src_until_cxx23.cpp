#include "../common/init.h"
#include "../common/traits.h"
#include <cstdlib>
#include <stdlib.h>
#include <utility>

int f138(int a0, fd_set *__restrict a1, fd_set *__restrict a2,
         fd_set *__restrict a3, const struct timespec *__restrict a4,
         const sigset_t *__restrict a5) {
  return pselect(a0, a1, a2, a3, a4, a5);
}
