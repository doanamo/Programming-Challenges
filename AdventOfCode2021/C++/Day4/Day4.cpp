#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

struct Board
{
     static const int Width = 5;
     static const int Height = 5;
     static const int Fields = Width * Height;

     struct Field
     {
         int number = 0;
         bool hit = false;
     };

     Field fields[Fields];

     bool Hit(int number)
     {
         for(int i = 0; i < Fields; ++i)
         {
             Field& field = fields[i];
             if(field.number == number)
             {
                 field.hit = true;
                 return true;
             }
         }

         return false;
     }

     bool CheckBingo() const
     {
         for(int x = 0; x < Width; ++x)
         {
             bool bingo = true;

             for(int y = 0; y < Height; ++y)
             {
                 bingo &= fields[x + y * Height].hit;
             }

             if(bingo)
             {
                 return true;
             }
         }

         for(int y = 0; y < Height; ++y)
         {
             bool bingo = true;

             for(int x = 0; x < Width; ++x)
             {
                 bingo &= fields[x + y * Height].hit;
             }

             if(bingo)
             {
                 return true;
             }
         }

         return false;
     }

     int GetScore() const
     {
         int sum = 0;
         for(const Field& field : fields)
         {
             if(!field.hit)
             {
                sum += field.number;
             }
         }

         return sum;
     }
};

int main()
{
    std::ifstream file(PROJECT_DIR "Day4.txt");
    if(!file.is_open())
        return 1;

    std::vector<int> numbers;
    while(file.peek() != '\n')
    {
        if(file.peek() == ',')
        {
            file.ignore();
            continue;
        }

        int number;
        file >> number;
        numbers.push_back(number);
    }

    cout << "Read " << numbers.size() << " numbers" << endl;

    std::vector<Board> boards;
    while(file.good())
    {
        Board board;
        for(int i = 0; i < Board::Fields; ++i)
        {
            file >> board.fields[i].number;
        }

        if(file.good())
        {
            boards.push_back(board);
        }
    }

    cout << "Read " << boards.size() << " boards" << endl;

    bool foundFirstWinningBoard = false;

    for(int number : numbers)
    {
        std::vector<Board*> winningBoards;

        for(Board& board : boards)
        {
            if(board.Hit(number) && board.CheckBingo())
            {
                if(!foundFirstWinningBoard)
                {
                    cout << "Answer for part 1 is " << number * board.GetScore() << endl;
                    foundFirstWinningBoard = true;
                }

                if(boards.size() == 1)
                {
                    cout << "Answer for part 2 is " << number * board.GetScore() << endl;
                    return 0;
                }

                winningBoards.push_back(&board);
            }
        }

        auto removeWinning = std::remove_if(boards.begin(), boards.end(),
            [&winningBoards](const Board& board)
            {
                return std::find(winningBoards.begin(), winningBoards.end(), &board) != winningBoards.end();
            });

        boards.erase(removeWinning, boards.end());
    }

    return 1;
}
