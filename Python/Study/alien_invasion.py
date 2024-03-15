import pygame


from settings import Settings
from ship import Ship
from alien import Alien
from bullet import Bullet
import game_functions as gf
from pygame.sprite import Group


def create_fleet(ai_settings, screen, aliens):
    alien = Alien(ai_settings, screen)
    alien_width = alien.rect.width
    available_space_x = ai_settings.screen_width - 2 * alien_width
    number_aliens_x = int(available_space_x / (2 * alien_width))

    for alien_number in range(number_aliens_x):
        alien = Alien(ai_settings, screen)
        alien.x = alien_width + 2 * alien_width * alien_number
        alien.rect.x = alien.x
        aliens.add(alien)

def run_game():
    # 初始化游戏并创建屏幕对象
    pygame.init()
    ai_settings = Settings()
    screen = pygame.display.set_mode(
        (ai_settings.screen_width, ai_settings.screen_height))
    pygame.display.set_caption("Alien Invasion")

    # 创建飞船
    ship = Ship(ai_settings, screen)

    bullets = Group()
    aliens = Group()

    alien = Alien(ai_settings, screen)

    gf.create_fleet(ai_settings, screen, aliens)

    # 开始游戏主体
    while True:
        gf.check_events(ai_settings, screen, ship, bullets)
        ship.update()
        gf.update_bullets(bullets)
        gf.update_screen(ai_settings, screen, ship, aliens, bullets)

        # 删除消失的子弹
        for bullet in bullets.copy():
            if bullet.rect.bottom < 0:
                bullets.remove(bullet)
        print(len(bullets))


run_game()
