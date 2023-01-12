import pygame

SQRT_THREE = 3 ** (1/2)

SIZE = 500
MATRIX_SIZE = 3
HEX_SIZE = SIZE / (MATRIX_SIZE*5)

BLACK = (0,0,0)
DARK = (63,63,63)
GRAY = (127,127,127)
LIGHT = (191,191,191)
WHITE = (255,255,255)

def hex_points(hex_size):
    return [
        (-hex_size*.5, -hex_size*SQRT_THREE*.5),
        (hex_size*.5, -hex_size*SQRT_THREE*.5),
        (hex_size, 0),
        (hex_size*.5, hex_size*SQRT_THREE*.5),
        (-hex_size*.5, hex_size*SQRT_THREE*.5),
        (-hex_size, 0)
    ]

def pixel_to_hex(x, y):
    x, y = x - SIZE/2, y - SIZE/2
    q = (2/3 * x) / HEX_SIZE
    r = (-1/3 * x + SQRT_THREE/3 * y) / HEX_SIZE
    return round(q), round(r) 

def hex_to_pixel(q, r):
    x = HEX_SIZE * (3/2 * q)
    y = HEX_SIZE * (SQRT_THREE/2 * q + SQRT_THREE * r)
    return x + SIZE/2, y + SIZE/2

def get_matrix(size):
    ret = []
    for q in range(-size, size+1):
        for r in range(-size, size+1):
            s = -q - r
            if max([abs(q), abs(r), abs(s)]) <= size:
                ret.append((q, r))
    return ret

def rotate(q, r, amount=1):
    s = -q - r
    if amount == 1:
        return -r, -s
    else:
        return rotate(-r, -s, amount-1)

def rotate_piece(piece_shape, amount):
    ret = set()
    for q, r in piece_shape:
        ret.add(rotate(q, r, amount))
    return ret

def move_piece(piece_shape, matrix, vec):
    ret = set()
    for q, r in piece_shape:
        hexo = q + vec[0], r + vec[1]
        if hexo in matrix:
            ret.add(hexo)
    return ret

pygame.init()
font = pygame.font.SysFont('consolas', int(HEX_SIZE))
screen = pygame.display.set_mode(size=[SIZE, SIZE])

# list of (q, r) axial coords
piece_shape = set([(1, 0), (-1, 1), (-2, 1), (0, 0)])

# list of coords to constraint the defintion of piece coords to
matrix = get_matrix(MATRIX_SIZE)

while True:
    # == input handling ==
    # keyboard
    for event in pygame.event.get():
        match event.type:
            case pygame.QUIT:
                exit()
            case pygame.KEYDOWN:
                match event.key:
                    case pygame.K_ESCAPE:
                        exit()
                    case pygame.K_z:
                        piece_shape = rotate_piece(piece_shape, 5)
                    case pygame.K_c:
                        piece_shape = rotate_piece(piece_shape, 1)
                    case pygame.K_x:
                        piece_shape = rotate_piece(piece_shape, 3)
                    case pygame.K_a:
                        piece_shape = rotate_piece(piece_shape, 4)
                    case pygame.K_d:
                        piece_shape = rotate_piece(piece_shape, 2)
                    case pygame.K_SPACE:
                        print(list(piece_shape))
                    case pygame.K_LEFT:
                        piece_shape = move_piece(piece_shape, matrix, (-1, 0))
                    case pygame.K_RIGHT:
                        piece_shape = move_piece(piece_shape, matrix, (1, 0))
                    case pygame.K_UP:
                        piece_shape = move_piece(piece_shape, matrix, (0, -1))
                    case pygame.K_DOWN:
                        piece_shape = move_piece(piece_shape, matrix, (0, 1))
    # mouse
    mouse_buttons = pygame.mouse.get_pressed()
    mouse_xy = pygame.mouse.get_pos()
    mouse_qr = pixel_to_hex(*mouse_xy)
    if mouse_buttons[0] and mouse_qr in matrix:
        piece_shape.add(mouse_qr)
    
    if mouse_buttons[2] and mouse_qr in piece_shape:
        piece_shape.remove(mouse_qr)

    # == rendering ==
    screen.fill(BLACK)
    # piece shape
    for q, r in piece_shape:
        x, y = hex_to_pixel(q, r)
        points = [(x + _, y + __) for _, __ in hex_points(HEX_SIZE)]
        pygame.draw.polygon(screen, LIGHT, points)
    # matrix
    for q, r in matrix:
        x, y = hex_to_pixel(q, r)
        points = [(x + _, y + __) for _, __ in hex_points(HEX_SIZE)]
        pygame.draw.polygon(screen, DARK, points, 3)
    # highlight
    if mouse_qr in matrix:
        x, y = hex_to_pixel(*mouse_qr)
        points = [(x + _, y + __) for _, __ in hex_points(HEX_SIZE)]
        pygame.draw.polygon(screen, WHITE, points, 3)
    # text
    screen.blit(font.render(str(mouse_qr), True, WHITE if mouse_qr in matrix else DARK), (0, 0))

    pygame.display.flip()
