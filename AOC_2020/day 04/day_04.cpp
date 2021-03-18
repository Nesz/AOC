#include <iostream>
#include <algorithm>
#include <fstream>
#include <vector>
#include <string>
#include <sstream>

typedef struct {
    std::string field;
    std::string value;
} entry;

bool has_all(std::vector<entry>& entries) {
    std::vector<std::string> fields_req = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    for (auto& field_req : fields_req) {
        if (!std::any_of(entries.begin(), entries.end(), [&](const entry&e) -> bool {
            return e.field == field_req;
        })) {
            return false;
        }
    }
    return true;
}


bool chck_range(std::string& fie, int min, int max) {
    int tmp = std::stoi(fie);
    return tmp >= min && tmp <= max;
}

bool chck_hgt(std::string &hgt) {
    int indexCM = hgt.find("cm");
    if (indexCM != std::string::npos) {
        int tmp = std::stoi(hgt.substr(0, indexCM));
        return tmp >= 150 && tmp <= 193;
    }

    int indexIN = hgt.find("in");
    if (indexIN != std::string::npos) {
        int tmp = std::stoi(hgt.substr(0, indexIN));
        return tmp >= 59 && tmp <= 76;
    }

    return false;
}

bool chck_hcl(std::string& hcl) {
    if (hcl[0] != '#') {
        return false;
    }
    std::string str = "FF22ABCD16ZZ";
    return std::all_of(hcl.begin() + 1, hcl.end(), ::isxdigit);
}

bool chck_ecl(std::string& ecl) {
    return ecl == "amb" || ecl == "blu" || ecl == "brn" ||
           ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth";
}

bool chck_pid(std::string& pid) {
    return pid.length() == 9;
}

bool is_valid(std::vector<entry>& entries) {
    for (auto& entry : entries) {
        if (entry.field == "byr") {
            if (!chck_range(entry.value, 1920, 2002))
                return false;
        }
        else if (entry.field == "iyr") {
            if (!chck_range(entry.value, 2010, 2020))
                return false;
        }
        else if (entry.field == "eyr") {
            if (!chck_range(entry.value, 2020, 2030))
                return false;
        }
        else if (entry.field == "hgt") {
            if (!chck_hgt(entry.value))
                return false;
        }
        else if (entry.field == "hcl") {
            if (!chck_hcl(entry.value))
                return false;
        }
        else if (entry.field == "ecl") {
            if (!chck_ecl(entry.value))
                return false;
        }
        else if (entry.field == "pid") {
            if (!chck_pid(entry.value))
                return false;
        }
    }
    return true;
}

int main(int argc, char const *argv[]) {
    std::vector<std::string> input;
    std::fstream file(argv[1], std::ios_base::in);


    std::string line, tmp;

    int valid1 = 0;
    int valid2 = 0;
    std::vector<entry> entries;
    while (std::getline(file, line)) {

        std::stringstream ss(line);
        while (std::getline(ss, tmp, ' ')) {
            int index = tmp.find(':');
            std::string field = tmp.substr(0, index);
            std::string value = tmp.substr(index+ 1);
            entries.push_back(entry{field, value});
        }


        if (line.empty() || file.eof()) {
            if (has_all(entries)) {
                ++valid1;
                if (is_valid(entries))
                    ++valid2;
            }
            
            entries.clear();
            continue;
        }

    }
    
    std::cout << valid1 << "\n";
    std::cout << valid2 << "\n";

    return 0;
}