import sys
import math as m

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

def main():
    if len(sys.argv) < 3:
        eprint("Invalid arguments provided - Please specify the file to try convert and its grid size")
        exit()

    filepath = sys.argv[1]
    grid_size = int(sys.argv[2])
    file = None
    try:
        file = open(filepath, "r")
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
    for i in range(int((grid_size**2 - m.floor(grid_size**2 / 2)))):
        true_r = int(m.floor( 2 * i / grid_size ))
        true_c = int(( i % ( grid_size / 2 ) ) * 2 + (1 - grid_size % 2) * ( true_r % 2 ))

        r = 2 * grid_size - 1 - 2 * true_r
        c = 2 * true_c + 1

        cell_encoding = 0
        assert(lines[r][c] == ' ') # check its a valid cell being read
        if lines[r - 1][c] == '-': # NORTH
            cell_encoding = cell_encoding | 0b0001
        if lines[r + 1][c] == '-': # SOUTH 
            cell_encoding = cell_encoding | 0b0100
        if lines[r][c + 1] == '|': # EAST 
            cell_encoding = cell_encoding | 0b0010
        if lines[r][c - 1] == '|': # WEST 
            cell_encoding = cell_encoding | 0b1000

        encoding += hex(cell_encoding)[2]

    print(encoding)
    file.close()

if __name__ == "__main__":
    main()