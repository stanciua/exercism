package com.company;

import java.util.ArrayList;
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[][] aa = {{1,2,3}, {4,5,6}};
        Arrays.stream(aa).flatMapToInt(a -> Arrays.stream(a)).forEach(a -> System.out.println(a));
    }
}
