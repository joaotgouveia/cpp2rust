#include <algorithm>
#include <locale>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, bool T2> bool f12() {
  return std::moneypunct_byname<T1, T2>::intl;
}
