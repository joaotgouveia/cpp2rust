#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::pmr::basic_string<T1, T2>;

using t2 = std::pmr::string;

using t3 = std::pmr::u16string;

using t4 = std::pmr::u32string;

using t5 = std::pmr::wstring;
