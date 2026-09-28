/*
 * Copyright © 2013 Bart Massey
 * [This program is licensed under the "MIT License"]
 * Please see the file COPYING in the source
 * distribution of this software for license terms.
 */

/* Java player for the simple game "Chomp"
   (http://en.wikipedia.org/wiki/Chomp) */

import java.io.*;

class Move {
    int r, c;

    Move(int r, int c) {
        this.r = r;
        this.c = c;
    }
}

public class Chomp {
    /* http://www.abbeyworkshop.com/howto/java/readLine */
    static InputStreamReader converter = new InputStreamReader(System.in);
    static BufferedReader in = new BufferedReader(converter);

    static final int dimx = 4, dimy = 3;
    static boolean[][] board;

    static void initBoard() {
        board = new boolean[dimy][dimx];
        for (int r = 0; r < dimy; r++)
            for (int c = 0; c < dimx; c++)
                board[r][c] = true;
    }

    static void makeMove(boolean[][] b, int r0, int c0) {
        for (int r = r0; r < dimy; r++)
            for (int c = c0; c < dimx; c++)
                b[r][c] = false;
    }

    static void printBoard(boolean b[][]) {
        for (int r = 0; r < dimy; r++) {
            for (int c = 0; c < dimx; c++) {
                if (b[r][c]) {
                    if (r == 0 && c == 0)
                        System.out.print("*");
                    else
                        System.out.print("o");
                } else {
                    System.out.print(".");
                }
            }
            System.out.println();
        }
    }

    static Move getMove() throws IOException {
        printBoard(board);
        System.out.print("? ");
        String move = in.readLine();
        System.out.println();
        int moveDigits = Integer.parseInt(move);
        int r = moveDigits / 10 - 1;
        int c = moveDigits % 10 - 1;
        return new Move(r, c);
    }

    static boolean[][] copyBoard(boolean[][] b) {
        boolean[][] cb = new boolean[dimy][dimx];
        for (int r = 0; r < dimy; r++)
            for (int c = 0; c < dimx; c++)
                cb[r][c] = b[r][c];
        return cb;
    }

    static boolean negamax(boolean[][] b, int d) {
        int rx = 0;
        int cx = 0;
        for (int r = 0; r < dimy; r++) {
            for (int c = 0; c < dimx; c++) {
                if (b[r][c]) {
                    if (r == 0 && c == 0)
                        continue;
                    rx = r;
                    cx = c;
                    boolean[][] mb = copyBoard(b);
                    makeMove(mb, r, c);
                    if (negamax(mb, d + 1))
                        continue;
                    if (d == 0) {
                        System.out.println("> " + (10 * (r + 1) + (c + 1)) +
                                           " :-)");
                        makeMove(b, r, c);
                    }
                    return true;
                }
            }
        }
        if (d == 0) {
            System.out.print("> " + (10 * (rx + 1) + (cx + 1)));
            if (rx == 0 && cx == 0)
                System.out.println(" :-(");
            else
                System.out.println(" :-P");
            makeMove(b, rx, cx);
            System.out.println();
        }
        return false;
    }

    static void computeMove() {
        printBoard(board);
        negamax(board, 0);
    }

    public static void main(String args[]) throws IOException {
        initBoard();
        while (true) {
            Move m = getMove();
            if (!board[m.r][m.c]) {
                System.out.println("Illegal Move");
                continue;
            }
            if (m.r == 0 && m.c == 0) {
                System.out.println("I win");
                break;
            }
            makeMove(board, m.r, m.c);
            computeMove();
            if (!board[0][0]) {
                System.out.println("You win");
                break;
            }
        }
    }
}
