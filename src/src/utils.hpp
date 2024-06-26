#pragma once

#include <cstddef>
#include <functional>
#include <string>
#include <vector>
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

std::vector<std::string> split(std::string s, std::string delimiter);

template<typename T>
struct memfun_type
{
        using type = void;
};

template<typename Ret, typename Class, typename... Args>
struct memfun_type<Ret(Class::*)(Args...) const>
{
        using type = std::function<Ret(Args...)>;
};

template<typename F>
typename memfun_type<decltype(&F::operator())>::type
FFL(F const &func)
{ // Function from lambda !
    return func;
};
