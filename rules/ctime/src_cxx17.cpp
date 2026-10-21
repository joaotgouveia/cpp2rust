#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <stdlib.h>
#include <utility>

int f10(struct timespec *a0, int a1) { return timespec_get(a0, a1); }
