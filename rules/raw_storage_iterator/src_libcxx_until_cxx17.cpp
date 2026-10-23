#include <memory>
#include <utility>

template <typename T1, typename T2>
using t2 = typename std::raw_storage_iterator<T1, T2>::iterator_category;

template <typename T1, typename T2>
using t3 = typename std::raw_storage_iterator<T1, T2>::value_type;

template <typename T1, typename T2>
using t4 = typename std::raw_storage_iterator<T1, T2>::difference_type;

template <typename T1, typename T2>
using t5 = typename std::raw_storage_iterator<T1, T2>::pointer;

template <typename T1, typename T2>
using t6 = typename std::raw_storage_iterator<T1, T2>::reference;
