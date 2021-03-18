#include <iostream>
#include <algorithm>
#include <fstream>
#include <string>

int count_if_everyone_answered_yes(std::string& uniq, std::string& cpy, int size) {
    return std::count_if(uniq.begin(), uniq.end(), [&](const unsigned char c) -> bool {
        return std::count(cpy.begin(), cpy.end(), c) == size;
    });
}

int main(int argc, char const *argv[]) {
    std::fstream file(argv[1], std::ios_base::in);

    int valid1 = 0, valid2 = 0, group = 0;
    std::string entries, line;
    while (std::getline(file, line)) {

        if (!line.empty()) {
            entries += line;
            ++group;
        }

        if (line.empty() || file.eof()) {
            std::sort(entries.begin(), entries.end());
            std::string cpy = entries;
            entries.erase(std::unique(entries.begin(), entries.end()), entries.end());

            valid1 += entries.length();
            valid2 += count_if_everyone_answered_yes(entries, cpy, group);

            group = 0;
            entries.clear();
        }

    }

    std::cout << valid1 << "\n";
    std::cout << valid2 << "\n";


    return 0;
}