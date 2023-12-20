//
// Created by 86134 on 2024-02-01.
//
#include <iostream>

int stoneolb(int);

int main()
{
    using namespace std;
    int stone;
    cout << "Enter the weight in stone: ";
    cin >> stone;
    int pounds = stoneolb(stone);
    cout << stone << " stone = " << pounds << " pounds." << endl;
    return 0;
}

int stoneolb(int s)
{
    return 14 * s;
}