import sys
import os
import math as m
import json

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

def encode_maze(maze_filepath: str, maze_size: int) -> str:
    file = None
    try:
        file = open(maze_filepath, "r")
    except:
        eprint("Failed to open file")
        exit()

    lines = file.readlines()
    size = len(lines)
    for i in range(size):
        lines[i] = lines[i].strip()


    # row = floor( 2 * index / size )
    # col = (index % (size / 2)) * 2 + (1 - size % 2) * ( row % 2 )

    encoding = "0x"
    for i in range(int((maze_size**2 - m.floor(maze_size**2 / 2)))):
        true_r = int(m.floor( 2 * i / maze_size ))
        true_c = int(( i % ( maze_size / 2 ) ) * 2 + (1 - maze_size % 2) * ( true_r % 2 ))

        r = 2 * maze_size - 1 - 2 * true_r
        c = 2 * true_c + 1

        cell_encoding = 0
        if (lines[r][c] != ' '): # If invalid encoding then just skip
            return "0x0"
        if lines[r - 1][c] == '-': # NORTH
            cell_encoding = cell_encoding | 0b0001
        if lines[r + 1][c] == '-': # SOUTH 
            cell_encoding = cell_encoding | 0b0100
        if lines[r][c + 1] == '|': # EAST 
            cell_encoding = cell_encoding | 0b0010
        if lines[r][c - 1] == '|': # WEST 
            cell_encoding = cell_encoding | 0b1000

        encoding += hex(cell_encoding)[2]

    file.close()
    return encoding

def save_mazes(maze_dir):
    mazesize = 16
    dir = os.fsencode(maze_dir)
    mazes = []

    for file in os.listdir(dir):
        filename = os.fsdecode(file)
        filepath = dir.decode() + filename
        encoding = encode_maze(filepath, 16)

        mazes.append({
            "name": filename,
            "size": mazesize,
            "encoding": encoding
        })

    saved_mazes = open("resources/previous_mazes.json", "w")
    saved_mazes.write(json.dumps(mazes))

def main():
    if len(sys.argv) < 2:
        eprint("Invalid arguments provided - Please specify the file to try convert and its grid size")
        exit()

    all_mazes = sys.argv[1]
    save_mazes(all_mazes)


if __name__ == "__main__":
    main()