import vid_reader
import numpy

b_a_m = [' ', '.', ',', '-', '~', ':', ';', '=', '!', '*', '?', '$', '@', '#'] # brightness ascii mapping

def assign_ascii_to_brightness(rbg):
    w_b_a = (0.2126 * rbg[0]) + (0.0722 * rbg[1]) + (0.7152 * rbg[2]) # weighted_brightness_av
    char_range = int(255 / len(char_range))
    return b_a_m[min(int(w_b_a) / char_range), len(b_a_m) - 1]
