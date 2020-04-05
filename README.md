# compare
Compare sets and print result to console

This will read a text-file and split it using a delimiter. It will either show
[i] intersection (in all sets) or [s] symmetric difference (only in one set).

A text-file in CSV-format is what I had in mind when I began coding it but can be
used for any text-file. If the delimiter is found and the column is within bounds
it can do some work.

Example where delimiter is semicolon (;), column is two (2) and operation is
first intersection (i) and secondly symmetric difference (s).

$ cat a.csv  
1;a;one  
2;b;two  
3;c;three  
4;d;four  
5;e;five  

$ cat b.csv   
3;c;three  
4;d;four  
5;e;five  
6;f;six  
7;g;seven  

$ compare -f a.csv -f b.csv -d ';' -c 2 -o i  
five  
four  
three  

$ compare -f a.csv -f b.csv -d ';' -c 2 -o s  
one  
six  
seven  
two  


Can use three or more files for comparison.

cat c.csv  
5;e;five  
6;f;six  
7;g;seven  
8;h;eight  
9;i;nine  

$ compare -f a.csv -f b.csv -f c.csv -d ';' -c 2 -o i  
five  

To build it use 'cargo build' or 'cargo build --release'.
