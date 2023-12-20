#include <stdio.h>

int main(int argc, char** argv){

    long n = 28000;        //计算到公式的第多少项
    long remainder[28001]; //建立的这个数组，只用来保存每一项每一轮计算之后的余数
    long low_bits = 0;

    //将数组全部初始化为 2000
    //为什么是 2 呢？那是因为原公式计算的是 pi/2，乘以 2 之后正好是 pi
    //至于 2000，则正好把 pi 再放大五、总结 1000 倍，这样打印出来的 3141 前面正好没有 0
    for(int i=0; i<=n; i++){
        remainder[i] = 2000;
    }

    //开始外层循环，每循环一次，级数少计算 14 项，输出 4 位 pi 值
    while(n){
        long denominator = 2*n-1;
        long quotient = 0;
        long sum = 0;

        sum += remainder[n] * 10000;
        remainder[n] = sum % denominator;
        quotient = sum / denominator;

        //开启内层循环，该循环从 b == n-1 开始，循环到 b == 0
        long b = n-1;
        while(b){
            sum = quotient * b + remainder[b] * 10000;

            denominator = 2*b -1;

            //余数保存到数组中，进入下一轮大循环，商进入下一轮小循环
            remainder[b] = sum % denominator;
            quotient = sum / denominator;

            b = b - 1;
        }

        n = n - 14;

        //输出高 4 位，低 4 位进入下一轮循环
        printf("%.4ld", low_bits + sum / 10000);
        low_bits = sum % 10000;
    }

    printf("\n");
    return 0;
}