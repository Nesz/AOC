#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <iterator>

void part_one(std::vector<int>& input) {
    for (int i = 0; i < input.size(); ++i) {
        for (int j = 0; j < input.size(); ++j) {
            if (input[i] + input[j] == 2020) {
                std::cout << input[i] * input[j] << "\n";
                return;
            }
        }
    }
}

void part_two(std::vector<int>& input) {
    for (int i = 0; i < input.size(); ++i) {
        for (int j = 0; j < input.size(); ++j) {
            for (int k = 0; k < input.size(); ++k) {
                if (input[i] + input[j] + input[k] == 2020) {
                    std::cout << input[i] * input[j] * input[k] << "\n";
                    return;
                }
            }
        }
    }
}

int main(int argc, char const *argv[]) {
    std::vector<int> input;
    std::fstream file(argv[1], std::ios_base::in);
    std::copy(std::istream_iterator<int>(file), 
              std::istream_iterator<int>(), 
              std::back_inserter(input));

    part_one(input);
    part_two(input);

    return 0;
}