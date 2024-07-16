#include "./utils.hpp"

#include <iostream>
#include <string>

std::string getAllStdin() noexcept {
    std::string lineInput;
    std::string buffer;
    while (std::getline(std::cin, lineInput)) {
        buffer += lineInput;
        buffer += '\n';
    };
    return buffer;
};
