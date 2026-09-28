# Copyright © 2013 Bart Massey
# [This program is licensed under the "MIT License"]
# Please see the file COPYING in the source
# distribution of this software for license terms.

chomp: chomp.hs
	runghc Setup.hs configure --user
	runghc Setup.hs build
	cp dist/build/chomp/chomp chomp

install: chomp
	rm -f chomp
	runghc Setup.hs install

clean:
	rm -rf dist chomp
