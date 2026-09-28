Haskell Chomp
Copyright © 2013 Bart Massey

[Chomp](http://en.wikipedia.org/wiki/Chomp) is a simple
mathematical two-player game perhaps best described as "2D
[Nim](http://en.wikipedia.org/wiki/Nim)". This Haskell
program plays perfect Chomp against a human using adversary
search. It's currently set up to play 4x3 Chomp with the
human moving first.

To build this code, you need the Haskell compiler GHC. The
"cabal" tool is also a useful thing to have around: if you
have it, you can just say "cabal install --user". Otherwise
you should be able to say "runghc Setup.hs configure --user;
runghc Setup.hs install". This should install the code such
that you can then just say "chomp" to play. You move first;
Chomp is a first-player win on all rectangular boards bigger
than 2x2.

To play, input your move using tuple syntax: "(2,3)" is a
move chomping row 2, column 3 of the board.

This work is made available under the "MIT License". Please
see the file COPYING in this distribution for license terms.
