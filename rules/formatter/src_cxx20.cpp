#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <format>
#include <string>
#include <string_view>
#include <utility>

template <typename T1> using t1 = std::formatter<std::chrono::day, T1>;

template <typename T1> using t2 = std::formatter<std::chrono::month, T1>;

template <typename T1> using t3 = std::formatter<std::chrono::month_day, T1>;

template <typename T1>
using t4 = std::formatter<std::chrono::month_day_last, T1>;

template <typename T1>
using t5 = std::formatter<std::chrono::month_weekday, T1>;

template <typename T1>
using t6 = std::formatter<std::chrono::month_weekday_last, T1>;

template <typename T1> using t7 = std::formatter<std::chrono::weekday, T1>;

template <typename T1>
using t8 = std::formatter<std::chrono::weekday_indexed, T1>;

template <typename T1> using t9 = std::formatter<std::chrono::weekday_last, T1>;

template <typename T1> using t10 = std::formatter<std::chrono::year, T1>;

template <typename T1> using t11 = std::formatter<std::chrono::year_month, T1>;

template <typename T1>
using t12 = std::formatter<std::chrono::year_month_day, T1>;

template <typename T1>
using t13 = std::formatter<std::chrono::year_month_day_last, T1>;

template <typename T1>
using t14 = std::formatter<std::chrono::year_month_weekday, T1>;

template <typename T1>
using t15 = std::formatter<std::chrono::year_month_weekday_last, T1>;

template <typename T1> using t16 = std::formatter<const T1 *, T1>;

template <typename T1> using t17 = std::formatter<T1 *, T1>;

template <typename T1, typename T2>
using t18 = std::formatter<std::chrono::sys_time<T1>, T2>;

template <typename T1, typename T2>
using t19 = std::formatter<std::chrono::local_time<T1>, T2>;

template <typename T1, typename T2> using t20 = std::formatter<T1, T2>;

template <typename T1, typename T2, typename T3>
using t21 = std::formatter<std::chrono::duration<T1, T2>, T3>;
