import curses
from curses import wrapper

def main(stdscr):
    stdscr.clear()
    stdscr.addstr(10, 10, "wubwub test i want a shark onesie c:")
    stdscr.refresh()
    stdscr.getch()

wrapper(main)