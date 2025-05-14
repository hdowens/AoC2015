#include <algorithm>   
#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <vector>

using std::cout;
using std::cerr;
using std::endl;

int count_vowels(const std::string& line) {
    static constexpr std::string_view VOWELS{"aeiou"};
    return std::count_if(
        line.begin(), 
        line.end(),
        [](char c){ 
            return VOWELS.find(c) != std::string_view::npos; 
        }
    );
}

bool has_bad(const std::string& line) {
    static constexpr std::array<std::string_view, 4> BAD{"ab","cd","pq","xy"};
    return std::any_of(BAD.begin(), BAD.end(),
                       [&](auto pat){ return line.find(pat) != std::string::npos; });
}

bool has_double_letter(const std::string& line) {
    return std::adjacent_find(line.begin(), line.end()) != line.end();
}

bool has_repeated_pair(const std::string& s) {
    // For each 2-char window at i:
    for (size_t i = 0; i + 1 < s.size(); ++i) {
        std::string_view pair(&s[i], 2);
        if (s.find(pair, i + 2) != std::string::npos) {return true;}
    }
    return false;
}
bool has_three_letter_substr(const std::string& line){
    std::string_view sv{line};
    for(size_t i = 0; i < sv.size() - 2; i++) {
        if (sv[i] == sv[i+2]) {return true;}
    }
    return false;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        cerr << "Usage: " << argv[0] << " <input_file>\n";
        return 1;
    }

    std::ifstream file{argv[1]};
    if (!file.is_open()) {
        cerr << "Failed to open file: " << argv[1] << "\n";
        return 1;
    }

    std::string line;
    size_t part1_count = 0; 
    size_t part2_count = 0; 
    while (std::getline(file, line)) {
        //part 1
        int vowel_count = count_vowels(line);
        bool bad_found = has_bad(line);
        bool repeat_found = has_double_letter(line);
        if(vowel_count > 2 && !bad_found && repeat_found) {
            part1_count++;
        }
        
        //part 2
        if (has_three_letter_substr(line) && has_repeated_pair(line)) {
            part2_count++;
        }

    }
    cout << "part 1: " << part1_count << "\n";
    cout << "part 2: " << part2_count << "\n";
    return 0;
}
