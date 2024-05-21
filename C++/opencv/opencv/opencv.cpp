#include <opencv2/core/core.hpp>
#include <opencv2/imgproc.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <iostream>
using namespace cv;
using namespace std;
int main()
{
	Mat img = imread("C:\\Users\\86134\\Downloads\\1.png");
	namedWindow("测试");
	imshow("测试", img);
	waitKey(0);
	return(0);
}
