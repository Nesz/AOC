#include <iostream>
#include <algorithm>
#include <iterator>
#include <fstream>
#include <vector>

unsigned int count(std::vector<std::string>& input, int x, int y) {
    unsigned int counter = 0, x_off = 0, y_off = 0;
    while (y_off < input.size()) {

        if (input[y_off][x_off] == '#')
            ++counter;

        y_off += y;
        x_off = (x_off + x) % input[0].size();
    }

    return counter;
}

int main(int argc, char const *argv[]) {
    std::vector<std::string> input;
    std::fstream file(argv[1], std::ios_base::in);
    std::copy(std::istream_iterator<std::string>(file),
              std::istream_iterator<std::string>(),
              std::back_inserter(input));

    std::cout << count(input, 3, 1) << "\n";

    std::vector<int> x = {1, 3, 5, 7, 1};
    std::vector<int> y = {1, 1, 1, 1, 2};

    long long int counter = 1;
    for (int i = 0; i < x.size(); ++i)
        counter *= count(input, x[i], y[i]);

    std::cout << counter << "\n";

    return 0;
}