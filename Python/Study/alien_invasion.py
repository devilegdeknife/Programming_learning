import pygame


from settings import Settings
from ship import Ship
from alien import Alien
import game_functions as gf
from pygame.sprite import Group


def run_game():
    # 初始化游戏并创建屏幕对象
    pygame.init()
    ai_settings = Settings()
    screen = pygame.display.set_mode(
        (ai_settings.screen_width, ai_settings.screen_height))
    pygame.display.set_caption("Alien Invasion")

    # 创建飞船
    ship = Ship(screen)

    bullets = Group()

    alien = Alien(ai_settings, screen)

    # 开始游戏主体
    while True:
        gf.check_events(ai_settings, screen, ship, bullets)
        ship.update()
        gf.update_bullets(bullets)
        gf.update_screen(ai_settings, screen, ship, alien, bullets)

        # 删除消失的子弹
        for bullet in bullets.copy():
            if bullet.rect.bottom < 0:
                bullets.remove(bullet)
        print(len(bullets))


run_game()
