class Settings:
    def __init__(self):
        # 屏幕设置
        self.screen_width = 1920
        self.screen_height = 1080
        self.bg_color = (60, 60, 60)
        self.ship_speed_factor = 0.1

        self.bullet_speed_factor = 0.5
        self.bullet_width = 5
        self.bullet_height = 15
        self.bullet_color = 255, 255, 255
        self.bullets_allowed = 5
