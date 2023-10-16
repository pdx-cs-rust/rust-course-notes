#!/bin/sh
for f in "$@"
do
    case "$f" in
        *.md) ;;
        *) continue ;;
    esac
    d="Supplementary Lectures"
    case "$f" in
        [01][0-9]-*)
            d="`echo \"$f\" | sed 's=-.*=='`";;
    esac
    d="`ls ../gcsf | egrep \"^${d}\"`"
    b="`basename $f .md`"
    echo pandoc -o "$b.pptx" "$f" &&
    echo mv "$b.pptx" "\"../gcsf/$d/\""
done
