//
// Created by 86134 on 2024-02-01.
//
# include <iostream>

int longolb(int);

int main()
{
    using namespace std;
    int along;
    cout << "Enter the weight in long: ";
    cin >> along;
    int pounds = longolb(along);
    cout << along << " long = " << pounds << " pounds." << endl;
    return 0;
}

int longolb(int l)
{
    return 220 * l;
}

