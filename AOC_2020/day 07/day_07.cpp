#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <map>
#include <regex>

auto parse(std::fstream& file) {
    std::map<std::string, std::map<std::string, int>> bags;
    std::regex reg(R"((\d+) (\w+ \w+) bags?[,.])");

    std::string line;
    while (std::getline(file, line)) {
        std::string bag_name = line.substr(0, line.find(" bags contain"));
        std::map<std::string, int> inner;
        for (auto it = std::sregex_iterator(line.begin(), line.end(), reg); it != std::sregex_iterator(); ++it) {
            std::smatch matches = *it;
            inner[matches[2]] = std::stoi(matches[1]);
        }
        bags[bag_name] = inner;
    }

    return bags;
}

bool canHold(std::map<std::string, std::map<std::string, int>>& bags, const std::string& bag_name) {
    std::map<std::string, int> bag = bags[bag_name];
    if (bag.find("shiny gold") != bag.end())
        return true;

    for (auto& x : bag)
        if (canHold(bags, x.first))
            return true;

    return false;
}

int first(std::map<std::string, std::map<std::string, int>>& bags) {
    int counter = 0;
    for (auto const& x : bags)
        if (canHold(bags, x.first))
            ++counter;

    return counter;
}

int second(std::map<std::string, std::map<std::string, int>>& bags, const std::string& bag_name) {
    int counter = 0;
    for (auto& x : bags[bag_name])
        counter += x.second * second(bags, x.first) + x.second;

    return counter;
}

int main(int argc, char const* argv[]) {
    std::fstream file(argv[1], std::ios_base::in);
    auto bags = parse(file);

    std::cout << first(bags) << "\n";
    std::cout << second(bags, "shiny gold") << "\n";

    return 0;
}