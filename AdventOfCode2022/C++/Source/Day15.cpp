#include <vector>
#include <unordered_set>
#include <optional>
#include <fstream>
#include <scn/scn.h>
#include <fmt/core.h>
#include <glm/glm.hpp>
#include <glm/gtx/hash.hpp>

int main()
{
    int widthMin = INT32_MAX, widthMax = INT32_MIN;
    std::vector<glm::ivec2> sensors;
    std::vector<int> sensorRanges;
    std::unordered_set<glm::ivec2> beacons;

    auto calculateDistance = [](const glm::ivec2& a, const glm::ivec2& b)
    {
        return std::abs(a.x - b.x) + std::abs(a.y - b.y);
    };

    std::ifstream input("../../Input/Day15.txt");
    for(std::string line; std::getline(input, line);)
    {
        glm::ivec2 beacon;
        glm::ivec2& sensor = sensors.emplace_back();
        scn::scan(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                  sensor.x, sensor.y, beacon.x, beacon.y);
        beacons.insert(beacon);

        int& sensorRange = sensorRanges.emplace_back();
        sensorRange = calculateDistance(sensor, beacon);
        widthMin = std::min(widthMin, sensor.x - sensorRange);
        widthMax = std::max(widthMax, sensor.x + sensorRange);
    }

    auto hasSensorCoverage = [&](const glm::ivec2& point)
    {
        for(int i = 0; i < sensors.size(); ++i)
        {
            if(calculateDistance(point, sensors[i]) <= sensorRanges[i])
                return true;
        }
        return false;
    };

    int part1 = 0;
    for(int x = widthMin; x <= widthMax; ++x)
    {
        glm::ivec2 point(x, 2000000);
        if(hasSensorCoverage(point) && !beacons.contains(point))
            part1 += 1;
    }

    std::optional<glm::ivec2> part2;
    for(int i = 0; !part2.has_value() && i < sensors.size(); ++i)
    {
        int pastSensorRange = sensorRanges[i] + 1;
        for(int j = 0; j < pastSensorRange * 4; ++j)
        {
            glm::ivec2 sensorEdge(sensors[i]);
            if(j < pastSensorRange)
            {
                sensorEdge.x += pastSensorRange - j;
                sensorEdge.y += j;
            }
            else if(j < pastSensorRange * 2)
            {
                sensorEdge.x += pastSensorRange - j;
                sensorEdge.y += pastSensorRange - j % pastSensorRange;
            }
            else if(j < pastSensorRange * 3)
            {
                sensorEdge.x += -pastSensorRange + j % pastSensorRange;
                sensorEdge.y += -j % pastSensorRange;
            }
            else
            {
                sensorEdge.x += j % pastSensorRange;
                sensorEdge.y += -pastSensorRange + j % pastSensorRange;
            }

            if(sensorEdge.x >= 0 && sensorEdge.x <= 4000000)
            if(sensorEdge.y >= 0 && sensorEdge.y <= 4000000)
            if(!hasSensorCoverage(sensorEdge))
            {
                part2 = sensorEdge;
                break;
            }
        }
    }

    fmt::print("Day 15-1 answer: {}\n", part1);
    fmt::print("Day 15-2 answer: {}\n", part2.value().x * 4000000ll + part2.value().y);
}
