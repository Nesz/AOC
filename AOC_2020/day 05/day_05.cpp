#include <iostream>
#include <algorithm>
#include <iterator>
#include <fstream>
#include <vector>
#include <sstream>

int to_dec(const std::string& line) {
    std::string bin;
    for (char i : line)
        bin += (i == 'F' || i  == 'L') ? '0' : '1';

    return std::stoi(bin, nullptr, 2);
}

int main(int argc, char const *argv[]) {
    std::fstream file(argv[1], std::ios_base::in);
    std::vector<int> indexes;
    std::transform(std::istream_iterator<std::string>(file),
                   std::istream_iterator<std::string>(),
                   std::back_inserter(indexes),
                   &to_dec);

    std::sort(indexes.begin(), indexes.end());
    std::cout << indexes[indexes.size() - 1] << "\n";

    for (int i = 0; i < indexes.size() - 1; ++i) {
        if (indexes[i + 1] - indexes[i] > 1) {
            std::cout << indexes[i] + 1 << "\n";
            break;
        }
    }

    return 0;
}