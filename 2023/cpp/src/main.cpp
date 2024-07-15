#include <array>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>

int day1(void) {
  std::ifstream file{"day1.txt"};
  if (!file) {
    std::cerr << "Couldn't open file" << std::endl;
    return 1;
  }
  std::string line{};
  int sum = 0;
  while (std::getline(file, line)) {
    bool isFirst = true;
    int first = 0;
    int last = 0;

    for (auto c : line) {
      if (c >= '0' && c <= '9') {
        last = (c - '0');
        if (isFirst) {
          first = (c - '0');
          isFirst = false;
        }
      }
    }
    sum += first * 10 + last;
  }
  std::cout << sum << std::endl;
  return 0;
}

int day1Pt2(void) {
  std::array<std::string, 10> numStrings{
      "zero", "one", "two",   "three", "four",
      "five", "six", "seven", "eight", "nine",
  };
  std::ifstream file{"day1.txt"};
  if (!file) {
    std::cerr << "Couldn't open file" << std::endl;
    return 1;
  }
  std::string line{};
  int sum = 0;
  while (std::getline(file, line)) {
    bool isFirst = true;
    int first = 0;
    int last = 0;
    std::string str{};

    for (auto c : line) {
      int digit = -1;
      if (c >= '0' && c <= '9') {
        digit = c - '0';
      } else {
        str += c;
        for (size_t i = 0; i < numStrings.size(); i++) {
          if (str.find(numStrings[i]) != std::string::npos) {
            digit = i;
            str.clear();
            str += c; // some numbers share a start and end char.... for some unknown reason
          }
        }
      }

      if (digit != -1) {
        last = digit;
        if (isFirst) {
          first = digit;
          isFirst = false;
        }
      }
    }
    sum += first * 10 + last;
  }
  std::cout << sum << std::endl;

  return 0;
}

int day2(void) {
  return 0;
}

static int (*const days[][2])(void) = {
    {day1, day1Pt2},
    {day2, nullptr},
};

int main(int argc, char *argv[]) {
  int dayToRun = 0;
  int partToRun = 1;
  if (argc > 1) {
    auto arg = atoi(argv[1]);
    if (arg < 1 || (unsigned long)arg > sizeof(days) / sizeof(days[0])) {
      std::cerr << "Invalid argument " << arg << " Must be between 1-"
                << sizeof(days) / sizeof(days[0]) << " " << std::endl;
      exit(1);
    } else {
      dayToRun = arg;
    }
    if (argc > 2) {
      auto arg = atoi(argv[2]);
      if (arg != 1 && arg != 2) {
        std::cerr << "Invalid second argument must be between 1 or 2"
                  << std::endl;
        exit(1);
      } else {
        partToRun = arg;
      }
    }
  }

  if (dayToRun == 0) {
    for (auto d : days) {
      for (int i = 0; i < 2; i++) {
        if (d[i]) {
          auto ret = d[i]();
          if (ret != 0) {
            return ret;
          }
        }
      }
    }
  } else {
    return days[dayToRun - 1][partToRun - 1]();
  }

  return 0;
}
