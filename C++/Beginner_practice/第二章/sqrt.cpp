//
// Created by 86134 on 2024-02-01.
//

# include <iostream>
# include <cmath>

int main()
{
    using namespace std;

    double area;
    cout << "Enter the floor area, in square feet, of your room: ";
    cin >> area;
    double side;
    side = sqrt(area);
    cout << "That's the equivalent of square " << side
         << " feet to the side. " << endl;
    cout << "How fascinating!" << endl;
    return 0;
}