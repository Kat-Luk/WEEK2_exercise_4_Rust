Task 4

In this task you make more extensive use of the functions of the external module you created, according to your needs.

Make a program where the user can move x in a character table by giving input. X cannot slip outside of the array.

Put all the map functions and the x shift functions in a folder called "map" inside the src folder. Put the (public) functions in the arraymap.rs file. In mod.rs file make arraymap public.
Make a function that makes a array-like structure to the console using print functions to print 5 rows of stars, a 5 x 5 grid. Print an x ​​in the center of the grid instead of an asterisk (*).

Make a loop that moves x based on user input. Enter w) to move x up the grid, a) move it left, s) move it down, d) move it right. If x can't move out of the map, don't move x and print "Can't move out of the map" in addition to printing the current grid. e) terminates the program.

Use a function called create_map() to create a default map that is a 2-dimensional char-array.

In the main file, call the function create_map() to create the default map. Print the map.

Then loop as above, asking the user to input w, a s, or d to move x, or e to exit the program. Call the appropriate functions in the arraymap.rs file as needed. Use references when passing an arraymap variable to functions that are mutable or immutable as needed.

Example run:

* * * * *
* * * * *
* * x * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
* * * * *
* * x * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
w
Can't move out of the map
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
* x * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
x * * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
Can't move out of the map
x * * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* x * * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * x * *
* * * * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * x * *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * * * *
* * * x *
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
* * * * *
* * * * x
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
d
Can't move out of the map
* * * * *
* * * * x
* * * * *
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * x
* * * * *
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * *
* * * * x
* * * * *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
* * * * *
* * * * *
* * * * *
* * * * *
* * * * x
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
s
Can't move out of the map
* * * * *
* * * * *
* * * * *
* * * * *
* * * * x
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
a
* * * * *
* * * * *
* * * * *
* * * * *
* * * x *
| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |
e
Ending the program.
