### Colour blending test

import pygame
import math

WIDTH, HEIGHT = 500,500

win = pygame.display.set_mode((WIDTH, HEIGHT))

class Colour:
    def __init__(self, r,g,b,a):
        self.r = r
        self.g = g
        self.b = b
        self.a = a
    def blend_with(self, other):
        return Colour(
                (self.r * self.a / 255.0 + other.r)/2.0,
                (self.g * self.a / 255.0 + other.g)/2.0,
                (self.b * self.a / 255.0 + other.b)/2.0,
                max(self.a, other.a)
            )
    def colour(self):
        return (self.r, self.g, self.b)

def main():

    black = Colour(0,0,0,255)
    top = Colour(255,0,0,255)
    bottom = Colour(0,255,0,255)

    delta = 0
    
    run = True
    while run:
        win.fill(black.colour())

        delta += 1
        bottom = Colour(0,200,100,255)
        top = Colour(220,100,30,255)

        # print(bottom.blend_with(top).colour())

        pygame.draw.rect(win, bottom.blend_with(black).colour(), (0,100,400,300))
        pygame.draw.rect(win, top.blend_with(black).colour(), (100,100+50,800,300))
        pygame.draw.rect(win, bottom.blend_with(top).colour(), (100,100+50,300,300-50))

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                run = False

        pygame.display.update()

    pygame.quit()

if __name__=="__main__":
    main()


