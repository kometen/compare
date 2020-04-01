# compare
Compare sets and print result to console

This will read a text-file and split it using a delimiter. It will either show
[i] intersection (in all sets) or [s] symmetric difference (only in one set).

A text-file in CSV-format is what I had in mind when I began coding it but can be
used for any text-file. If the delimiter is found and the column is within bounds
it can do some work.

Example where delimiter is semicolon (;), column is three (3) and operation is
intersection (i).

$ compare -f 10.csv -f 11.csv -d ';' -c 3 -o i

To build it use 'cargo build' or 'cargo build --release'.