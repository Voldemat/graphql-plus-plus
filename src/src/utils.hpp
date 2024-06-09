#pragma once

#include <cstddef>
#include <string>
#include <vector>
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

std::vector<std::string> split(std::string s, std::string delimiter);
