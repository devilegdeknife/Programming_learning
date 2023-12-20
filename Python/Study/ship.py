import pygame


class Ship:
    def __init__(self, screen):
        # 初始化飞船
        self.screen = screen

        # 加载飞船并获取外接矩形
        self.image = pygame.image.load("images/ship.bmp")
        self.rect = self.image.get_rect()
        self.screen_rect = screen.get_rect()

        # 设置飞船位置，置于底部中央
        self.rect.centerx = self.screen_rect.centerx
        self.rect.bottom = self.screen_rect.bottom

        # 移动标志
        self.moving_right = False
        self.moving_left = False

    def update(self):
        # 右移位置条件判断
        if self.moving_right:
            self.rect.x += 1
        if self.moving_left:
            self.rect.x -= 1

    def blitme(self):
        # 指定位置绘制飞船
        self.screen.blit(self.image, self.rect)
