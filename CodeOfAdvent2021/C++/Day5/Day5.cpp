#include <cassert>
#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

struct Line
{
    int x1 = 0, y1 = 0;
    int x2 = 0, y2 = 0;
};

int main()
{
    std::ifstream file(PROJECT_DIR "Day5.txt");
    if(!file.is_open())
        return 1;

    const int width = 1000;
    const int height = 1000;
    std::vector<int> map(width * height, 0);
    std::vector<int> map_diagonal(width * height, 0);

    while(file.good())
    {
        Line line;

        char comma;
        file >> line.x1 >> comma >> line.y1;
        file.ignore(4);
        file >> line.x2 >> comma >> line.y2;

        if(file.good())
        {
            int offset_x = line.x2 - line.x1;
            int offset_y = line.y2 - line.y1;

            if(line.x1 == line.x2 || line.y1 == line.y2)
            {

                for(int x = 0; x < std::abs(offset_x); ++x)
                {
                    map[line.y1 * width + (line.x1 + x * (offset_x >= 0 ? 1 : -1))] += 1;
                    map_diagonal[line.y1 * width + (line.x1 + x * (offset_x >= 0 ? 1 : -1))] += 1;
                }

                for(int y = 0; y < std::abs(offset_y); ++y)
                {
                    map[(line.y1 + y * (offset_y >= 0 ? 1 : -1)) * width + line.x1] += 1;
                    map_diagonal[(line.y1 + y * (offset_y >= 0 ? 1 : -1)) * width + line.x1] += 1;
                }

                map[line.y2 * width + line.x2] += 1;
                map_diagonal[line.y2 * width + line.x2] += 1;
            }
            else if(std::abs(offset_x) == std::abs(offset_y))
            {
                for(int i = 0; i <= std::abs(offset_x); ++i)
                    map_diagonal[(line.y1 + i * (offset_y >= 0 ? 1 : -1)) * width + (line.x1 + i * (offset_x >= 0 ? 1 : -1))] += 1;
            }
        }
    }

    int overlaps = 0, overlaps_diagonal = 0;
    for(int i = 0; i < width * height; ++i)
    {
        if(map[i] >= 2)
            overlaps += 1;

        if(map_diagonal[i] >= 2)
            overlaps_diagonal += 1;
    }

    cout << "Answer for part 1 is " << overlaps << endl;
    cout << "Answer for part 2 is " << overlaps_diagonal << endl;

    return 0;
}
