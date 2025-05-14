#include <bits/stdc++.h>
using namespace std;

constexpr int N = 1000;

array<bitset<N>, N> grid_bits;
array<array<int, N>, N> brightness;

int main(int argc, char* argv[]) {

    if (argc != 2) {
        cerr << "Usage: " << argv[0] << " <input_file>\n";
        return 1;
    }

    ifstream infile(argv[1]);
    if (!infile) {
        cerr << "Error opening file: " << argv[1] << "\n";
        return 1;
    }

    string w1, w2, through;
    string line;
    while (getline(infile, line)) {
        istringstream iss(line);
        int op = -1; // 0=on, 1=off, 2=toggle
        iss >> w1;
        if (w1 == "turn") {
            iss >> w2;
            op = (w2 == "on") ? 0 : 1;  // "on" -> 0, "off" -> 1
        } else if (w1 == "toggle") {
            op = 2;
        }
        
        int x0, y0, x1, y1;
        char comma;
        iss >> x0 >> comma >> y0;
        iss >> through; // "through"
        iss >> x1 >> comma >> y1;

        for (int y = y0; y <= y1; ++y) {
            auto& bits = grid_bits[y];
            auto& br = brightness[y];
            for (int x = x0; x <= x1; ++x) {
                switch (op) {
                    case 0: // turn on
                        bits.set(x);
                        ++br[x];
                        break;
                    case 1: // turn off
                        bits.reset(x);
                        if (br[x] > 0) --br[x];
                        break;
                    case 2: // toggle
                        bits.flip(x);
                        br[x] += 2;
                        break;
                }
            }
        }
    }

    // Part 1: count lights on
    long long count_on = 0;
    for (int y = 0; y < N; ++y) {
        count_on += grid_bits[y].count();
    }

    // Part 2: total brightness
    long long total_brightness = 0;
    for (int y = 0; y < N; ++y) {
        for (int x = 0; x < N; ++x) {
            total_brightness += brightness[y][x];
        }
    }

    cout << "Part 1: " << count_on << "\n";
    cout << "Part 2: " << total_brightness << "\n";

    return 0;
}

/*
To compile (with GCC or Clang):

    g++ -std=c++17 -O2 -march=native lights.cpp -o lights

Then run:

    ./lights input.txt
*/
