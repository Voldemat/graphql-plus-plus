#pragma once

#include <algorithm>
#include <cstddef>
#include <functional>
#include <iterator>
#include <map>
#include <string>
#include <utility>
#include <vector>

namespace gql::utils {
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

std::vector<std::string> split(std::string s, std::string delimiter);

template <typename T>
struct memfun_type {
    using type = void;
};

template <typename Ret, typename Class, typename... Args>
struct memfun_type<Ret (Class::*)(Args...) const> {
    using type = std::function<Ret(Args...)>;
};

template <typename F>
typename memfun_type<decltype(&F::operator())>::type FuncFromLambda(
    F const &func) {  // Function from lambda !
    return func;
};

template <typename A, typename B>
constexpr std::pair<B, A> flip_pair(const std::pair<A, B> &p) {
    return std::pair<B, A>(p.second, p.first);
}

template <typename A, typename B>
constexpr std::map<B, A> flip_map(const std::map<A, B> &src) {
    std::map<B, A> dst;
    std::transform(src.begin(), src.end(), std::inserter(dst, dst.begin()),
                   flip_pair<A, B>);
    return dst;
}
};  // namespace gql
