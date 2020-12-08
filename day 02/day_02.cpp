#include <iostream>
#include <algorithm>
#include <fstream>

bool test1(int min, int max, char lf, std::string& passwd) {
    int occ = std::count(passwd.begin(), passwd.end(), lf);
    return occ <= max && occ >= min;
}

bool test2(int min, int max, char lf, std::string& passwd) {
    return passwd[min-1] == lf && passwd[max-1] != lf || passwd[min-1] != lf && passwd[max-1] == lf;
}

int main(int argc, char const *argv[]) {
    std::fstream file(argv[1], std::ios_base::in);

    char sep, lf;
    int min, max;
    std::string passwd;

    int valid1 = 0;
    int valid2 = 0;
    while (file >> min >> sep >> max >> lf >> sep >> passwd) {
        if (test1(min, max, lf, passwd))
            ++valid1;
        if (test2(min, max, lf, passwd))
            ++valid2;

    }

    std::cout << valid1 << "\n";
    std::cout << valid2 << "\n";

    return 0;
}